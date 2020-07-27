use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::FieldReference;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::class::Class;
use crate::instructions::references::ResolveFieldRef;

pub struct PutStatic(ConstantPoolInstruction);

impl PutStatic {
    #[inline]
    pub fn new() -> PutStatic {
        return PutStatic(ConstantPoolInstruction::new());
    }
}

impl Instruction for PutStatic {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.method();
        let current_class = current_method.class();

        let field_option = self.resolve_field_ref(current_class.clone());

        let field = (*field_option).borrow();
        let class = field.parent().class();
        if !(*class).borrow().initialized() {
            frame.revert_next_pc();
            init_class(frame.thread(), class.clone());
            return;
        }
        if !field.parent().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        if field.parent().is_final() {
            if current_class != class || current_method.name() != "<clinit>" {
                panic!("java.lang.IllegalAccessError");
            }
        }
        let desc = field.parent().descriptor();
        let slot_id = field.slot_id();
        let mut borrow_class = (*class).borrow_mut();
        let slots = borrow_class.mut_static_vars().expect("slots is none");
        let stack = frame.operand_stack().expect("stack is none");
        let first_char = desc.chars().next().unwrap();
        match first_char {
            'Z' | 'B' | 'C' | 'S' | 'I' => slots.set_int(slot_id, stack.pop_int()),
            'F' => slots.set_float(slot_id, stack.pop_float()),
            'J' => slots.set_long(slot_id, stack.pop_long()),
            'D' => slots.set_double(slot_id, stack.pop_double()),
            'L' | '[' => slots.set_ref(slot_id, stack.pop_ref()),
            _ => {}
        }
    }
}

impl ResolveFieldRef for PutStatic {
    fn get_index_in_constant_pool(&self) -> usize {
        return self.0.index();
    }
}
