use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

///aconst_null
pub struct AconstNull(NoOperandsInstruction);

impl Instruction for AconstNull {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_ref(None);
    }
}

///dconst_0
pub struct Dconst0(NoOperandsInstruction);

impl Instruction for Dconst0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_double(0.0f64);
    }
}

///dconst_1
pub struct Dconst1(NoOperandsInstruction);

impl Instruction for Dconst1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_double(1.0f64);
    }
}

///fconst_0
pub struct Fconst0(NoOperandsInstruction);

impl Instruction for Fconst0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_float(0.0f32);
    }
}

///fconst_1
pub struct Fconst1(NoOperandsInstruction);

impl Instruction for Fconst1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_float(1.0f32);
    }
}

///fconst_2
pub struct Fconst2(NoOperandsInstruction);

impl Instruction for Fconst2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_float(2.0f32);
    }
}

///iconst_m1
pub struct IconstM1(NoOperandsInstruction);

impl Instruction for IconstM1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(-1i32);
    }
}

///iconst_0
pub struct Iconst0(NoOperandsInstruction);

impl Instruction for Iconst0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(0i32);
    }
}

///iconst_1
pub struct Iconst1(NoOperandsInstruction);

impl Instruction for Iconst1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(1i32);
    }
}

///iconst_2
pub struct Iconst2(NoOperandsInstruction);

impl Instruction for Iconst2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(2i32);
    }
}

///iconst_3
pub struct Iconst3(NoOperandsInstruction);

impl Instruction for Iconst3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(3i32);
    }
}

///iconst_4
pub struct Iconst4(NoOperandsInstruction);

impl Instruction for Iconst4 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(4i32);
    }
}

///iconst_5
pub struct Iconst5(NoOperandsInstruction);

impl Instruction for Iconst5 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(5i32);
    }
}

///lconst_0
pub struct Lconst0(NoOperandsInstruction);

impl Instruction for Lconst0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_long(0i64);
    }
}

///lconst_1
pub struct Lconst1(NoOperandsInstruction);

impl Instruction for Lconst1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_long(1i64);
    }
}