use crate::instructions::base::instruction::Instruction;
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

///bipush
pub struct BiPush {
    val:i8
}

impl BiPush {
    #[inline]
    pub const fn new() -> BiPush {
        return BiPush{ val: 0 };
    }
}

impl Instruction for BiPush {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(self.val as i32);
    }
}

///sipush
pub struct SiPush {
    val:i16
}

impl SiPush {
    #[inline]
    pub const fn new() -> SiPush {
        return SiPush{ val: 0 };
    }
}

impl Instruction for SiPush {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().expect("operand_stack is empty")
            .push_int(self.val as i32);
    }
}