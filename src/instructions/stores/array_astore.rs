use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::instructions::check_index;
use crate::runtime::frame::Frame;

pub struct AAStore(NoOperandsInstruction);

impl AAStore {
    #[inline]
    pub fn new() -> AAStore {
        return AAStore(NoOperandsInstruction::new());
    }
}

impl Instruction for AAStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let val = stack.pop_ref();
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            let mut borrow = (*object).borrow_mut();
            let references = borrow.mut_references();
            check_index(references.len(), index);
            references[index] = val;
        })
    }
}

pub struct BAStore(NoOperandsInstruction);

impl BAStore {
    #[inline]
    pub fn new() -> BAStore {
        return BAStore(NoOperandsInstruction::new());
    }
}

impl Instruction for BAStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let val = stack.pop_int();
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            let mut borrow = (*object).borrow_mut();
            let bytes = borrow.mut_bytes();
            check_index(bytes.len(), index);
            bytes[index] = val as i8;
        })
    }
}

pub struct CAStore(NoOperandsInstruction);

impl CAStore {
    #[inline]
    pub fn new() -> CAStore {
        return CAStore(NoOperandsInstruction::new());
    }
}

impl Instruction for CAStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let val = stack.pop_int();
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            let mut borrow = (*object).borrow_mut();
            let chars = borrow.mut_chars();
            check_index(chars.len(), index);
            chars[index] = val as u16;
        })
    }
}

pub struct DAStore(NoOperandsInstruction);

impl DAStore {
    #[inline]
    pub fn new() -> DAStore {
        return DAStore(NoOperandsInstruction::new());
    }
}

impl Instruction for DAStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let val = stack.pop_double();
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            let mut borrow = (*object).borrow_mut();
            let doubles = borrow.mut_doubles();
            check_index(doubles.len(), index);
            doubles[index] = val;
        })
    }
}

pub struct FAStore(NoOperandsInstruction);

impl FAStore {
    #[inline]
    pub fn new() -> FAStore {
        return FAStore(NoOperandsInstruction::new());
    }
}

impl Instruction for FAStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let val = stack.pop_float();
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            let mut borrow = (*object).borrow_mut();
            let floats = borrow.mut_floats();
            check_index(floats.len(), index);
            floats[index] = val;
        })
    }
}

pub struct IAStore(NoOperandsInstruction);

impl IAStore {
    #[inline]
    pub fn new() -> IAStore {
        return IAStore(NoOperandsInstruction::new());
    }
}

impl Instruction for IAStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let val = stack.pop_int();
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            let mut borrow = (*object).borrow_mut();
            let ints = borrow.mut_ints();
            check_index(ints.len(), index);
            ints[index] = val;
        })
    }
}

pub struct LAStore(NoOperandsInstruction);

impl LAStore {
    #[inline]
    pub fn new() -> LAStore {
        return LAStore(NoOperandsInstruction::new());
    }
}

impl Instruction for LAStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let val = stack.pop_long();
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            let mut borrow = (*object).borrow_mut();
            let longs = borrow.mut_longs();
            check_index(longs.len(), index);
            longs[index] = val;
        })
    }
}

pub struct SAStore(NoOperandsInstruction);

impl SAStore {
    #[inline]
    pub fn new() -> SAStore {
        return SAStore(NoOperandsInstruction::new());
    }
}

impl Instruction for SAStore {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let val = stack.pop_int();
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            let mut borrow = (*object).borrow_mut();
            let shorts = borrow.mut_shorts();
            check_index(shorts.len(), index);
            shorts[index] = val as i16;
        })
    }
}
