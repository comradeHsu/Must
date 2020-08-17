use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;
use crate::runtime::thread::JavaThread;

pub struct Return(NoOperandsInstruction);

impl Return {
    #[inline]
    pub fn new() -> Return {
        return Return(NoOperandsInstruction::new());
    }
}

impl Instruction for Return {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, _frame: &Frame) {
        let thread = JavaThread::current();
        thread.pop_frame();
    }
}

pub struct AReturn(NoOperandsInstruction);

impl AReturn {
    #[inline]
    pub fn new() -> AReturn {
        return AReturn(NoOperandsInstruction::new());
    }
}

impl Instruction for AReturn {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let thread = JavaThread::current();
        let _current_frame = thread.pop_frame();
        let invoke_frame = thread.current_frame();
        //        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.pop_ref();
        invoke_frame.push_ref(return_value);
    }
}

pub struct DReturn(NoOperandsInstruction);

impl DReturn {
    #[inline]
    pub fn new() -> DReturn {
        return DReturn(NoOperandsInstruction::new());
    }
}

impl Instruction for DReturn {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let thread = JavaThread::current();
        let _current_frame = thread.pop_frame();
        let invoke_frame = thread.current_frame();
        //        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.pop_double();
        invoke_frame.push_double(return_value);
    }
}

pub struct FReturn(NoOperandsInstruction);

impl FReturn {
    #[inline]
    pub fn new() -> FReturn {
        return FReturn(NoOperandsInstruction::new());
    }
}

impl Instruction for FReturn {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let thread = JavaThread::current();
        let _current_frame = thread.pop_frame();
        let invoke_frame = thread.current_frame();
        //        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.pop_float();
        invoke_frame.push_float(return_value);
    }
}

pub struct IReturn(NoOperandsInstruction);

impl IReturn {
    #[inline]
    pub fn new() -> IReturn {
        return IReturn(NoOperandsInstruction::new());
    }
}

impl Instruction for IReturn {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let thread = JavaThread::current();
        let _current_frame = thread.pop_frame();
        let invoke_frame = thread.current_frame();
        //        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.pop_int();
        invoke_frame.push_int(return_value);
    }
}

pub struct LReturn(NoOperandsInstruction);

impl LReturn {
    #[inline]
    pub fn new() -> LReturn {
        return LReturn(NoOperandsInstruction::new());
    }
}

impl Instruction for LReturn {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let thread = JavaThread::current();
        let _current_frame = thread.pop_frame();
        let invoke_frame = thread.current_frame();
        //        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.pop_long();
        invoke_frame.push_long(return_value);
    }
}
