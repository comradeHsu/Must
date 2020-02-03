use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::MethodReference;
use std::ops::Deref;
use crate::runtime_data_area::heap::method_ref::MethodRef;
use crate::instructions::base::method_invoke_logic::invoke_method;

pub struct InvokeVirtual(ConstantPoolInstruction);

impl InvokeVirtual {
    #[inline]
    pub fn new() -> InvokeVirtual {
        return InvokeVirtual(ConstantPoolInstruction::new());
    }

    fn hack_println(frame:&mut Frame,desc:&str) {
        let stack = frame.operand_stack().expect("stack is none");
        match desc {
            "(Z)V" => println!("{}",stack.pop_int() != 0),
            "(C)V" => println!("{}",stack.pop_int() as u8 as char),
            "(I)V" | "(B)V" | "(S)V" => println!("{}",stack.pop_int()),
            "(F)V" => println!("{}",stack.pop_float()),
            "(J)V" => println!("{}",stack.pop_long()),
            "(D)V" => println!("{}",stack.pop_double()),
            "(Ljava/lang/String;)V" => {
                let java_str = stack.pop_ref();
                let mete_str = (*java_str.unwrap()).borrow()
                    .get_ref_var("value", "[C").expect("str is null");
                let borrow = (*mete_str).borrow();
                let string = borrow.chars();
                let target = String::from_utf16(string).expect("u16 seqs has mistake");
                println!("{}",target);
            },
            _ => panic!("println: {}",desc)
        }
        stack.pop_ref();
    }
}

impl Instruction for InvokeVirtual {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.method().class();
        let cp = (*current_class).borrow().constant_pool();
        let mut borrow_cp = (*cp).borrow_mut();
        let constant = borrow_cp.get_constant(self.0.index());
        let method_ref = match constant {
            MethodReference(c) => c,
            _ => panic!("Unknown constant type")
        };
        let resolved_method = method_ref.resolved_method(current_class.clone()).unwrap();
        if resolved_method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let object = frame.operand_stack().expect("stack is none")
            .get_ref_from_top(resolved_method.arg_slot_count()-1);
        if object.is_none() {
//            if method_ref.name() == "println" {
//                InvokeVirtual::hack_println(frame,method_ref.descriptor());
//                return;
//            }
            panic!("java.lang.NullPointerException");
        }
        let obj_class = (*object.unwrap()).borrow().class();
        let resolved_method_class = resolved_method.class();
        if resolved_method.is_protected() &&
            (*resolved_method_class).borrow().is_super_class_of((*current_class).borrow().deref()) &&
            (*resolved_method_class).borrow().package_name() != (*current_class).borrow().package_name() &&
            obj_class != current_class &&
            !(*obj_class).borrow().is_sub_class_of((*current_class).borrow().deref()) {

            panic!("java.lang.IllegalAccessError")
        }

        let method_to_be_invoked = MethodRef::look_up_method_in_class(obj_class,
                                                                      method_ref.name(), method_ref.descriptor());
        if method_to_be_invoked.is_none() || method_to_be_invoked.as_ref().unwrap().is_abstract() {
            panic!("java.lang.AbstractMethodError")
        }

        invoke_method(frame, method_to_be_invoked.unwrap());
    }
}