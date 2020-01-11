use crate::runtime_data_area::stack::Stack;
use crate::runtime_data_area::frame::Frame;

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

    pub fn current_frame(&mut self) -> &Frame {
        return self.stack.top();
    }

    pub fn new_frame(&self,max_locals:usize,max_stack:usize) -> Frame {
        return Frame::with_capacity(self,max_stack,max_stack);
    }
}