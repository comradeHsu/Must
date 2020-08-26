use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{
    ConstantPoolInstruction, Instruction, LocalVarsInstruction,
};

use crate::oops::constant_pool::Constant::{ClassReference, Double, Float, Integer, Long, Str};
use crate::oops::string_pool::StringPool;
use crate::runtime::frame::Frame;

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

    fn execute(&mut self, frame: &Frame) {
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

    fn execute(&mut self, frame: &Frame) {
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

    fn execute(&mut self, frame: &Frame) {
        //        let stack = frame.operand_stack().expect("stack is none");
        let class = frame.method().class();
        class.constant_with(self.0.index(),|constant|{
            match constant {
                Long(v) => frame.push_long(*v),
                Double(v) => frame.push_double(*v),
                _ => panic!("java.lang.ClassFormatError"),
            }
        });
    }
}

fn ldc(frame: &Frame, index: usize) {
    //    let stack = frame.operand_stack().expect("stack is none");
    let class = frame.method().class();
    class.constant_with(index,|constant|{
        match constant {
            Integer(v) => frame.push_int(*v),
            Float(v) => frame.push_float(*v),
            Str(v) => {
                let string = StringPool::java_string(v.to_string());
                frame.push_ref(Some(string))
            }
            ClassReference(v) => {
                let class = v.resolved_class(&class);
                let obj = class.java_class();
                frame.push_ref(obj);
            }
            _ => panic!("todo: ldc!"),
        }
    })
}
