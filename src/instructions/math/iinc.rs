use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::Instruction;
use crate::runtime::frame::Frame;

pub struct IInc {
    index: usize,
    constant: i32,
}

impl IInc {
    #[inline]
    pub fn new() -> IInc {
        return IInc {
            index: 0,
            constant: 0,
        };
    }

    #[inline]
    pub fn init(index: usize, constant: i32) -> IInc {
        return IInc { index, constant };
    }
}

impl Instruction for IInc {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
        self.constant = reader.read_i8() as i32;
    }

    fn execute(&mut self, frame: &Frame) {
        frame.local_vars_set(|vars| {
            let mut val = vars.get_int(self.index);
            val += self.constant;
            vars.set_int(self.index, val);
        })
    }
}
