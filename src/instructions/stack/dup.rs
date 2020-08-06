use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

///dup
pub struct Dup(NoOperandsInstruction);

impl Dup {
    #[inline]
    pub const fn new() -> Dup {
        return Dup(NoOperandsInstruction::new());
    }
}

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

impl DupX1 {
    #[inline]
    pub const fn new() -> DupX1 {
        return DupX1(NoOperandsInstruction::new());
    }
}

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

impl DupX2 {
    #[inline]
    pub const fn new() -> DupX2 {
        return DupX2(NoOperandsInstruction::new());
    }
}

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

impl Dup2 {
    #[inline]
    pub const fn new() -> Dup2 {
        return Dup2(NoOperandsInstruction::new());
    }
}

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

impl Dup2X1 {
    #[inline]
    pub const fn new() -> Dup2X1 {
        return Dup2X1(NoOperandsInstruction::new());
    }
}

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

impl Dup2X2 {
    #[inline]
    pub const fn new() -> Dup2X2 {
        return Dup2X2(NoOperandsInstruction::new());
    }
}

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
