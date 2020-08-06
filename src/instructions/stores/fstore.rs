use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{
    Instruction, LocalVarsInstruction, NoOperandsInstruction,
};
use crate::runtime::frame::Frame;

fn f_store(frame: &mut Frame, index: usize) {
    let val = frame
        .operand_stack()
        .expect("operand_stack is empty")
        .pop_float();
    frame
        .local_vars()
        .expect("local_vars is empty")
        .set_float(index, val);
}

///fstore
pub struct FStore(LocalVarsInstruction);

impl FStore {
    #[inline]
    pub const fn new() -> FStore {
        return FStore(LocalVarsInstruction::new());
    }

    #[inline]
    pub fn with_index(index: usize) -> FStore {
        return FStore(LocalVarsInstruction::with_index(index));
    }
}

impl Instruction for FStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_store(frame, self.0.get_index());
    }
}

///fstore_0
pub struct FStore0(NoOperandsInstruction);

impl FStore0 {
    #[inline]
    pub const fn new() -> FStore0 {
        return FStore0(NoOperandsInstruction::new());
    }
}

impl Instruction for FStore0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_store(frame, 0);
    }
}

///fstore_1
pub struct FStore1(NoOperandsInstruction);

impl FStore1 {
    #[inline]
    pub const fn new() -> FStore1 {
        return FStore1(NoOperandsInstruction::new());
    }
}

impl Instruction for FStore1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_store(frame, 1);
    }
}

///fstore_2
pub struct FStore2(NoOperandsInstruction);

impl FStore2 {
    #[inline]
    pub const fn new() -> FStore2 {
        return FStore2(NoOperandsInstruction::new());
    }
}

impl Instruction for FStore2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_store(frame, 2);
    }
}

///fstore_3
pub struct FStore3(NoOperandsInstruction);

impl FStore3 {
    #[inline]
    pub const fn new() -> FStore3 {
        return FStore3(NoOperandsInstruction::new());
    }
}

impl Instruction for FStore3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        f_store(frame, 3);
    }
}
