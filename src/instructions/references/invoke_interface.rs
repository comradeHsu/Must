use crate::instructions::base::instruction::Instruction;
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::InterfaceMethodReference;
use std::ops::Deref;
use crate::runtime_data_area::heap::method_ref::MethodRef;
use crate::instructions::base::method_invoke_logic::invoke_method;

pub struct InvokeInterface {
    index:usize
}

impl Instruction for InvokeInterface {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as usize;
        reader.read_u8();
        reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.method().class();
        let cp = (*current_class).borrow().constant_pool();
        let mut borrow_cp = (*cp).borrow_mut();
        let constant = borrow_cp.get_constant(self.index);
        let method_ref = match constant {
            InterfaceMethodReference(c) => c,
            _ => panic!("Unknown constant type")
        };
        let resolved_method = method_ref.resolved_interface_method().unwrap();
        if resolved_method.is_static() || resolved_method.is_private() {
            panic!("java.lang.IncompatibleClassChangeError")
        }

        let object = frame.operand_stack().expect("stack is none")
            .get_ref_from_top(resolved_method.arg_slot_count()-1);

        if object.is_none() {
            panic!("java.lang.NullPointerException") // todo
        }
        let object_class = (*object.unwrap()).borrow().class();
        let interface = method_ref.resolved_class(current_class.clone());
        if !(*object_class).borrow().is_implements((*interface).borrow().deref()) {
            panic!("java.lang.IncompatibleClassChangeError")
        }
        let method_to_be_invoked = MethodRef::look_up_method_in_class(object_class,
                                                                      method_ref.name(), method_ref.descriptor());
        if method_to_be_invoked.is_none() || method_to_be_invoked.as_ref().unwrap().is_abstract() {
            panic!("java.lang.AbstractMethodError")
        }
        if !method_to_be_invoked.as_ref().unwrap().is_public() {
            panic!("java.lang.IllegalAccessError")
        }

        invoke_method(frame, method_to_be_invoked.unwrap());
    }
}