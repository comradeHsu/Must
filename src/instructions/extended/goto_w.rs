use crate::instructions::base::branch_logic::branch;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::Instruction;
use crate::runtime_data_area::frame::Frame;

pub struct GotoW {
    offset: i32,
}

impl GotoW {
    #[inline]
    pub const fn new() -> GotoW {
        return GotoW { offset: 0 };
    }
}

impl Instruction for GotoW {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i32();
    }

    fn execute(&mut self, frame: &mut Frame) {
        branch(frame, self.offset);
    }
}
