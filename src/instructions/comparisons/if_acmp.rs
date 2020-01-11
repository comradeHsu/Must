use crate::instructions::base::instruction::{BranchInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::branch_logic::branch;

fn acmp(frame: &mut Frame) -> bool {
    let stack = frame.operand_stack().expect("operand_stack is none");
    let v2 = stack.pop_ref();
    let v1 = stack.pop_ref();
    return v1 == v2;
}

pub struct IfACmpEq(BranchInstruction);

impl IfACmpEq {
    #[inline]
    pub const fn new() -> IfACmpEq {
        return IfACmpEq(BranchInstruction::new());
    }
}

impl Instruction for IfACmpEq {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        if acmp(frame) {
            branch(frame,self.0.get_offset());
        }
    }
}

pub struct IfACmpNe(BranchInstruction);

impl IfACmpNe {
    #[inline]
    pub const fn new() -> IfACmpNe {
        return IfACmpNe(BranchInstruction::new());
    }
}

impl Instruction for IfACmpNe {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        if !acmp(frame) {
            branch(frame,self.0.get_offset());
        }
    }
}