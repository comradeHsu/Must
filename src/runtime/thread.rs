use crate::runtime::frame::Frame;
use crate::runtime::stack::Stack;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::oops::object::Object;
use crate::jvm::Jvm;
use crate::oops::class::Class;
use crate::oops::object::MetaData::Thread;
use crate::invoke_support::parameter::{Parameters, Parameter};
use crate::oops::string_pool::StringPool;
use crate::invoke_support::{JavaCall, ReturnType};
use crate::invoke_support::ReturnType::Void;
use std::fmt::{Debug, Formatter, Error};
use crate::utils::java_classes::JavaLangThread;
use std::thread;
use std::sync::{Arc, Mutex};


struct Inner {
    pub pc: i32,
    pub stack: Stack,
    object: Option<Object>
    //    thread:Option<Builder>
}

#[derive(Clone)]
pub struct JavaThread {
    inner: Arc<Mutex<Inner>>,
}

thread_local! {static CURRENT_THREAD:RefCell<Option<JavaThread>> = RefCell::new(None) }

impl JavaThread {

    pub fn new_thread(java_thread: Object) -> JavaThread {
        let mut size = JavaLangThread::stack_size(&java_thread) as usize;
        if  size == 0 {
            size = 1024;
        }
        return JavaThread {
            inner: Arc::new(Mutex::new(Inner {
                pc: 0,
                stack: Stack::new(size),
                object: Some(java_thread)
            })),
        };
    }

    pub fn new_main_thread() -> JavaThread {
        let thread = JavaThread {
            inner: Arc::new(Mutex::new(Inner {
                pc: 0,
                stack: Stack::new(1024),
                object: None
            })),
        };
        thread
    }

    pub fn get_pc(&self) -> i32 {
        let inner = self.inner.lock().unwrap();
        return inner.pc
    }

    pub fn set_pc(&self, pc: i32) {
        let mut inner = self.inner.lock().unwrap();
        inner.pc = pc;
    }

    pub fn push_frame(&self, frame: Frame) {
        let mut inner = self.inner.lock().unwrap();
        inner.stack.push(frame);
    }

    pub fn pop_frame(&self) -> Frame {
        let mut inner = self.inner.lock().unwrap();
        return inner.stack.pop()
    }

    pub fn current_frame(&self) -> Frame {
        let mut inner = self.inner.lock().unwrap();
        return inner.stack.top();
    }

    #[inline]
    pub fn is_stack_empty(&self) -> bool {
        let mut inner = self.inner.lock().unwrap();
        return inner.stack.is_empty();
    }

    #[inline]
    pub fn clear_stack(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.stack.clear()
    }

    pub fn java_thread(&self) -> Option<Object> {
        let inner = self.inner.lock().unwrap();
        return inner.object.clone();
    }

    /// just for create main thread call
    pub fn set_java_thread(&self, obj: Option<Object>) {
        let mut inner = self.inner.lock().unwrap();
        inner.object = obj
    }

    pub fn frames_with<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&VecDeque<Frame>) -> R,
    {
        let mut inner = self.inner.lock().unwrap();
        func(inner.stack.get_frames())
    }

    pub fn current() -> JavaThread {
        CURRENT_THREAD
            .try_with(move |c| {
                if c.borrow().is_none() {
                    panic!("The current thread is none")
                }
                c.borrow().as_ref().unwrap().clone()
            })
            .ok()
            .unwrap()
    }

    pub fn set(&self) {
        CURRENT_THREAD
            .try_with(move |c| *c.borrow_mut() = Some(self.clone()))
            .ok();
    }

    fn run(&self) {
        let thread_obj = self.java_thread();
        let class = thread_obj.as_ref().unwrap().class();
        let run_method = class.get_instance_method("run","()V");
        let parameters = vec![
            Parameter::Object(thread_obj),
        ];
        JavaCall::invoke(
            run_method.unwrap(),
            Some(Parameters::with_parameters(parameters)),
            ReturnType::Void,
        );
    }

    pub fn start(thread: Self) {
        thread::Builder::new().spawn(move || {
            thread.set();
            thread.run();
        });
    }
}

impl Debug for JavaThread {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}

pub mod thread_priority {        // JLS 20.20.1-3
    pub const NO_PRIORITY:i32       = -1;     // Initial non-priority value
    pub const MIN_PRIORITY:i32      =  1;     // Minimum priority
    pub const NORM_PRIORITY:i32    =  5;     // Normal (non-daemon) priority
    pub const NEAR_MAX_PRIORITY:i32  =  9;     // High priority, used for VMThread
    pub const MAX_PRIORITY:i32      = 10;     // Highest priority, used for WatcherThread
                                            // ensures that VMThread doesn't starve profiler
    pub const CRITICAL_PRIORITY:i32 = 11;     // Critical thread priority
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_mutex() {
        fn sum(until: i32, num: Arc<Mutex<i32>>) -> i32 {
            let clone = num.clone();
            //            {
            let mut lock = clone.lock().unwrap();
            if until == 0 {
                return *lock;
            }
            *lock += until;
            //            }
            return sum(until - 1, num);
        }
        println!("sum:{}", sum(100, Arc::new(Mutex::new(0))));
    }

    #[test]
    fn test_thread() {
        let handle = thread::spawn(move || {
            thread::current();
            println!("child thread");
        });
        println!("main thread");
        handle.join().expect("thread::spawn failed");
    }
}
