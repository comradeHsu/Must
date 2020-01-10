use crate::instructions::base::instruction::Instruction;
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

pub struct IInc {
    index:usize,
    constant:i32
}

impl Instruction for IInc {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
        self.constant = reader.read_i8() as i32;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let vars = frame.local_vars().expect("operand_stack is none");
        let mut val = vars.get_int(self.index);
        val += self.constant;
        vars.set_int(self.index,val);
    }
}