use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::instructions::check_index;
use crate::runtime::frame::Frame;

pub struct AAload(NoOperandsInstruction);

impl AAload {
    #[inline]
    pub fn new() -> AAload {
        return AAload(NoOperandsInstruction::new());
    }
}

impl Instruction for AAload {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            object.references(|references|{
                check_index(references.len(), index);
                stack.push_ref(references[index].clone());
            });
        })
    }
}

pub struct BAload(NoOperandsInstruction);

impl BAload {
    #[inline]
    pub fn new() -> BAload {
        return BAload(NoOperandsInstruction::new());
    }
}

impl Instruction for BAload {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            object.bytes(|bytes|{
                check_index(bytes.len(), index);
                stack.push_int(bytes[index] as i32);
            });
        })
    }
}

pub struct CAload(NoOperandsInstruction);

impl CAload {
    #[inline]
    pub fn new() -> CAload {
        return CAload(NoOperandsInstruction::new());
    }
}

impl Instruction for CAload {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            object.chars(|chars|{
                check_index(chars.len(), index);
                stack.push_int(chars[index] as i32);
            });
        })
    }
}

pub struct DAload(NoOperandsInstruction);

impl DAload {
    #[inline]
    pub fn new() -> DAload {
        return DAload(NoOperandsInstruction::new());
    }
}

impl Instruction for DAload {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            object.doubles(|doubles|{
                check_index(doubles.len(), index);
                stack.push_double(doubles[index]);
            });
        })
    }
}

pub struct FAload(NoOperandsInstruction);

impl FAload {
    #[inline]
    pub fn new() -> FAload {
        return FAload(NoOperandsInstruction::new());
    }
}

impl Instruction for FAload {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            object.floats(|floats|{
                check_index(floats.len(), index);
                stack.push_float(floats[index]);
            });
        })
    }
}

pub struct IAload(NoOperandsInstruction);

impl IAload {
    #[inline]
    pub fn new() -> IAload {
        return IAload(NoOperandsInstruction::new());
    }
}

impl Instruction for IAload {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            object.ints(|ints|{
                check_index(ints.len(), index);
                stack.push_int(ints[index]);
            });
        })
    }
}

pub struct LAload(NoOperandsInstruction);

impl LAload {
    #[inline]
    pub fn new() -> LAload {
        return LAload(NoOperandsInstruction::new());
    }
}

impl Instruction for LAload {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            object.longs(|longs|{
                check_index(longs.len(), index);
                stack.push_long(longs[index]);
            });
        })
    }
}

pub struct SAload(NoOperandsInstruction);

impl SAload {
    #[inline]
    pub fn new() -> SAload {
        return SAload(NoOperandsInstruction::new());
    }
}

impl Instruction for SAload {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let index = stack.pop_int() as usize;
            let arr_ref = stack.pop_ref();
            if arr_ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let object = arr_ref.unwrap();
            object.shorts(|shorts|{
                check_index(shorts.len(), index);
                stack.push_int(shorts[index] as i32);
            });
        })
    }
}
