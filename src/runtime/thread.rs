use crate::runtime::frame::Frame;
use crate::oops::method::Method;
use crate::runtime::stack::Stack;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::thread;
use std::thread::{Builder, Thread};

pub struct JavaThread {
    pc: i32,
    stack: Stack,
    //    thread:Option<Builder>
}

impl JavaThread {
    pub fn new_thread() -> JavaThread {
        return JavaThread {
            pc: 0,
            stack: Stack::new(1024),
            //            thread: Some(thread::Builder::new())
        };
    }

    pub fn new_main_thread() -> JavaThread {
        return JavaThread {
            pc: 0,
            stack: Stack::new(1024),
            //            thread: Some(thread::Builder::new().name("Main".to_string()))
        };
    }

    pub fn get_pc(&self) -> i32 {
        return self.pc;
    }

    pub fn set_pc(&mut self, pc: i32) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Rc<RefCell<Frame>> {
        return self.stack.pop();
    }

    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        return self.stack.top();
    }

    //    pub fn current_frame_mut(&mut self) -> &mut Frame {
    //        return self.stack.top_mut();
    //    }

    pub fn new_frame(thread: Rc<RefCell<JavaThread>>, method: Rc<Method>) -> Frame {
        return Frame::new(thread, method);
    }

    #[inline]
    pub fn is_stack_empty(&self) -> bool {
        return self.stack.is_empty();
    }

    #[inline]
    pub fn stack_size(&self) -> usize {
        return self.stack.size();
    }

    #[inline]
    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }

    #[inline]
    pub fn get_frames(&self) -> &VecDeque<Rc<RefCell<Frame>>> {
        return self.stack.get_frames();
    }

    //    #[inline]
    //    pub fn std_thread(&mut self) -> Builder {
    //        return self.thread.take().unwrap();
    //    }
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
            println!("child thread");
        });
        println!("main thread");
        handle.join().expect("thread::spawn failed");
    }
}
