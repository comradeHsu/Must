use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::frame::Frame;

pub struct I2d(NoOperandsInstruction);

impl Instruction for I2d {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_int();
        stack.push_double(v1 as f64);
    }
}

pub struct I2f(NoOperandsInstruction);

impl Instruction for I2f {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_int();
        stack.push_float(v1 as f32);
    }
}

pub struct I2l(NoOperandsInstruction);

impl Instruction for I2l {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_int();
        stack.push_long(v1 as i64);
    }
}