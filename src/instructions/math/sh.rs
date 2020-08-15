use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;

///i_shl
pub struct IShl(NoOperandsInstruction);

impl IShl {
    #[inline]
    pub const fn new() -> IShl {
        return IShl(NoOperandsInstruction::new());
    }
}

impl Instruction for IShl {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_int();
            let v1 = stack.pop_int();
            let rs = v1 << v2;
            stack.push_int(rs);
        })
    }
}

///i_shr
pub struct IShr(NoOperandsInstruction);

impl IShr {
    #[inline]
    pub const fn new() -> IShr {
        return IShr(NoOperandsInstruction::new());
    }
}

impl Instruction for IShr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_int();
            let v1 = stack.pop_int();
            let rs = v1 >> v2;
            stack.push_int(rs);
        })
    }
}

///iu_shr
pub struct IuShr(NoOperandsInstruction);

impl IuShr {
    #[inline]
    pub const fn new() -> IuShr {
        return IuShr(NoOperandsInstruction::new());
    }
}

impl Instruction for IuShr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_int() as u32;
            let v1 = stack.pop_int() as u32;
            let rs = v1 >> v2;
            stack.push_int(rs as i32);
        })
    }
}

///l_shl
pub struct LShl(NoOperandsInstruction);

impl LShl {
    #[inline]
    pub const fn new() -> LShl {
        return LShl(NoOperandsInstruction::new());
    }
}

impl Instruction for LShl {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_int() as u32;
            let v1 = stack.pop_long();
            let (rs, _) = v1.overflowing_shl(v2);
            stack.push_long(rs);
        })
    }
}

///l_shr
pub struct LShr(NoOperandsInstruction);

impl LShr {
    #[inline]
    pub const fn new() -> LShr {
        return LShr(NoOperandsInstruction::new());
    }
}

impl Instruction for LShr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_int() as i64;
            let v1 = stack.pop_long();
            let rs = v1 >> v2;
            stack.push_long(rs);
        })
    }
}

///lu_shr
pub struct LuShr(NoOperandsInstruction);

impl LuShr {
    #[inline]
    pub const fn new() -> LuShr {
        return LuShr(NoOperandsInstruction::new());
    }
}

impl Instruction for LuShr {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        frame.operand_stack(|stack| {
            let v2 = stack.pop_int() as u64;
            let v1 = stack.pop_long() as u64;
            let rs = v1 >> v2;
            stack.push_long(rs as i64);
        })
    }
}
