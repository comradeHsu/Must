use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct F2d(NoOperandsInstruction);

impl F2d {
    #[inline]
    pub const fn new() -> F2d {
        return F2d(NoOperandsInstruction::new());
    }
}

impl Instruction for F2d {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v1 = stack.pop_float();
            stack.push_double(v1 as f64);
        })
    }
}

pub struct F2i(NoOperandsInstruction);

impl F2i {
    #[inline]
    pub const fn new() -> F2i {
        return F2i(NoOperandsInstruction::new());
    }
}

impl Instruction for F2i {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v1 = stack.pop_float();
            stack.push_int(v1 as i32);
        })
    }
}

pub struct F2l(NoOperandsInstruction);

impl F2l {
    #[inline]
    pub const fn new() -> F2l {
        return F2l(NoOperandsInstruction::new());
    }
}

impl Instruction for F2l {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v1 = stack.pop_float();
            stack.push_long(v1 as i64);
        })
    }
}
