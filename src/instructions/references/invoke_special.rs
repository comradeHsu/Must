use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::MethodReference;
use crate::runtime_data_area::heap::method_ref::MethodRef;
use crate::instructions::base::method_invoke_logic::invoke_method;
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

    fn execute(&mut self, frame: &mut Frame) {
        let class = frame.method().class();
        let cp = (*class).borrow().constant_pool();
        let pool_class = (*cp).borrow().class();
        let mut borrow_cp = (*cp).borrow_mut();
        let constant = borrow_cp.get_constant(self.0.index());
        let method_ref = match constant {
            MethodReference(c) => c,
            _ => panic!("Unknown constant type")
        };
        let resolved_class = method_ref.resolved_class(class.clone());

        let resolved_method = method_ref.resolved_method(pool_class).unwrap();
//        println!("resolved_method class:{}",(*resolved_method.class()).borrow().name());
        if resolved_method.name() == "<init>" && resolved_method.class() != resolved_class  {
            panic!("java.lang.NoSuchMethodError")
        }
        if resolved_method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError")
        }
        let object = frame.operand_stack()
            .expect("stack is none").get_ref_from_top(resolved_method.arg_slot_count()-1);
        if object.is_none() {
            panic!("java.lang.NullPointerException");
        }

        let method_class = resolved_method.class();
        let borrow_method_class = (*method_class).borrow();
        let object_class = (*object.unwrap()).borrow().class();
        let borrow_class = (*class).borrow();
        if resolved_method.is_protected() && borrow_method_class.is_super_class_of((*class).borrow().deref()) &&
            borrow_method_class.package_name() != borrow_class.package_name() &&
            object_class != class && !(*object_class).borrow().is_sub_class_of(borrow_class.deref()) {
            panic!("java.lang.IllegalAccessError");
        };
        let mut method_to_be_invoked = Some(resolved_method.clone());
        if borrow_class.is_super() &&
            (*resolved_class).borrow().is_super_class_of(borrow_class.deref()) &&
            resolved_method.name() != "<init>" {

            method_to_be_invoked = MethodRef::look_up_method_in_class(borrow_class.super_class().unwrap(),
                                                                      method_ref.name(), method_ref.descriptor());
        }

        if method_to_be_invoked.is_none() || method_to_be_invoked.as_ref().unwrap().is_abstract() {
            panic!("java.lang.AbstractMethodError")
        }

        invoke_method(frame, method_to_be_invoked.unwrap());
    }
}