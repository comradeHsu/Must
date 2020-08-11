use crate::runtime::frame::Frame;
use crate::oops::method::Method;
use crate::runtime::stack::Stack;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::thread;
use std::thread::{Builder, Thread};
use std::borrow::Borrow;

struct Inner {
    pub pc: i32,
    pub stack: Stack,
    //    thread:Option<Builder>
}

#[derive(Clone)]
pub struct JavaThread {
    inner:Rc<RefCell<Inner>>
}

thread_local!{static CURRENT_THREAD:RefCell<Option<JavaThread>> = RefCell::new(None) }

impl JavaThread {
    pub fn new_thread() -> JavaThread {
        return JavaThread {
            inner: Rc::new(RefCell::new(
                Inner {
                    pc: 0,
                    stack: Stack::new(1024),
                }
            )),
        };
    }

    pub fn new_main_thread() -> JavaThread {
        return JavaThread {
            inner: Rc::new(RefCell::new(
                Inner {
                    pc: 0,
                    stack: Stack::new(1024),
                }
            )),
        };
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

    pub fn pop_frame(&self) -> Rc<RefCell<Frame>> {
        return (*self.inner).borrow_mut().stack.pop();
    }

    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        return (*self.inner).borrow().stack.top();
    }

    //    pub fn current_frame_mut(&mut self) -> &mut Frame {
    //        return self.stack.top_mut();
    //    }

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

    pub fn frames_with<R, F>(&self,func: F) -> R
        where
            F: FnOnce(&VecDeque<Rc<RefCell<Frame>>>) -> R,
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
            .try_with(move |c| {
                *c.borrow_mut() = Some(self.clone())
            })
            .ok();
    }
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
