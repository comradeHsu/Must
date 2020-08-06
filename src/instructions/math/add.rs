use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct DAdd(NoOperandsInstruction);

impl DAdd {
    #[inline]
    pub const fn new() -> DAdd {
        return DAdd(NoOperandsInstruction::new());
    }
}

impl Instruction for DAdd {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let rs = v1 + v2;
        stack.push_double(rs);
    }
}

pub struct FAdd(NoOperandsInstruction);

impl FAdd {
    #[inline]
    pub const fn new() -> FAdd {
        return FAdd(NoOperandsInstruction::new());
    }
}

impl Instruction for FAdd {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let rs = v1 + v2;
        stack.push_float(rs);
    }
}

pub struct IAdd(NoOperandsInstruction);

impl IAdd {
    #[inline]
    pub const fn new() -> IAdd {
        return IAdd(NoOperandsInstruction::new());
    }
}

impl Instruction for IAdd {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let (rs, _) = v1.overflowing_add(v2);
        stack.push_int(rs);
    }
}

pub struct LAdd(NoOperandsInstruction);

impl LAdd {
    #[inline]
    pub const fn new() -> LAdd {
        return LAdd(NoOperandsInstruction::new());
    }
}

impl Instruction for LAdd {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 + v2;
        stack.push_long(rs);
    }
}
