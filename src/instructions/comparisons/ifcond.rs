use crate::instructions::base::instruction::{BranchInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::branch_logic::branch;

pub struct IfEq(BranchInstruction);

impl Instruction for IfEq {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().expect("operand_stack is none")
            .pop_int();
        if val == 0 {
            branch(frame,self.0.get_offset());
        }
    }
}

pub struct IfNe(BranchInstruction);

impl Instruction for IfNe {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().expect("operand_stack is none")
            .pop_int();
        if val != 0 {
            branch(frame,self.0.get_offset());
        }
    }
}

pub struct IfLt(BranchInstruction);

impl Instruction for IfLt {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().expect("operand_stack is none")
            .pop_int();
        if val < 0 {
            branch(frame,self.0.get_offset());
        }
    }
}

pub struct IfLe(BranchInstruction);

impl Instruction for IfLe {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().expect("operand_stack is none")
            .pop_int();
        if val <= 0 {
            branch(frame,self.0.get_offset());
        }
    }
}

pub struct IfGt(BranchInstruction);

impl Instruction for IfGt {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().expect("operand_stack is none")
            .pop_int();
        if val > 0 {
            branch(frame,self.0.get_offset());
        }
    }
}

pub struct IfGe(BranchInstruction);

impl Instruction for IfGe {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().expect("operand_stack is none")
            .pop_int();
        if val >= 0 {
            branch(frame,self.0.get_offset());
        }
    }
}