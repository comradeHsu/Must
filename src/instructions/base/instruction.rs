use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::frame::Frame;

pub trait Instruction {

    fn fetch_operands(reader:BytecodeReader);

    fn execute(frame:Frame);

}

///没有操作数的指令
struct NoOperandsInstruction {

}

impl Instruction for NoOperandsInstruction {
    fn fetch_operands(reader: BytecodeReader) {
        unimplemented!()
    }

    fn execute(frame: Frame) {
        unimplemented!()
    }
}

///跳转指令
struct BranchInstruction {
    offset:i32
}

impl Instruction for BranchInstruction {
    fn fetch_operands(reader: BytecodeReader) {
        unimplemented!()
    }

    fn execute(frame: Frame) {
        unimplemented!()
    }
}

///存储和加载指令：本地变量表
struct LocalVarsInstruction {
    index:usize
}

///存储和加载指令：常量池
struct ConstantPoolInstruction {
    index:usize
}