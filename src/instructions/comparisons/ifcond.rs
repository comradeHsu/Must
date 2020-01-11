use crate::instructions::base::instruction::{BranchInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::branch_logic::branch;

pub struct IfEq(BranchInstruction);

impl IfEq {
    #[inline]
    pub const fn new() -> IfEq {
        return IfEq(BranchInstruction::new());
    }
}

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

impl IfNe {
    #[inline]
    pub const fn new() -> IfNe {
        return IfNe(BranchInstruction::new());
    }
}

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

impl IfLt {
    #[inline]
    pub const fn new() -> IfLt {
        return IfLt(BranchInstruction::new());
    }
}

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

impl IfLe {
    #[inline]
    pub const fn new() -> IfLe {
        return IfLe(BranchInstruction::new());
    }
}

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

impl IfGt {
    #[inline]
    pub const fn new() -> IfGt {
        return IfGt(BranchInstruction::new());
    }
}

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

impl IfGe {
    #[inline]
    pub const fn new() -> IfGe {
        return IfGe(BranchInstruction::new());
    }
}

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