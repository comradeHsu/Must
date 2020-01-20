use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;
use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use std::cell::RefCell;

pub struct New(ConstantPoolInstruction);

impl New {
    #[inline]
    pub fn new() -> New {
        return New(ConstantPoolInstruction::new());
    }
}

impl Instruction for New {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let class = frame.method().class();
        let pool = (*class).borrow().constant_pool();
        let mut borrow_pool = (*pool).borrow_mut();
        let constant = borrow_pool.get_constant(self.0.index());
        let class_ref = match constant {
            ClassReference(c) => c,
            _ => panic!("Unknown constant type")
        };
        let class = class_ref.resolved_class(class.clone());
        let ref_class= (*class).borrow();
        if ref_class.is_interface() || ref_class.is_abstract(){
            panic!("java.lang.InstantiationError")
        }
        let object = Class::new_object(&class);
        frame.operand_stack().expect("").push_ref(Some(Rc::new(RefCell::new(object))));
    }
}