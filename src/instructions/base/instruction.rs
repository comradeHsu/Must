use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::frame::Frame;

pub trait Instruction {

    fn fetch_operands(&mut self,reader:&mut BytecodeReader);

    fn execute(&mut self,frame:&mut Frame);

}

///没有操作数的指令
pub struct NoOperandsInstruction {

}

impl Instruction for NoOperandsInstruction {
    fn fetch_operands(&mut self,reader: &mut BytecodeReader) {
        unimplemented!()
    }

    fn execute(&mut self,frame: &mut Frame) {
        unimplemented!()
    }
}

///跳转指令
pub struct BranchInstruction {
    offset:i32
}

impl Instruction for BranchInstruction {
    fn fetch_operands(&mut self,reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32;
    }

    fn execute(&mut self,frame: &mut Frame) {
        unimplemented!()
    }
}

///存储和加载指令：本地变量表
pub struct LocalVarsInstruction {
    index:usize
}

impl Instruction for LocalVarsInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        unimplemented!()
    }
}

///存储和加载指令：常量池
pub struct ConstantPoolInstruction {
    index:usize
}

impl Instruction for ConstantPoolInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        unimplemented!()
    }
}
