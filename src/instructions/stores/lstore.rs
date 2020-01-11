use crate::instructions::base::instruction::{LocalVarsInstruction, NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;

fn i_store(frame: &mut Frame,index:usize) {
    let val = frame.operand_stack().expect("operand_stack is empty").pop_long();
    frame.local_vars().expect("local_vars is empty").set_long(index,val);
}

///lstore
pub struct LStore(LocalVarsInstruction);

impl LStore {

    #[inline]
    pub fn with_index(index:usize) -> LStore {
        return LStore(LocalVarsInstruction::with_index(index));
    }
}

impl Instruction for LStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,self.0.get_index());
    }
}

///lstore_0
pub struct LStore0(NoOperandsInstruction);

impl LStore0 {
    #[inline]
    pub const fn new() -> LStore0 {
        return LStore0(NoOperandsInstruction::new());
    }
}

impl Instruction for LStore0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,0);
    }
}

///lstore_1
pub struct LStore1(NoOperandsInstruction);

impl LStore1 {
    #[inline]
    pub const fn new() -> LStore1 {
        return LStore1(NoOperandsInstruction::new());
    }
}

impl Instruction for LStore1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,1);
    }
}

///lstore_2
pub struct LStore2(NoOperandsInstruction);

impl LStore2 {
    #[inline]
    pub const fn new() -> LStore2 {
        return LStore2(NoOperandsInstruction::new());
    }
}

impl Instruction for LStore2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,2);
    }
}

///lstore_3
pub struct LStore3(NoOperandsInstruction);

impl LStore3 {
    #[inline]
    pub const fn new() -> LStore3 {
        return LStore3(NoOperandsInstruction::new());
    }
}

impl Instruction for LStore3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        i_store(frame,3);
    }
}