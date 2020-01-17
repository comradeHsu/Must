use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;

pub struct CheckCast(ConstantPoolInstruction);

impl CheckCast {
    #[inline]
    pub fn new() -> CheckCast {
        return CheckCast(ConstantPoolInstruction::new());
    }
}

impl Instruction for CheckCast {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("stack is none");
        let reference = stack.pop_ref();
        if reference.is_none() {
            return;
        }
        let cp = (*frame.method().class()).borrow().constant_pool();
        let mut borrow_cp = (*cp).borrow_mut();
        let constant = borrow_cp.get_constant(self.0.index());
        let class_ref = match constant {
            ClassReference(c) => c,
            _ => panic!("Unknown constant type")
        };
        let class = class_ref.resolved_class();
        if !(*reference.unwrap()).borrow().is_instance_of(class) {
            panic!("java.lang.ClassCastException");
        }
    }
}