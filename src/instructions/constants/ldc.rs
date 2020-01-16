use crate::instructions::base::instruction::{LocalVarsInstruction, ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::{Integer, Float, Long, Double};

pub struct LDC(LocalVarsInstruction);

impl LDC {
    #[inline]
    pub const fn new() -> LDC {
        return LDC(LocalVarsInstruction::new());
    }
}

impl Instruction for LDC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        ldc(frame, self.0.get_index());
    }
}

pub struct LDCw(ConstantPoolInstruction);

impl LDCw {
    #[inline]
    pub const fn new() -> LDCw {
        return LDCw(ConstantPoolInstruction::new());
    }
}

impl Instruction for LDCw {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        ldc(frame, self.0.get_index());
    }
}

pub struct LDC2w(ConstantPoolInstruction);

impl LDC2w {
    #[inline]
    pub const fn new() -> LDC2w {
        return LDC2w(ConstantPoolInstruction::new());
    }
}

impl Instruction for LDC2w {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("stack is none");
        let cp = (*frame.method().class()).borrow().constant_pool();
        let constant = cp.get_constant(index);
        match constant {
            Long(v) => stack.push_long(*v),
            Double(v) => stack.push_double(*v),
            _ => panic!("java.lang.ClassFormatError")
        }
    }
}

fn ldc(frame: &mut Frame, index:usize) {
    let stack = frame.operand_stack().expect("stack is none");
    let cp = (*frame.method().class()).borrow().constant_pool();
    let constant = cp.get_constant(index);
    match constant {
        Integer(v) => stack.push_int(*v),
        Float(v) => stack.push_float(*v),
        _ => panic!("todo: ldc!")
    }
}