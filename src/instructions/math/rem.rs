use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

///drem
pub struct DRem(NoOperandsInstruction);

impl DRem {
    #[inline]
    pub const fn new() -> DRem {
        return DRem(NoOperandsInstruction::new());
    }
}

impl Instruction for DRem {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let rs = v1.rem_euclid(v2);
        stack.push_double(rs);
    }
}

///frem
pub struct FRem(NoOperandsInstruction);

impl FRem {
    #[inline]
    pub const fn new() -> FRem {
        return FRem(NoOperandsInstruction::new());
    }
}

impl Instruction for FRem {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let rs = v1.rem_euclid(v2);
        stack.push_float(rs);
    }
}

///irem
pub struct IRem(NoOperandsInstruction);

impl IRem {
    #[inline]
    pub const fn new() -> IRem {
        return IRem(NoOperandsInstruction::new());
    }
}

impl Instruction for IRem {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let rs = v1.rem_euclid(v2);
        stack.push_int(rs);
    }
}

///lrem
pub struct LRem(NoOperandsInstruction);

impl LRem {
    #[inline]
    pub const fn new() -> LRem {
        return LRem(NoOperandsInstruction::new());
    }
}

impl Instruction for LRem {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1.rem_euclid(v2);
        stack.push_long(rs);
    }
}
