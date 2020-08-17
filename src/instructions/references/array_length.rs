use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

pub struct ArrayLength(NoOperandsInstruction);

impl ArrayLength {
    #[inline]
    pub fn new() -> ArrayLength {
        return ArrayLength(NoOperandsInstruction::new());
    }
}

impl Instruction for ArrayLength {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let object = stack.pop_ref();
            if object.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let array_len = object.unwrap().array_length();
            stack.push_int(array_len as i32);
        })
    }
}
