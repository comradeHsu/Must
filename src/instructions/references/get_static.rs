use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::references::ResolveFieldRef;

use crate::runtime::frame::Frame;

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

        let field = self.resolve_field_ref(&class);
        let class = field.parent().class();
        if !class.initialized() {
            frame.revert_next_pc();
            init_class(class.clone());
            return;
        }
        if !field.parent().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let desc = field.parent().descriptor();
        let slot_id = field.slot_id();

        let first_char = desc.chars().next().unwrap();
        match first_char {
            'Z' | 'B' | 'C' | 'S' | 'I' => frame.push_int(class.get_static_int(slot_id)),
            'F' => frame.push_float(class.get_static_float(slot_id)),
            'J' => frame.push_long(class.get_static_long(slot_id)),
            'D' => frame.push_double(class.get_static_double(slot_id)),
            'L' | '[' => frame.push_ref(class.get_static_ref(slot_id)),
            _ => {}
        }
    }
}

impl ResolveFieldRef for GetStatic {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
