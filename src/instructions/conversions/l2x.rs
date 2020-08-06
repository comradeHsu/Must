use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct L2d(NoOperandsInstruction);

impl L2d {
    #[inline]
    pub const fn new() -> L2d {
        return L2d(NoOperandsInstruction::new());
    }
}

impl Instruction for L2d {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_long();
        stack.push_double(v1 as f64);
    }
}

pub struct L2f(NoOperandsInstruction);

impl L2f {
    #[inline]
    pub const fn new() -> L2f {
        return L2f(NoOperandsInstruction::new());
    }
}

impl Instruction for L2f {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_long();
        stack.push_float(v1 as f32);
    }
}

pub struct L2i(NoOperandsInstruction);

impl L2i {
    #[inline]
    pub const fn new() -> L2i {
        return L2i(NoOperandsInstruction::new());
    }
}

impl Instruction for L2i {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_long();
        stack.push_int(v1 as i32);
    }
}
