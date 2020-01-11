use crate::instructions::base::instruction::{BranchInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::branch_logic::branch;

pub struct IfNull(BranchInstruction);

impl IfNull {
    #[inline]
    pub const fn new() -> IfNull {
        return IfNull(BranchInstruction::new());
    }
}

impl Instruction for IfNull {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let reference = frame.operand_stack()
            .expect("operand_stack is none")
            .pop_ref();
        if reference.is_none() {
            branch(frame,self.0.get_offset());
        }
    }
}

pub struct IfNonNull(BranchInstruction);

impl IfNonNull {
    #[inline]
    pub const fn new() -> IfNonNull {
        return IfNonNull(BranchInstruction::new());
    }
}

impl Instruction for IfNonNull {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let reference = frame.operand_stack()
            .expect("operand_stack is none")
            .pop_ref();
        if reference.is_some() {
            branch(frame,self.0.get_offset());
        }
    }
}