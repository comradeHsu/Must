use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::instructions::references::ResolveMethodRef;

use crate::oops::method_ref::MethodRef;
use crate::runtime::frame::Frame;
use std::borrow::Borrow;
use std::ops::Deref;

pub struct InvokeSpecial(ConstantPoolInstruction);

impl InvokeSpecial {
    #[inline]
    pub fn new() -> InvokeSpecial {
        return InvokeSpecial(ConstantPoolInstruction::new());
    }
}

impl Instruction for InvokeSpecial {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let class = frame.method().class();

        let (resolved_class, resolved_method) = self.resolved_method_ref_tuple(&class);

        if resolved_method.name() == "<init>" && resolved_method.class() != resolved_class {
            panic!("java.lang.NoSuchMethodError")
        }
        if resolved_method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError")
        }
        let object = frame.get_ref_from_top(resolved_method.arg_slot_count() - 1);
        if object.is_none() {
            panic!("java.lang.NullPointerException");
        }

        let method_class = resolved_method.class();
        let object_class = object.unwrap().class();
        if resolved_method.is_protected()
            && method_class.is_super_class_of(&class)
            && method_class.package_name() != class.package_name()
            && object_class != class
            && !object_class
                .is_sub_class_of(&class)
        {
            panic!("java.lang.IllegalAccessError");
        };
        let mut method_to_be_invoked = Some(resolved_method.clone());
        if class.is_super()
            && resolved_class
                .is_super_class_of(&class)
            && resolved_method.name() != "<init>"
        {
            method_to_be_invoked = MethodRef::look_up_method_in_class(
                &class.super_class().unwrap(),
                resolved_method.name(),
                resolved_method.descriptor(),
            );
        }

        if method_to_be_invoked.is_none() || method_to_be_invoked.as_ref().unwrap().is_abstract() {
            panic!("java.lang.AbstractMethodError")
        }

        invoke_method(frame, method_to_be_invoked.unwrap());
    }
}

impl ResolveMethodRef for InvokeSpecial {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
