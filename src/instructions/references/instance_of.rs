use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;
use std::borrow::Borrow;

pub struct InstanceOf(ConstantPoolInstruction);

impl InstanceOf {
    #[inline]
    pub fn new() -> InstanceOf {
        return InstanceOf(ConstantPoolInstruction::new());
    }
}

impl Instruction for InstanceOf {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        //        let stack = frame.operand_stack().expect("stack is none");
        let reference = frame.operand_stack().expect("stack is none").pop_ref();
        if reference.is_none() {
            frame.operand_stack().expect("stack is none").push_int(0);
            return;
        }
        let class = frame.method().class();
        let cp = (*class).borrow().constant_pool();
        let mut borrow_cp = (*cp).borrow_mut();
        let constant = borrow_cp.get_constant(self.0.index());
        let class_ref = match constant {
            ClassReference(c) => c,
            _ => panic!("Unknown constant type"),
        };
        let class = class_ref.resolved_class();
        if (*reference.unwrap()).borrow().is_instance_of(class) {
            frame.operand_stack().expect("stack is none").push_int(1);
        } else {
            frame.operand_stack().expect("stack is none").push_int(0);
        }
    }
}
