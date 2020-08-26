use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::references::ResolveClassRef;
use crate::oops::class::Class;

use crate::runtime::frame::Frame;





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

    fn execute(&mut self, frame: &Frame) {
        let class = frame.method().class();

        let class = self.resolve_class_ref(&class);
        if !class.initialized() {
            frame.revert_next_pc();
            init_class(class.clone());
            return;
        }
        if class.is_interface() || class.is_abstract() {
            panic!("java.lang.InstantiationError")
        }
        let object = match class.is_class_loader() {
            true => Class::new_class_loader_object(&class),
            false => Class::new_object(&class),
        };
        frame.push_ref(Some(object));
    }
}

impl ResolveClassRef for New {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
