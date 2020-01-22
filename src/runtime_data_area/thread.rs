use crate::runtime_data_area::stack::Stack;
use crate::runtime_data_area::frame::Frame;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::method::Method;

pub struct Thread {
    pc:i32,
    stack:Stack
}

impl Thread {
    pub fn new_thread() -> Thread {
        return Thread{ pc: 0, stack: Stack::new(1024) };
    }

    pub fn get_pc(&self) -> i32 {
        return self.pc;
    }

    pub fn set_pc(&mut self,pc:i32) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self,frame:Frame) {
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Frame {
        return self.stack.pop();
    }

    pub fn current_frame(& self) -> &Frame {
        return self.stack.top();
    }

    pub fn current_frame_mut(&mut self) -> &mut Frame {
        return self.stack.top_mut();
    }

    pub fn new_frame(thread:Rc<RefCell<Thread>>,method:Rc<Method>) -> Frame {
        return Frame::new(thread,method);
    }

    #[inline]
    pub fn is_stack_empty(&self) -> bool {
        return self.stack.is_empty();
    }

}