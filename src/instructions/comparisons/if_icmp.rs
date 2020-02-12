use crate::instructions::base::branch_logic::branch;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{BranchInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;

fn int_pop(frame: &mut Frame) -> (i32, i32) {
    let stack = frame.operand_stack().expect("operand_stack is none");
    let v2 = stack.pop_int();
    let v1 = stack.pop_int();
    return (v1, v2);
}

pub struct IfICmpEq(BranchInstruction);

impl IfICmpEq {
    #[inline]
    pub const fn new() -> IfICmpEq {
        return IfICmpEq(BranchInstruction::new());
    }
}

impl Instruction for IfICmpEq {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (v1, v2) = int_pop(frame);
        if v1 == v2 {
            branch(frame, self.0.get_offset());
        }
    }
}

pub struct IfICmpNe(BranchInstruction);

impl IfICmpNe {
    #[inline]
    pub const fn new() -> IfICmpNe {
        return IfICmpNe(BranchInstruction::new());
    }
}

impl Instruction for IfICmpNe {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (v1, v2) = int_pop(frame);
        if v1 != v2 {
            branch(frame, self.0.get_offset());
        }
    }
}

pub struct IfICmpLt(BranchInstruction);

impl IfICmpLt {
    #[inline]
    pub const fn new() -> IfICmpLt {
        return IfICmpLt(BranchInstruction::new());
    }
}

impl Instruction for IfICmpLt {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (v1, v2) = int_pop(frame);
        if v1 < v2 {
            branch(frame, self.0.get_offset());
        }
    }
}

pub struct IfICmpLe(BranchInstruction);

impl IfICmpLe {
    #[inline]
    pub const fn new() -> IfICmpLe {
        return IfICmpLe(BranchInstruction::new());
    }
}

impl Instruction for IfICmpLe {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (v1, v2) = int_pop(frame);
        if v1 <= v2 {
            branch(frame, self.0.get_offset());
        }
    }
}

pub struct IfICmpGt(BranchInstruction);

impl IfICmpGt {
    #[inline]
    pub const fn new() -> IfICmpGt {
        return IfICmpGt(BranchInstruction::new());
    }
}

impl Instruction for IfICmpGt {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (v1, v2) = int_pop(frame);
        if v1 > v2 {
            branch(frame, self.0.get_offset());
        }
    }
}

pub struct IfICmpGe(BranchInstruction);

impl IfICmpGe {
    #[inline]
    pub const fn new() -> IfICmpGe {
        return IfICmpGe(BranchInstruction::new());
    }
}

impl Instruction for IfICmpGe {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let (v1, v2) = int_pop(frame);
        if v1 >= v2 {
            branch(frame, self.0.get_offset());
        }
    }
}
