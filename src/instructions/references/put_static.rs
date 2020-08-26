use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::references::ResolveFieldRef;



use crate::runtime::frame::Frame;




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

    fn execute(&mut self, frame: &Frame) {
        let current_method = frame.method();
        let current_class = current_method.class();

        let field = self.resolve_field_ref(&current_class);
        let class = field.parent().class();
        if !class.initialized() {
            frame.revert_next_pc();
            init_class(class.clone());
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
        let first_char = desc.chars().next().unwrap();
        match first_char {
            'Z' | 'B' | 'C' | 'S' | 'I' => class.set_static_int(slot_id, frame.pop_int()),
            'F' => class.set_static_float(slot_id, frame.pop_float()),
            'J' => class.set_static_long(slot_id, frame.pop_long()),
            'D' => class.set_static_double(slot_id, frame.pop_double()),
            'L' | '[' => class.set_static_ref(slot_id, frame.pop_ref()),
            _ => {}
        }
    }
}

impl ResolveFieldRef for PutStatic {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
