use crate::class_loader::app_class_loader::ClassLoader;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::Instruction;
use crate::jvm::Jvm;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::utils::boxed;
use std::cell::RefCell;
use std::rc::Rc;

const AT_BOOLEAN: u8 = 4;
const AT_CHAR: u8 = 5;
const AT_FLOAT: u8 = 6;
const AT_DOUBLE: u8 = 7;
const AT_BYTE: u8 = 8;
const AT_SHORT: u8 = 9;
const AT_INT: u8 = 10;
const AT_LONG: u8 = 11;

pub struct NewArray {
    atype: u8,
}

impl NewArray {
    #[inline]
    pub fn new() -> NewArray {
        return NewArray { atype: 0 };
    }
}

impl Instruction for NewArray {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.atype = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("stack is none");
        let count = stack.pop_int();
        if count < 0 {
            panic!("java.lang.NegativeArraySizeException")
        }
        let array_class = get_primitive_array_class(self.atype);
        let array_object = Class::new_array(&array_class, count as usize);
        stack.push_ref(Some(boxed(array_object)));
    }
}

fn get_primitive_array_class(atype: u8) -> Rc<RefCell<Class>> {
    let boot_loader = Jvm::boot_class_loader();
    let optional_class = match atype {
        AT_BOOLEAN => boot_loader.find_or_create("[Z"),
        AT_CHAR => boot_loader.find_or_create("[C"),
        AT_FLOAT => boot_loader.find_or_create("[F"),
        AT_DOUBLE => boot_loader.find_or_create("[D"),
        AT_BYTE => boot_loader.find_or_create("[B"),
        AT_SHORT => boot_loader.find_or_create("[S"),
        AT_INT => boot_loader.find_or_create("[I"),
        AT_LONG => boot_loader.find_or_create("[J"),
        _ => panic!("Invalid atype!"),
    };
    return optional_class.unwrap();
}
