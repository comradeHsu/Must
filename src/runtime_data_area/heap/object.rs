use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::slot::Slot;

//#[derive(Debug, PartialEq)]
pub struct Object {
    class:Rc<Class>,
    fields:Option<Vec<Slot>>
}