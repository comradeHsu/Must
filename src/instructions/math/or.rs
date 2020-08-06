use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

///i_or
pub struct IOr(NoOperandsInstruction);

impl IOr {
    #[inline]
    pub const fn new() -> IOr {
        return IOr(NoOperandsInstruction::new());
    }
}

impl Instruction for IOr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let rs = v1 | v2;
        stack.push_int(rs);
    }
}

///l_or
pub struct LOr(NoOperandsInstruction);

impl LOr {
    #[inline]
    pub const fn new() -> LOr {
        return LOr(NoOperandsInstruction::new());
    }
}

impl Instruction for LOr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 | v2;
        stack.push_long(rs);
    }
}
