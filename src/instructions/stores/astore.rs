use crate::instructions::base::instruction::{LocalVarsInstruction, NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn a_store(frame: &mut Frame,index:usize) {
    let val = frame.operand_stack().expect("operand_stack is empty").pop_ref();
    frame.local_vars().expect("local_vars is empty").set_ref(index,val);
}

///astore
pub struct AStore(LocalVarsInstruction);

impl AStore {

    #[inline]
    pub fn with_index(index:usize) -> AStore {
        return AStore(LocalVarsInstruction::with_index(index));
    }
}

impl Instruction for AStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_store(frame,self.0.get_index());
    }
}

///astore_0
pub struct AStore0(NoOperandsInstruction);

impl AStore0 {
    #[inline]
    pub const fn new() -> AStore0 {
        return AStore0(NoOperandsInstruction::new());
    }
}

impl Instruction for AStore0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_store(frame,0);
    }
}

///astore_1
pub struct AStore1(NoOperandsInstruction);

impl AStore1 {
    #[inline]
    pub const fn new() -> AStore1 {
        return AStore1(NoOperandsInstruction::new());
    }
}

impl Instruction for AStore1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_store(frame,1);
    }
}

///astore_2
pub struct AStore2(NoOperandsInstruction);

impl AStore2 {
    #[inline]
    pub const fn new() -> AStore2 {
        return AStore2(NoOperandsInstruction::new());
    }
}

impl Instruction for AStore2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_store(frame,2);
    }
}

///astore_3
pub struct AStore3(NoOperandsInstruction);

impl AStore3 {
    #[inline]
    pub const fn new() -> AStore3 {
        return AStore3(NoOperandsInstruction::new());
    }
}

impl Instruction for AStore3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        a_store(frame,3);
    }
}