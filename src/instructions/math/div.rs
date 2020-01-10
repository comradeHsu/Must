use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

///d_div
pub struct DDiv(NoOperandsInstruction);

impl Instruction for DDiv {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let rs = v1 / v2;
        stack.push_double(rs);
    }
}

///f_div
pub struct FDiv(NoOperandsInstruction);

impl Instruction for FDiv {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let rs = v1 / v2;
        stack.push_float(rs);
    }
}

///i_div
pub struct IDiv(NoOperandsInstruction);

impl Instruction for IDiv {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let rs = v1 / v2;
        stack.push_int(rs);
    }
}

///l_div
pub struct LDiv(NoOperandsInstruction);

impl Instruction for LDiv {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 / v2;
        stack.push_long(rs);
    }
}