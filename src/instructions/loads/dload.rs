use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::instruction::{LocalVarsInstruction, Instruction, NoOperandsInstruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn d_load(frame: &mut Frame, index:usize) {
    let val = frame.local_vars().expect("local_vars is empty")
        .get_double(index);
    frame.operand_stack().expect("operand_stack is empty")
        .push_double(val);
}
///dload
pub struct DLoad(LocalVarsInstruction);

impl DLoad {

    #[inline]
    pub const fn new() -> DLoad {
        return DLoad(LocalVarsInstruction::new());
    }

    #[inline]
    pub fn with_index(index:usize) -> DLoad {
        return DLoad(LocalVarsInstruction::with_index(index));
    }
}

impl Instruction for DLoad {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_load(frame, self.0.get_index());
    }
}

///dload_0
pub struct DLoad0(NoOperandsInstruction);

impl DLoad0 {
    #[inline]
    pub const fn new() -> DLoad0 {
        return DLoad0(NoOperandsInstruction::new());
    }
}

impl Instruction for DLoad0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_load(frame,0);
    }
}

///dload_1
pub struct DLoad1(NoOperandsInstruction);

impl DLoad1 {
    #[inline]
    pub const fn new() -> DLoad1 {
        return DLoad1(NoOperandsInstruction::new());
    }
}

impl Instruction for DLoad1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_load(frame,1);
    }
}

///dload_2
pub struct DLoad2(NoOperandsInstruction);

impl DLoad2 {
    #[inline]
    pub const fn new() -> DLoad2 {
        return DLoad2(NoOperandsInstruction::new());
    }
}

impl Instruction for DLoad2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_load(frame,2);
    }
}

///dload_3
pub struct DLoad3(NoOperandsInstruction);

impl DLoad3 {
    #[inline]
    pub const fn new() -> DLoad3 {
        return DLoad3(NoOperandsInstruction::new());
    }
}

impl Instruction for DLoad3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_load(frame,3);
    }
}