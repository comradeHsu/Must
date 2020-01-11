use crate::instructions::base::instruction::Instruction;
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::branch_logic::branch;

pub struct TableSwitch {
    default_offset:i32,
    low:i32,
    high:i32,
    jump_offsets:Vec<i32>
}

impl Instruction for TableSwitch {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        reader.skip_padding();
        self.default_offset = reader.read_i32();
        self.low = reader.read_i32();
        self.high = reader.read_i32();
        let jump_offset_count = self.high - self.low + 1;
        self.jump_offsets = reader.read_i32_table(jump_offset_count as usize);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().expect("operand_stack is none")
            .pop_int();
        let mut offset = 0;
        if index >= self.low && index <= self.high {
            offset = *self.jump_offsets.get((index-self.low) as usize)
                .expect("jump_offsets' index is small");
        } else {
            offset = self.default_offset;
        }
        branch(frame,offset);
    }
}