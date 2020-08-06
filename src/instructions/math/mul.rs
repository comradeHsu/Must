use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct DMul(NoOperandsInstruction);

impl DMul {
    #[inline]
    pub const fn new() -> DMul {
        return DMul(NoOperandsInstruction::new());
    }
}

impl Instruction for DMul {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let rs = v1 * v2;
        stack.push_double(rs);
    }
}

pub struct FMul(NoOperandsInstruction);

impl FMul {
    #[inline]
    pub const fn new() -> FMul {
        return FMul(NoOperandsInstruction::new());
    }
}

impl Instruction for FMul {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let rs = v1 * v2;
        stack.push_float(rs);
    }
}

pub struct IMul(NoOperandsInstruction);

impl IMul {
    #[inline]
    pub const fn new() -> IMul {
        return IMul(NoOperandsInstruction::new());
    }
}

impl Instruction for IMul {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int() as i64;
        let v1 = stack.pop_int() as i64;
        let rs = v1 * v2;
        stack.push_int(rs as i32);
    }
}

pub struct LMul(NoOperandsInstruction);

impl LMul {
    #[inline]
    pub const fn new() -> LMul {
        return LMul(NoOperandsInstruction::new());
    }
}

impl Instruction for LMul {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 * v2;
        stack.push_long(rs);
    }
}
