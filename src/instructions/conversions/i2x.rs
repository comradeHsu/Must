use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::frame::Frame;

pub struct I2d(NoOperandsInstruction);

impl I2d {
    #[inline]
    pub const fn new() -> I2d {
        return I2d(NoOperandsInstruction::new());
    }
}

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

impl I2f {
    #[inline]
    pub const fn new() -> I2f {
        return I2f(NoOperandsInstruction::new());
    }
}

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

impl I2l {
    #[inline]
    pub const fn new() -> I2l {
        return I2l(NoOperandsInstruction::new());
    }
}

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

pub struct I2b(NoOperandsInstruction);

impl I2b {
    #[inline]
    pub const fn new() -> I2b {
        return I2b(NoOperandsInstruction::new());
    }
}

impl Instruction for I2b {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_int() as i8;
        stack.push_int(v1 as i32);
    }
}

pub struct I2c(NoOperandsInstruction);

impl I2c {
    #[inline]
    pub const fn new() -> I2c {
        return I2c(NoOperandsInstruction::new());
    }
}

impl Instruction for I2c {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_int() as u16;
        stack.push_int(v1 as i32);
    }
}

pub struct I2s(NoOperandsInstruction);

impl I2s {
    #[inline]
    pub const fn new() -> I2s {
        return I2s(NoOperandsInstruction::new());
    }
}

impl Instruction for I2s {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v1 = stack.pop_int() as i16;
        stack.push_int(v1 as i32);
    }
}