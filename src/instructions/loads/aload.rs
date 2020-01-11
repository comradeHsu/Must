use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::instruction::{LocalVarsInstruction, Instruction, NoOperandsInstruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn a_load(frame: &mut Frame, index:usize) {
    let val = frame.local_vars().expect("local_vars is empty")
        .get_ref(index);
    frame.operand_stack().expect("operand_stack is empty")
        .push_ref(val);
}
///aload
pub struct ALoad(LocalVarsInstruction);

impl Instruction for ALoad {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_load(frame, self.0.get_index());
    }
}

///aload_0
pub struct ALoad0(NoOperandsInstruction);

impl Instruction for ALoad0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_load(frame,0);
    }
}

///aload_1
pub struct ALoad1(NoOperandsInstruction);

impl Instruction for ALoad1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_load(frame,1);
    }
}

///aload_2
pub struct ALoad2(NoOperandsInstruction);

impl Instruction for ALoad2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_load(frame,2);
    }
}

///aload_3
pub struct ALoad3(NoOperandsInstruction);

impl Instruction for ALoad3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_load(frame,3);
    }
}