use crate::instructions::base::instruction::{LocalVarsInstruction, NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn d_store(frame: &mut Frame,index:usize) {
    let val = frame.operand_stack().expect("operand_stack is empty").pop_double();
    frame.local_vars().expect("local_vars is empty").set_double(index,val);
}

///dstore
pub struct DStore(LocalVarsInstruction);

impl DStore {

    #[inline]
    pub const fn new() -> DStore {
        return DStore(LocalVarsInstruction::new());
    }

    #[inline]
    pub fn with_index(index:usize) -> DStore {
        return DStore(LocalVarsInstruction::with_index(index));
    }
}

impl Instruction for DStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_store(frame,self.0.get_index());
    }
}

///dstore_0
pub struct DStore0(NoOperandsInstruction);

impl DStore0 {
    #[inline]
    pub const fn new() -> DStore0 {
        return DStore0(NoOperandsInstruction::new());
    }
}

impl Instruction for DStore0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_store(frame,0);
    }
}

///dstore_1
pub struct DStore1(NoOperandsInstruction);

impl DStore1 {
    #[inline]
    pub const fn new() -> DStore1 {
        return DStore1(NoOperandsInstruction::new());
    }
}

impl Instruction for DStore1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_store(frame,1);
    }
}

///dstore_2
pub struct DStore2(NoOperandsInstruction);

impl DStore2 {
    #[inline]
    pub const fn new() -> DStore2 {
        return DStore2(NoOperandsInstruction::new());
    }
}

impl Instruction for DStore2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_store(frame,2);
    }
}

///dstore_3
pub struct DStore3(NoOperandsInstruction);

impl DStore3 {
    #[inline]
    pub const fn new() -> DStore3 {
        return DStore3(NoOperandsInstruction::new());
    }
}

impl Instruction for DStore3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        d_store(frame,3);
    }
}