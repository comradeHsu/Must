use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;

pub struct ClassVerifier();

impl ClassVerifier {
    /// waiting for implementation
    pub fn verify(class: &Rc<RefCell<Class>>) {}
}