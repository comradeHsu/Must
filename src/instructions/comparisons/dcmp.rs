use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

pub struct Dcmpg(NoOperandsInstruction);

impl Instruction for Dcmpg {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        dcmp(frame,true);
    }
}

pub struct Dcmpl(NoOperandsInstruction);

impl Instruction for Dcmpl {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        dcmp(frame,false);
    }
}

fn dcmp(frame: &mut Frame,flag:bool) {
    let stack = frame.operand_stack().expect("operand_stack is none");
    let v2 = stack.pop_double();
    let v1 = stack.pop_double();
    if v1 > v2 {
        stack.push_int(1);
    } else if  v1 == v2 {
        stack.push_int(0);
    } else if  v1 < v2{
        stack.push_int(-1);
    } else if flag {
        stack.push_int(1);
    } else {
        stack.push_int(-1);
    }
}