use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct Nop(NoOperandsInstruction);

impl Nop {
    #[inline]
    pub const fn new() -> Nop {
        return Nop(NoOperandsInstruction::new());
    }
}

impl Instruction for Nop {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {}
}
