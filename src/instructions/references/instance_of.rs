use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime::frame::Frame;
use crate::oops::constant_pool::Constant::ClassReference;
use std::borrow::Borrow;
use crate::instructions::references::ResolveClassRef;

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

    fn execute(&mut self, frame: &Frame) {
        //        let stack = frame.operand_stack().expect("stack is none");
        let reference = frame.pop_ref();
        if reference.is_none() {
            frame.push_int(0);
            return;
        }
        let class = frame.method().class();

        let class = self.resolve_class_ref(class);
        if (*reference.unwrap()).borrow().is_instance_of(class) {
            frame.push_int(1);
        } else {
            frame.push_int(0);
        }
    }
}

impl ResolveClassRef for InstanceOf {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
