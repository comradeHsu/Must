use crate::runtime_data_area::heap::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub enum ReturnValue {
    Void,
    Boolean(bool),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Char(char),
    Object(Option<Rc<RefCell<Object>>>),
}

impl ReturnValue {
    #[inline]
    pub fn object(&self) -> Option<Rc<RefCell<Object>>> {
        match &self {
            ReturnValue::Object(object) => object.clone(),
            _ => panic!("This return value isn't object"),
        }
    }
}
