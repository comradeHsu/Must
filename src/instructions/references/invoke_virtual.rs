use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::jvm::Jvm;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::MethodReference;
use crate::runtime_data_area::heap::method_ref::MethodRef;
use std::ops::Deref;
use std::rc::Rc;
use crate::runtime_data_area::heap::method::Method;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;
use crate::instructions::references::ResolveMethodRef;
use crate::invoke_support::throw_exception;

pub struct InvokeVirtual(ConstantPoolInstruction);

impl InvokeVirtual {
    #[inline]
    pub fn new() -> InvokeVirtual {
        return InvokeVirtual(ConstantPoolInstruction::new());
    }
}

impl Instruction for InvokeVirtual {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.method().class();

        let resolved_method = self.resolved_method_ref(current_class.clone());
        if resolved_method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let object = frame
            .operand_stack()
            .expect("stack is none")
            .get_ref_from_top(resolved_method.arg_slot_count() - 1);
        if object.is_none() {
            //            if method_ref.name() == "println" {
            //                InvokeVirtual::hack_println(frame,method_ref.descriptor());
            //                return;
            //            }
            throw_exception(frame, "java/lang/NullPointerException", None);
            return;
            //panic!("java.lang.NullPointerException");
        }
        let obj_class = (*object.unwrap()).borrow().class();
        let resolved_method_class = resolved_method.class();
        if resolved_method.is_protected()
            && (*resolved_method_class)
                .borrow()
                .is_super_class_of((*current_class).borrow().deref())
            && (*resolved_method_class).borrow().package_name()
                != (*current_class).borrow().package_name()
            && obj_class != current_class
            && !(*obj_class)
                .borrow()
                .is_sub_class_of((*current_class).borrow().deref())
        {
            if !((*obj_class).borrow().is_array() && resolved_method.name() == "clone") {
                panic!("java.lang.IllegalAccessError")
            }
            //            panic!("java.lang.IllegalAccessError")
        }

        let method_to_be_invoked = MethodRef::look_up_method_in_class(
            obj_class,
            resolved_method.name(),
            resolved_method.descriptor(),
        );
        if method_to_be_invoked.is_none() || method_to_be_invoked.as_ref().unwrap().is_abstract() {
            panic!("java.lang.AbstractMethodError")
        }

        invoke_method(frame, method_to_be_invoked.unwrap());
    }
}

impl ResolveMethodRef for InvokeVirtual {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
