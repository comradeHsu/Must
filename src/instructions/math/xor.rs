use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

///i_xor
pub struct IXor(NoOperandsInstruction);

impl IXor {
    #[inline]
    pub const fn new() -> IXor {
        return IXor(NoOperandsInstruction::new());
    }
}

impl Instruction for IXor {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let rs = v1 ^ v2;
        stack.push_int(rs);
    }
}

///l_xor
pub struct LXor(NoOperandsInstruction);

impl LXor {
    #[inline]
    pub const fn new() -> LXor {
        return LXor(NoOperandsInstruction::new());
    }
}

impl Instruction for LXor {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 ^ v2;
        stack.push_long(rs);
    }
}