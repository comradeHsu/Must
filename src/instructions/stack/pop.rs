use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime_data_area::frame::Frame;

///pop
pub struct Pop(NoOperandsInstruction);

impl Pop {
    #[inline]
    pub const fn new() -> Pop {
        return Pop(NoOperandsInstruction::new());
    }
}

impl Instruction for Pop {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        stack.pop_slot();
    }
}

///pop_2
pub struct Pop2(NoOperandsInstruction);

impl Pop2 {
    #[inline]
    pub const fn new() -> Pop2 {
        return Pop2(NoOperandsInstruction::new());
    }
}

impl Instruction for Pop2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        stack.pop_slot();
        stack.pop_slot();
    }
}
