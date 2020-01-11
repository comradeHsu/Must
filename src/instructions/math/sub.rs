use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::frame::Frame;

///d_div
pub struct DSub(NoOperandsInstruction);

impl DSub {
    #[inline]
    pub const fn new() -> DSub {
        return DSub(NoOperandsInstruction::new());
    }
}

impl Instruction for DSub {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let rs = v1 - v2;
        stack.push_double(rs);
    }
}

///f_div
pub struct FSub(NoOperandsInstruction);

impl FSub {
    #[inline]
    pub const fn new() -> FSub {
        return FSub(NoOperandsInstruction::new());
    }
}

impl Instruction for FSub {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let rs = v1 - v2;
        stack.push_float(rs);
    }
}

///i_div
pub struct ISub(NoOperandsInstruction);

impl ISub {
    #[inline]
    pub const fn new() -> ISub {
        return ISub(NoOperandsInstruction::new());
    }
}

impl Instruction for ISub {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let rs = v1 - v2;
        stack.push_int(rs);
    }
}

///l_div
pub struct LSub(NoOperandsInstruction);

impl LSub {
    #[inline]
    pub const fn new() -> LSub {
        return LSub(NoOperandsInstruction::new());
    }
}

impl Instruction for LSub {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 - v2;
        stack.push_long(rs);
    }
}