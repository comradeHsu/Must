use crate::instructions::base::instruction::{NoOperandsInstruction, LocalVarsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn i_load(frame: &mut Frame,index:usize) {
    let val = frame.local_vars().expect("local_vars is empty")
        .get_int(index);
    frame.operand_stack().expect("operand_stack is empty")
        .push_int(val);
}
///iload
pub struct ILoad(LocalVarsInstruction);

impl Instruction for ILoad {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,self.0.get_index());
    }
}

///iload_0
pub struct ILoad0(NoOperandsInstruction);

impl Instruction for ILoad0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,0);
    }
}

///iload_1
pub struct ILoad1(NoOperandsInstruction);

impl Instruction for ILoad1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,1);
    }
}

///iload_2
pub struct ILoad2(NoOperandsInstruction);

impl Instruction for ILoad2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,2);
    }
}

///iload_3
pub struct ILoad3(NoOperandsInstruction);

impl Instruction for ILoad3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_load(frame,3);
    }
}