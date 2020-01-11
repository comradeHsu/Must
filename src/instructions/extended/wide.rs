use crate::instructions::base::instruction::Instruction;
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

pub struct Wide {
    modified_instruction: dyn Instruction
}

impl Instruction for Wide {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        let code = reader.read_u8();
        match code {
            0x15 => {},
            0x16 => {},
            0x17 => {},
            0x18 => {},
            0x19 => {},
            0x36 => {},
            0x37 => {},
            0x38 => {},
            0x39 => {},
            0x3a => {},
            0x84 => {},
            0xa9 => {},
            _ => {}
        }
    }

    fn execute(&mut self, frame: &mut Frame) {
        unimplemented!()
    }
}