use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;
use crate::utils::boxed;
use std::cell::RefCell;
use std::rc::Rc;

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
        let class = (*pool).borrow_mut().resolve_class_ref(self.0.index());
        if !(*class).borrow().initialized() {
            frame.revert_next_pc();
            init_class(frame.thread(), class.clone());
            return;
        }
        let ref_class = (*class).borrow();
        if ref_class.is_interface() || ref_class.is_abstract() {
            panic!("java.lang.InstantiationError")
        }
        let object = match ref_class.is_class_loader() {
            true => Class::new_class_loader_object(&class),
            false => Class::new_object(&class),
        };
        frame
            .operand_stack()
            .expect("")
            .push_ref(Some(boxed(object)));
    }
}
