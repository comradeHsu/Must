use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::instruction::{LocalVarsInstruction, Instruction, NoOperandsInstruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn l_load(frame: &mut Frame, index:usize) {
    let val = frame.local_vars().expect("local_vars is empty")
        .get_long(index);
    frame.operand_stack().expect("operand_stack is empty")
        .push_long(val);
}
///lload
pub struct LLoad(LocalVarsInstruction);

impl Instruction for LLoad {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        l_load(frame,self.0.get_index());
    }
}

///lload_0
pub struct LLoad0(NoOperandsInstruction);

impl Instruction for LLoad0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,0);
    }
}

///lload_1
pub struct LLoad1(NoOperandsInstruction);

impl Instruction for LLoad1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,1);
    }
}

///lload_2
pub struct LLoad2(NoOperandsInstruction);

impl Instruction for LLoad2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,2);
    }
}

///lload_3
pub struct LLoad3(NoOperandsInstruction);

impl Instruction for LLoad3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,3);
    }
}