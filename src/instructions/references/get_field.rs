use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::FieldReference;

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

    fn execute(&mut self, frame: &mut Frame) {
        let c = frame.method().class();
        let cp = (*c).borrow().constant_pool();
        let field_option = (*cp)
            .borrow_mut()
            .resolve_field_ref(self.0.index())
            .unwrap();
        let field = (*field_option).borrow();
        let class = field.parent().class();
        if field.parent().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let stack = frame.operand_stack().expect("stack is none");
        let reference = stack.pop_ref();
        if reference.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let desc = field.parent().descriptor();
        let slot_id = field.slot_id();

        let object = reference.unwrap();
        let borrow_object = (*object).borrow();
        let slots = borrow_object.fields_immutable();
        let first_char = desc.chars().next().unwrap();
        match first_char {
            'Z' | 'B' | 'C' | 'S' | 'I' => stack.push_int(slots.get_int(slot_id)),
            'F' => stack.push_float(slots.get_float(slot_id)),
            'J' => stack.push_long(slots.get_long(slot_id)),
            'D' => stack.push_double(slots.get_double(slot_id)),
            'L' | '[' => stack.push_ref(slots.get_ref(slot_id)),
            _ => {}
        }
    }
}
