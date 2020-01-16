use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;
use std::borrow::Borrow;

pub struct InstanceOf(ConstantPoolInstruction);

impl InstanceOf {
    #[inline]
    pub const fn new() -> InstanceOf {
        return InstanceOf(ConstantPoolInstruction::new());
    }
}

impl Instruction for InstanceOf {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("stack is none");
        let reference = stack.pop_ref();
        if reference.is_none() {
            stack.push_int(0);
            return;
        }
        let cp = (*frame.method().class()).borrow().constant_pool();
        let constant = cp.get_constant(self.0.index());
        let class_ref = match constant {
            ClassReference(c) => c,
            _ => {}
        };
        let class = class_ref.resolved_class();
        if (*reference.unwrap()).borrow().is_instance_of(class) {
            stack.push_int(1);
        } else {
            stack.push_int(0);
        }
    }
}