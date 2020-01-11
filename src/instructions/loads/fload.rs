use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::instruction::{LocalVarsInstruction, Instruction, NoOperandsInstruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn f_load(frame: &mut Frame, index:usize) {
    let val = frame.local_vars().expect("local_vars is empty")
        .get_float(index);
    frame.operand_stack().expect("operand_stack is empty")
        .push_float(val);
}
///fload
pub struct FLoad(LocalVarsInstruction);

impl FLoad {

    #[inline]
    pub const fn new() -> FLoad {
        return FLoad(LocalVarsInstruction::new());
    }

    #[inline]
    pub fn with_index(index:usize) -> FLoad {
        return FLoad(LocalVarsInstruction::with_index(index));
    }
}

impl Instruction for FLoad {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_load(frame, self.0.get_index());
    }
}

///fload_0
pub struct FLoad0(NoOperandsInstruction);

impl FLoad0 {
    #[inline]
    pub const fn new() -> FLoad0 {
        return FLoad0(NoOperandsInstruction::new());
    }
}

impl Instruction for FLoad0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_load(frame,0);
    }
}

///fload_1
pub struct FLoad1(NoOperandsInstruction);

impl FLoad1 {
    #[inline]
    pub const fn new() -> FLoad1 {
        return FLoad1(NoOperandsInstruction::new());
    }
}

impl Instruction for FLoad1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_load(frame,1);
    }
}

///fload_2
pub struct FLoad2(NoOperandsInstruction);

impl FLoad2 {
    #[inline]
    pub const fn new() -> FLoad2 {
        return FLoad2(NoOperandsInstruction::new());
    }
}

impl Instruction for FLoad2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_load(frame,2);
    }
}

///fload_3
pub struct FLoad3(NoOperandsInstruction);

impl FLoad3 {
    #[inline]
    pub const fn new() -> FLoad3 {
        return FLoad3(NoOperandsInstruction::new());
    }
}

impl Instruction for FLoad3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_load(frame,3);
    }
}