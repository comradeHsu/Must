use crate::instructions::base::branch_logic::branch;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{BranchInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;

pub struct Goto(BranchInstruction);

impl Goto {
    #[inline]
    pub const fn new() -> Goto {
        return Goto(BranchInstruction::new());
    }
}

impl Instruction for Goto {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        branch(frame, self.0.get_offset());
    }
}
