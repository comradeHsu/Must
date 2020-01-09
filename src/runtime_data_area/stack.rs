use std::collections::VecDeque;
use crate::runtime_data_area::frame::Frame;

pub struct Stack {
    max_size:usize,
    size:usize,
    frames:VecDeque<Frame>
}

impl Stack {
    #[inline]
    pub fn new(max_size:usize) -> Stack {
        return Stack{
            max_size,
            size:0,
            frames: Default::default()
        };
    }

    pub fn push(&mut self,frame:Frame) {
        if self.frames.len() >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }
        self.frames.push_back(frame);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Frame {
        if self.frames.is_empty() {
            panic!("jvm stack is empty!");
        }
        let frame = self.frames.pop_back().unwrap();
        self.size -= 1;
        return frame;
    }

    pub fn top(&mut self) -> &Frame {
        if self.frames.is_empty() {
            panic!("jvm stack is empty!");
        }
        let frame = self.frames.get(self.size).unwrap();
        return frame;
    }
}