use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::FieldReference;

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

    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.method();
        let current_class = current_method.class();
        let cp = (*current_class).borrow().constant_pool();

        let field_option = (*cp)
            .borrow_mut()
            .resolve_field_ref(self.0.index())
            .unwrap();
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
        let stack = frame.operand_stack().expect("stack is none");

        let first_char = desc.chars().next().unwrap();
        match first_char {
            'Z' | 'B' | 'C' | 'S' | 'I' => {
                let val = stack.pop_int();
                let reference = stack.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                (*reference.unwrap())
                    .borrow_mut()
                    .fields()
                    .set_int(slot_id, val);
            }
            'F' => {
                let val = stack.pop_float();
                let reference = stack.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                (*reference.unwrap())
                    .borrow_mut()
                    .fields()
                    .set_float(slot_id, val);
            }
            'J' => {
                let val = stack.pop_long();
                let reference = stack.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                (*reference.unwrap())
                    .borrow_mut()
                    .fields()
                    .set_long(slot_id, val);
            }
            'D' => {
                let val = stack.pop_double();
                let reference = stack.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                (*reference.unwrap())
                    .borrow_mut()
                    .fields()
                    .set_double(slot_id, val);
            }
            'L' | '[' => {
                let val = stack.pop_ref();
                let reference = stack.pop_ref();
                if reference.is_none() {
                    panic!("java.lang.NullPointerException");
                }
                (*reference.unwrap())
                    .borrow_mut()
                    .fields()
                    .set_ref(slot_id, val);
            }
            _ => {}
        }
    }
}
