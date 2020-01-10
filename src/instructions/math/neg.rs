use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

///d_neg
pub struct DNeg(NoOperandsInstruction);

impl Instruction for DNeg {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_double();
        stack.push_double(-v1);
    }
}

///f_neg
pub struct FNeg(NoOperandsInstruction);

impl Instruction for FNeg {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_float();
        stack.push_float(-v1);
    }
}

///i_neg
pub struct INeg(NoOperandsInstruction);

impl Instruction for INeg {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_int();
        stack.push_int(-v1);
    }
}

///l_neg
pub struct LNeg(NoOperandsInstruction);

impl Instruction for LNeg {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_long();
        stack.push_long(-v1);
    }
}