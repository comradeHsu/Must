use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct Fcmpg(NoOperandsInstruction);

impl Fcmpg {
    #[inline]
    pub const fn new() -> Fcmpg {
        return Fcmpg(NoOperandsInstruction::new());
    }
}

impl Instruction for Fcmpg {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        fcmp(frame, true);
    }
}

pub struct Fcmpl(NoOperandsInstruction);

impl Fcmpl {
    #[inline]
    pub const fn new() -> Fcmpl {
        return Fcmpl(NoOperandsInstruction::new());
    }
}

impl Instruction for Fcmpl {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        fcmp(frame, false);
    }
}

fn fcmp(frame: &Frame, flag: bool) {
    frame.operand_stack(|stack| {
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
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
    });
}
