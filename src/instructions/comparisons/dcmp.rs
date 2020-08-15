use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct Dcmpg(NoOperandsInstruction);

impl Dcmpg {
    #[inline]
    pub const fn new() -> Dcmpg {
        return Dcmpg(NoOperandsInstruction::new());
    }
}

impl Instruction for Dcmpg {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        dcmp(frame, true);
    }
}

pub struct Dcmpl(NoOperandsInstruction);

impl Dcmpl {
    #[inline]
    pub const fn new() -> Dcmpl {
        return Dcmpl(NoOperandsInstruction::new());
    }
}

impl Instruction for Dcmpl {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        dcmp(frame, false);
    }
}

fn dcmp(frame: &Frame, flag: bool) {
    frame.operand_stack(|stack|
        {
            let v2 = stack.pop_double();
            let v1 = stack.pop_double();
            if v1 > v2 {
                stack.push_int(1);
            } else if v1 == v2 {
                stack.push_int(0);
            } else if v1 < v2 {
                stack.push_int(-1);
            } else if flag {
                stack.push_int(1);
            } else {
                stack.push_int(-1);
            }
        }
    );
}
