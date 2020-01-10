use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

///i_shl
pub struct IShl(NoOperandsInstruction);

impl Instruction for IShl {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let rs = v1 << v2;
        stack.push_int(rs);
    }
}

///i_shr
pub struct IShr(NoOperandsInstruction);

impl Instruction for IShr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let rs = v1 >> v2;
        stack.push_int(rs);
    }
}

///iu_shr
pub struct IuShr(NoOperandsInstruction);

impl Instruction for IuShr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_int() as u32;
        let v1 = stack.pop_int() as u32;
        let rs = v1 >> v2;
        stack.push_int(rs as i32);
    }
}

///l_shl
pub struct LShl(NoOperandsInstruction);

impl Instruction for LShl {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 << v2;
        stack.push_long(rs);
    }
}

///l_shr
pub struct LShr(NoOperandsInstruction);

impl Instruction for LShr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let rs = v1 >> v2;
        stack.push_long(rs);
    }
}

///lu_shr
pub struct LuShr(NoOperandsInstruction);

impl Instruction for LuShr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("operand_stack is none");
        let v2 = stack.pop_long() as u64;
        let v1 = stack.pop_long() as u64;
        let rs = v1 >> v2;
        stack.push_long(rs as i64);
    }
}