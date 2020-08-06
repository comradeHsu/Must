use crate::runtime::frame::Frame;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Stack {
    max_size: usize,
    size: usize,
    frames: VecDeque<Rc<RefCell<Frame>>>,
}

impl Stack {
    #[inline]
    pub fn new(max_size: usize) -> Stack {
        return Stack {
            max_size,
            size: 0,
            frames: Default::default(),
        };
    }

    pub fn push(&mut self, frame: Frame) {
        if self.frames.len() >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }
        self.frames.push_back(Frame::boxed(frame));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Rc<RefCell<Frame>> {
        if self.frames.is_empty() {
            panic!("jvm stack is empty!");
        }
        let frame = self.frames.pop_back().unwrap();
        self.size -= 1;
        return frame;
    }

    pub fn top(&self) -> Rc<RefCell<Frame>> {
        if self.frames.is_empty() {
            panic!("jvm stack is empty!");
        }
        let frame = self.frames.get(self.size - 1).unwrap();
        return frame.clone();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    #[inline]
    pub fn size(&self) -> usize {
        return self.size;
    }

    #[inline]
    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.pop();
        }
    }

    #[inline]
    pub fn get_frames(&self) -> &VecDeque<Rc<RefCell<Frame>>> {
        return &self.frames;
    }
}
