use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime_data_area::frame::Frame;

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

    fn execute(&mut self, frame: &mut Frame) {
        let object = frame.operand_stack().expect("stack is none").pop_ref();
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

    fn execute(&mut self, frame: &mut Frame) {
        let object = frame.operand_stack().expect("stack is none").pop_ref();
        if object.is_none() {
            panic!("java.lang.NullPointerException")
        }
    }
}
