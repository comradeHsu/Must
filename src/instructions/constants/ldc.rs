use crate::instructions::base::instruction::{LocalVarsInstruction, ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::{Integer, Float, Long, Double, Str, ClassReference};
use crate::runtime_data_area::heap::string_pool::StringPool;

pub struct LDC(LocalVarsInstruction);

impl LDC {
    #[inline]
    pub fn new() -> LDC {
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
    pub fn new() -> LDCw {
        return LDCw(ConstantPoolInstruction::new());
    }
}

impl Instruction for LDCw {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        ldc(frame, self.0.index());
    }
}

pub struct LDC2w(ConstantPoolInstruction);

impl LDC2w {
    #[inline]
    pub fn new() -> LDC2w {
        return LDC2w(ConstantPoolInstruction::new());
    }
}

impl Instruction for LDC2w {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
//        let stack = frame.operand_stack().expect("stack is none");
        let class = frame.method().class();
        let cp = (*class).borrow().constant_pool();
        let borrow_cp = cp.borrow();
        let constant = borrow_cp.get_constant_immutable(self.0.index());
        match constant {
            Long(v) => frame.operand_stack().expect("stack is none").push_long(*v),
            Double(v) => frame.operand_stack().expect("stack is none").push_double(*v),
            _ => panic!("java.lang.ClassFormatError")
        }
    }
}

fn ldc(frame: &mut Frame, index:usize) {
//    let stack = frame.operand_stack().expect("stack is none");
    let class = frame.method().class();
    let cp = (*class).borrow().constant_pool();
    let borrow_cp = cp.borrow();
    let constant = borrow_cp.get_constant_immutable(index);
    let mut borrow_cp = cp.borrow_mut();
    let constant = borrow_cp.get_constant(index);
    println!("constant:{:?}",constant);
    match constant {
        Integer(v) => frame.operand_stack().expect("stack is none").push_int(*v),
        Float(v) => frame.operand_stack().expect("stack is none").push_float(*v),
        Str(v) => {
            let string = StringPool::java_string((*class).borrow().loader(),v.clone());
            frame.operand_stack().expect("stack is none").push_ref(Some(string))
        },
        ClassReference(v) => {
            let class = v.resolved_class(class);
            let borrow = (*class).borrow();
            let obj = borrow.java_class();
            frame.operand_stack().expect("stack is none").push_ref(Some(obj.unwrap().clone()));
        },
        _ => panic!("todo: ldc!")
    }
}