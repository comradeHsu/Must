use crate::instructions::base::instruction::Instruction;
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::branch_logic::branch;

pub struct LookUpSwitch {
    default_offset:i32,
    npairs:i32,
    match_offsets:Vec<i32>
}

impl LookUpSwitch {
    #[inline]
    pub fn new() -> LookUpSwitch {
        return LookUpSwitch{
            default_offset: 0,
            npairs: 0,
            match_offsets: vec![]
        };
    }
}

impl Instruction for LookUpSwitch {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        reader.skip_padding();
        self.default_offset = reader.read_i32();
        self.npairs = reader.read_i32();
        self.match_offsets = reader.read_i32_table((self.npairs * 2)as usize);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().expect("operand_stack is none")
            .pop_int();
        for i in 0..self.npairs {
            let index = (i << 1) as usize;
            if self.match_offsets[index] == val {
                let offset = self.match_offsets[index+1];
                branch(frame,offset);
                return
            }
        }
        branch(frame,self.default_offset);
    }
}