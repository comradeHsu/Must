use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

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

    fn execute(&mut self, frame: &mut Frame) {
        let thread = frame.thread();
        (*thread).borrow_mut().pop_frame();
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

    fn execute(&mut self, frame: &mut Frame) {
        let thread = frame.thread();
        let mut current_frame = (*thread).borrow_mut().pop_frame();
        let mut borrow = (*thread).borrow_mut();
        let invoke_frame = borrow.current_frame();
//        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.operand_stack().expect("stack is none")
            .pop_ref();
        let mut borrow_invoke = (*invoke_frame).borrow_mut();
        borrow_invoke.operand_stack().expect("stack is none").push_ref(return_value.clone());
        println!("return value is none:{}",return_value.is_none());
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

    fn execute(&mut self, frame: &mut Frame) {
        let thread = frame.thread();
        let mut current_frame = (*thread).borrow_mut().pop_frame();
        let mut borrow = (*thread).borrow_mut();
        let invoke_frame = borrow.current_frame();
//        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.operand_stack().expect("stack is none")
            .pop_double();
        let mut borrow_invoke = (*invoke_frame).borrow_mut();
        borrow_invoke.operand_stack().expect("stack is none").push_double(return_value);
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

    fn execute(&mut self, frame: &mut Frame) {
        let thread = frame.thread();
        let mut current_frame = (*thread).borrow_mut().pop_frame();
        let mut borrow = (*thread).borrow_mut();
        let invoke_frame = borrow.current_frame();
//        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.operand_stack().expect("stack is none")
            .pop_float();
        let mut borrow_invoke = (*invoke_frame).borrow_mut();
        borrow_invoke.operand_stack().expect("stack is none").push_float(return_value);
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

    fn execute(&mut self, frame: &mut Frame) {
        let thread = frame.thread();
        let mut current_frame = (*thread).borrow_mut().pop_frame();
        let mut borrow = (*thread).borrow_mut();
        let invoke_frame = borrow.current_frame();
//        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.operand_stack().expect("stack is none")
            .pop_int();
        let mut borrow_invoke = (*invoke_frame).borrow_mut();
        borrow_invoke.operand_stack().expect("stack is none").push_int(return_value);
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

    fn execute(&mut self, frame: &mut Frame) {
        let thread = frame.thread();
        let mut current_frame = (*thread).borrow_mut().pop_frame();
        let mut borrow = (*thread).borrow_mut();
        let invoke_frame = borrow.current_frame();
//        let mut borrow_frame = (*current_frame).borrow_mut();
        let return_value = frame.operand_stack().expect("stack is none")
            .pop_long();
        let mut borrow_invoke = (*invoke_frame).borrow_mut();
        borrow_invoke.operand_stack().expect("stack is none").push_long(return_value);
    }
}