use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

///i_and
pub struct IAnd(NoOperandsInstruction);

impl IAnd {
    #[inline]
    pub const fn new() -> IAnd {
        return IAnd(NoOperandsInstruction::new());
    }
}

impl Instruction for IAnd {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let rs = v1 & v2;
        stack.push_int(rs);
    }
}
///l_and
pub struct LAnd(NoOperandsInstruction);

impl LAnd {
    #[inline]
    pub const fn new() -> LAnd {
        return LAnd(NoOperandsInstruction::new());
    }
}

impl Instruction for LAnd {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 & v2;
        stack.push_long(rs);
    }
}
