use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct D2f(NoOperandsInstruction);

impl D2f {
    #[inline]
    pub const fn new() -> D2f {
        return D2f(NoOperandsInstruction::new());
    }
}

impl Instruction for D2f {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack|{
            let v1 = stack.pop_double();
            stack.push_float(v1 as f32);
        })
    }
}

pub struct D2i(NoOperandsInstruction);

impl D2i {
    #[inline]
    pub const fn new() -> D2i {
        return D2i(NoOperandsInstruction::new());
    }
}

impl Instruction for D2i {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack|{
            let v1 = stack.pop_double();
            stack.push_int(v1 as i32);
        })
    }
}

pub struct D2l(NoOperandsInstruction);

impl D2l {
    #[inline]
    pub const fn new() -> D2l {
        return D2l(NoOperandsInstruction::new());
    }
}

impl Instruction for D2l {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack|{
            let v1 = stack.pop_double();
            stack.push_long(v1 as i64);
        })
    }
}
