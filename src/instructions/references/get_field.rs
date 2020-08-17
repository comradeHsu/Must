use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime::frame::Frame;
use crate::oops::constant_pool::Constant::FieldReference;
use crate::instructions::references::ResolveFieldRef;

pub struct GetField(ConstantPoolInstruction);

impl GetField {
    #[inline]
    pub fn new() -> GetField {
        return GetField(ConstantPoolInstruction::new());
    }
}

impl Instruction for GetField {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let class = frame.method().class();

        let field_option = self.resolve_field_ref(class);
        let field = (*field_option).borrow();
        if field.parent().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        frame.operand_stack(move |stack| {
            let reference = stack.pop_ref();
            if reference.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let desc = field.parent().descriptor();
            let slot_id = field.slot_id();

            let object = reference.unwrap();
            let first_char = desc.chars().next().unwrap();
            let func = object.fields_with(|slots|{
                match first_char {
                    'Z' | 'B' | 'C' | 'S' | 'I' => stack.push_int(slots.get_int(slot_id)),
                    'F' => stack.push_float(slots.get_float(slot_id)),
                    'J' => stack.push_long(slots.get_long(slot_id)),
                    'D' => stack.push_double(slots.get_double(slot_id)),
                    'L' | '[' => stack.push_ref(slots.get_ref(slot_id)),
                    _ => {}
                }
            });
        })
    }
}

impl ResolveFieldRef for GetField {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
