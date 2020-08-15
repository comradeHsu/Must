use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime::frame::Frame;
use crate::oops::constant_pool::Constant::FieldReference;
use crate::instructions::references::ResolveFieldRef;

pub struct GetStatic(ConstantPoolInstruction);

impl GetStatic {
    #[inline]
    pub fn new() -> GetStatic {
        return GetStatic(ConstantPoolInstruction::new());
    }
}

impl Instruction for GetStatic {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let class = frame.method().class();

        let field_option = self.resolve_field_ref(class);
        let field = (*field_option).borrow();
        let class = field.parent().class();
        if !(*class).borrow().initialized() {
            frame.revert_next_pc();
            init_class(class.clone());
            return;
        }
        if !field.parent().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let desc = field.parent().descriptor();
        let slot_id = field.slot_id();
        let mut borrow_class = (*class).borrow_mut();
        let slots = borrow_class.mut_static_vars().expect("slots is none");

        let first_char = desc.chars().next().unwrap();
        match first_char {
            'Z' | 'B' | 'C' | 'S' | 'I' => frame.push_int(slots.get_int(slot_id)),
            'F' => frame.push_float(slots.get_float(slot_id)),
            'J' => frame.push_long(slots.get_long(slot_id)),
            'D' => frame.push_double(slots.get_double(slot_id)),
            'L' | '[' => frame.push_ref(slots.get_ref(slot_id)),
            _ => {}
        }
    }
}

impl ResolveFieldRef for GetStatic {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
