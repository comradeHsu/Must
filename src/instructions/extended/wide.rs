use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::instructions::loads::aload::ALoad;
use crate::instructions::loads::dload::DLoad;
use crate::instructions::loads::fload::FLoad;
use crate::instructions::loads::iload::ILoad;
use crate::instructions::loads::lload::LLoad;
use crate::instructions::math::iinc::IInc;
use crate::instructions::stores::astore::AStore;
use crate::instructions::stores::dstore::DStore;
use crate::instructions::stores::fstore::FStore;
use crate::instructions::stores::istore::IStore;
use crate::instructions::stores::lstore::LStore;
use crate::runtime_data_area::frame::Frame;

pub struct Wide {
    modified_instruction: Box<dyn Instruction>,
}

impl Wide {
    #[inline]
    pub fn new() -> Wide {
        return Wide {
            modified_instruction: Box::new(NoOperandsInstruction::new()),
        };
    }
}

impl Instruction for Wide {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        let code = reader.read_u8();
        match code {
            0x15 => {
                let inst = ILoad::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x16 => {
                let inst = LLoad::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x17 => {
                let inst = FLoad::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x18 => {
                let inst = DLoad::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x19 => {
                let inst = ALoad::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x36 => {
                let inst = IStore::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x37 => {
                let inst = LStore::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x38 => {
                let inst = FStore::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x39 => {
                let inst = DStore::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x3a => {
                let inst = AStore::with_index(reader.read_u16() as usize);
                self.modified_instruction = Box::new(inst);
            }
            0x84 => {
                let index = reader.read_u16() as usize;
                let constant = reader.read_i16() as i32;
                let inst = IInc::init(index, constant);
                self.modified_instruction = Box::new(inst);
            }
            0xa9 => panic!("Unsupported opcode: 0xa9!"),
            _ => panic!("Unsupported opcode: {}!", code),
        }
    }

    fn execute(&mut self, frame: &mut Frame) {
        self.modified_instruction.execute(frame);
    }
}
