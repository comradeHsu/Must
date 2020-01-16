use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

pub struct InvokeSpecial(ConstantPoolInstruction);

impl InvokeSpecial {
    #[inline]
    pub const fn new() -> InvokeSpecial {
        return InvokeSpecial(ConstantPoolInstruction::new());
    }
}

impl Instruction for InvokeSpecial {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("stack is none").pop_ref();
    }
}