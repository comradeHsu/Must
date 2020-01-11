use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

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

    fn execute(&mut self, frame: &mut Frame) {
    }
}