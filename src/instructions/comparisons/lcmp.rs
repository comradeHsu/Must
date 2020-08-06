use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

///lcmp
pub struct Lcmp(NoOperandsInstruction);

impl Lcmp {
    #[inline]
    pub const fn new() -> Lcmp {
        return Lcmp(NoOperandsInstruction::new());
    }
}

impl Instruction for Lcmp {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        if v1 > v2 {
            stack.push_int(1);
        } else if v1 == v2 {
            stack.push_int(0);
        } else {
            stack.push_int(-1);
        }
    }
}
