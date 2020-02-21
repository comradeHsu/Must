use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;
use crate::class_loader::class_init_preparation::ClassPreparation;
use crate::class_loader::class_verifier::ClassVerifier;

pub struct ClassLinker();

impl ClassLinker {
    pub fn link(class: &Rc<RefCell<Class>>) {
        ClassVerifier::verify(class);
        ClassPreparation::prepare(class);
    }
}