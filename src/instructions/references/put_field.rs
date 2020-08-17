use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime::frame::Frame;
use crate::oops::constant_pool::Constant::FieldReference;
use std::cell::RefCell;
use std::rc::Rc;
use crate::oops::class::Class;
use crate::oops::field::Field;
use crate::instructions::references::ResolveFieldRef;

pub struct PutField(ConstantPoolInstruction);

impl PutField {
    #[inline]
    pub fn new() -> PutField {
        return PutField(ConstantPoolInstruction::new());
    }
}

impl Instruction for PutField {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let current_method = frame.method();
        let current_class = current_method.class();

        let field_option = self.resolve_field_ref(current_class.clone());

        let field = (*field_option).borrow();
        let class = field.parent().class();
        if field.parent().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        if field.parent().is_final() {
            if current_class != class || current_method.name() != "<init>" {
                panic!("java.lang.IllegalAccessError");
            }
        }
        let desc = field.parent().descriptor();
        let slot_id = field.slot_id();

        let first_char = desc.chars().next().unwrap();
        match first_char {
            'Z' | 'B' | 'C' | 'S' | 'I' => {
                let val = frame.pop_int();
                let reference = frame.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                reference.unwrap().mut_fields_with(|fields| fields.set_int(slot_id,val));
            }
            'F' => {
                let val = frame.pop_float();
                let reference = frame.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                reference.unwrap().mut_fields_with(|fields| fields.set_float(slot_id,val));
            }
            'J' => {
                let val = frame.pop_long();
                let reference = frame.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                reference.unwrap().mut_fields_with(|fields| fields.set_long(slot_id,val));
            }
            'D' => {
                let val = frame.pop_double();
                let reference = frame.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                reference.unwrap().mut_fields_with(|fields| fields.set_double(slot_id,val));
            }
            'L' | '[' => {
                let val = frame.pop_ref();
                let reference = frame.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                reference.unwrap().mut_fields_with(|fields| fields.set_ref(slot_id,val));
            }
            _ => {}
        }
    }
}

impl ResolveFieldRef for PutField {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
