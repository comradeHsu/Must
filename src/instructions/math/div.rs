use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

///d_div
pub struct DDiv(NoOperandsInstruction);

impl DDiv {
    #[inline]
    pub const fn new() -> DDiv {
        return DDiv(NoOperandsInstruction::new());
    }
}

impl Instruction for DDiv {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_double();
            let v1 = stack.pop_double();
            let rs = v1 / v2;
            stack.push_double(rs);
        })
    }
}

///f_div
pub struct FDiv(NoOperandsInstruction);

impl FDiv {
    #[inline]
    pub const fn new() -> FDiv {
        return FDiv(NoOperandsInstruction::new());
    }
}

impl Instruction for FDiv {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_float();
            let v1 = stack.pop_float();
            let rs = v1 / v2;
            stack.push_float(rs);
        })
    }
}

///i_div
pub struct IDiv(NoOperandsInstruction);

impl IDiv {
    #[inline]
    pub const fn new() -> IDiv {
        return IDiv(NoOperandsInstruction::new());
    }
}

impl Instruction for IDiv {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_int();
            let v1 = stack.pop_int();
            let rs = v1 / v2;
            stack.push_int(rs);
        })
    }
}

///l_div
pub struct LDiv(NoOperandsInstruction);

impl LDiv {
    #[inline]
    pub const fn new() -> LDiv {
        return LDiv(NoOperandsInstruction::new());
    }
}

impl Instruction for LDiv {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_long();
            let v1 = stack.pop_long();
            let rs = v1 / v2;
            stack.push_long(rs);
        })
    }
}
