use crate::runtime::frame::Frame;
use crate::runtime::stack::Stack;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::oops::object::Object;
use crate::jvm::Jvm;
use crate::oops::class::Class;
use crate::oops::object::MetaData::Thread;
use crate::invoke_support::parameter::{Parameters, Parameter};
use crate::oops::string_pool::StringPool;
use crate::invoke_support::JavaCall;
use crate::invoke_support::ReturnType::Void;
use std::fmt::{Debug, Formatter, Error};
use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;


struct Inner {
    pub pc: i32,
    pub stack: Stack,
    object: Option<Object>
    //    thread:Option<Builder>
}

#[derive(Clone)]
pub struct JavaThread {
    inner: Rc<RefCell<Inner>>,
}

thread_local! {static CURRENT_THREAD:RefCell<Option<JavaThread>> = RefCell::new(None) }

impl JavaThread {

    pub fn new_thread(java_thread: Option<Object>) -> JavaThread {
        return JavaThread {
            inner: Rc::new(RefCell::new(Inner {
                pc: 0,
                stack: Stack::new(1024),
                object: java_thread
            })),
        };
    }

    pub fn new_main_thread() -> JavaThread {
        let thread = JavaThread {
            inner: Rc::new(RefCell::new(Inner {
                pc: 0,
                stack: Stack::new(1024),
                object: None
            })),
        };
        thread
    }

    pub fn get_pc(&self) -> i32 {
        return (*self.inner).borrow().pc;
    }

    pub fn set_pc(&self, pc: i32) {
        (*self.inner).borrow_mut().pc = pc;
    }

    pub fn push_frame(&self, frame: Frame) {
        (*self.inner).borrow_mut().stack.push(frame);
    }

    pub fn pop_frame(&self) -> Frame {
        return (*self.inner).borrow_mut().stack.pop();
    }

    pub fn current_frame(&self) -> Frame {
        return (*self.inner).borrow().stack.top();
    }

    #[inline]
    pub fn is_stack_empty(&self) -> bool {
        return (*self.inner).borrow().stack.is_empty();
    }

    #[inline]
    pub fn stack_size(&self) -> usize {
        return (*self.inner).borrow().stack.size();
    }

    #[inline]
    pub fn clear_stack(&self) {
        (*self.inner).borrow_mut().stack.clear();
    }

    pub fn java_thread(&self) -> Option<Object> {
        return (*self.inner).borrow().object.clone();
    }

    /// just for create main thread call
    pub fn set_java_thread(&self, obj: Option<Object>) {
        (*self.inner).borrow_mut().object = obj;
    }

    pub fn frames_with<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&VecDeque<Frame>) -> R,
    {
        let borrow = (*self.inner).borrow();
        func(borrow.stack.get_frames())
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
