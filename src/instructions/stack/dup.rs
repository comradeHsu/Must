use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

///dup
pub struct Dup(NoOperandsInstruction);

impl Instruction for Dup {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let slot = stack.pop_slot();
        stack.push_slot(slot.clone());
        stack.push_slot(slot);
    }
}

///dup_x1
pub struct DupX1(NoOperandsInstruction);

impl Instruction for DupX1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let slot = stack.pop_slot();
        let slot_2 = stack.pop_slot();
        stack.push_slot(slot.clone());
        stack.push_slot(slot_2);
        stack.push_slot(slot);
    }
}

///dup_x2
pub struct DupX2(NoOperandsInstruction);

impl Instruction for DupX2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let slot_1 = stack.pop_slot();
        let slot_2 = stack.pop_slot();
        let slot_3 = stack.pop_slot();
        stack.push_slot(slot_1.clone());
        stack.push_slot(slot_3);
        stack.push_slot(slot_2);
        stack.push_slot(slot_1);
    }
}

///dup2
pub struct Dup2(NoOperandsInstruction);

impl Instruction for Dup2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let slot_1 = stack.pop_slot();
        let slot_2 = stack.pop_slot();
        stack.push_slot(slot_2.clone());
        stack.push_slot(slot_1.clone());
        stack.push_slot(slot_2);
        stack.push_slot(slot_1);
    }
}

///dup2_x1
pub struct Dup2X1(NoOperandsInstruction);

impl Instruction for Dup2X1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let slot_1 = stack.pop_slot();
        let slot_2 = stack.pop_slot();
        let slot_3 = stack.pop_slot();
        stack.push_slot(slot_2.clone());
        stack.push_slot(slot_1.clone());
        stack.push_slot(slot_3);
        stack.push_slot(slot_2);
        stack.push_slot(slot_1);
    }
}

///dup2_x2
pub struct Dup2X2(NoOperandsInstruction);

impl Instruction for Dup2X2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let slot_1 = stack.pop_slot();
        let slot_2 = stack.pop_slot();
        let slot_3 = stack.pop_slot();
        let slot_4 = stack.pop_slot();
        stack.push_slot(slot_2.clone());
        stack.push_slot(slot_1.clone());
        stack.push_slot(slot_4);
        stack.push_slot(slot_3);
        stack.push_slot(slot_2);
        stack.push_slot(slot_1);
    }
}