use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct MonitorEnter(NoOperandsInstruction);

impl MonitorEnter {
    #[inline]
    pub fn new() -> Self {
        return MonitorEnter(NoOperandsInstruction::new());
    }
}

impl Instruction for MonitorEnter {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let object = frame.pop_ref();
        if object.is_none() {
            panic!("java.lang.NullPointerException")
        }
    }
}

pub struct MonitorExit(NoOperandsInstruction);

impl MonitorExit {
    #[inline]
    pub fn new() -> Self {
        return MonitorExit(NoOperandsInstruction::new());
    }
}

impl Instruction for MonitorExit {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let object = frame.pop_ref();
        if object.is_none() {
            panic!("java.lang.NullPointerException")
        }
    }
}
