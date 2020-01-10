use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

pub struct Swap(NoOperandsInstruction);

impl Instruction for Swap {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let slot = stack.pop_slot();
        let slot_2 = stack.pop_slot();
        stack.push_slot(slot);
        stack.push_slot(slot_2);
    }
}