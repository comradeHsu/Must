use crate::instructions::base::instruction::Instruction;
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::class::Class;
use crate::utils::boxed;

const AT_BOOLEAN:u8 = 4;
const AT_CHAR:u8 = 5;
const AT_FLOAT:u8 = 6;
const AT_DOUBLE:u8 = 7;
const AT_BYTE:u8 = 8;
const AT_SHORT:u8 = 9;
const AT_INT:u8 = 10;
const AT_LONG:u8 = 11;

pub struct NewArray {
    atype:u8
}

impl NewArray {
    #[inline]
    pub fn new() -> NewArray {
        return NewArray{ atype: 0 };
    }
}

impl Instruction for NewArray {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.atype = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let class = frame.method().class();
        let stack = frame.operand_stack().expect("stack is none");
        let count = stack.pop_int();
        if count < 0 {
            panic!("java.lang.NegativeArraySizeException")
        }
        let class_loader = (*class).borrow().loader();
        let array_class = get_primitive_array_class(class_loader,self.atype);
        let array_object = Class::new_array(&array_class,count as usize);
        stack.push_ref(Some(boxed(array_object)));
    }
}

fn get_primitive_array_class(loader:Rc<RefCell<ClassLoader>>, atype:u8) -> Rc<RefCell<Class>> {
    match atype {
        AT_BOOLEAN => ClassLoader::load_class(loader,"[Z"),
        AT_CHAR => ClassLoader::load_class(loader,"[C"),
        AT_FLOAT => ClassLoader::load_class(loader,"[F"),
        AT_DOUBLE => ClassLoader::load_class(loader,"[D"),
        AT_BYTE => ClassLoader::load_class(loader,"[B"),
        AT_SHORT => ClassLoader::load_class(loader,"[S"),
        AT_INT => ClassLoader::load_class(loader,"[I"),
        AT_LONG => ClassLoader::load_class(loader,"[J"),
        _ => panic!("Invalid atype!")
    }
}