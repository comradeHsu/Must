use crate::class_loader::class_init_preparation::ClassPreparation;
use crate::class_loader::class_verifier::ClassVerifier;
use crate::runtime_data_area::heap::class::Class;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ClassLinker();

impl ClassLinker {
    pub fn link(class: &Rc<RefCell<Class>>) {
        ClassVerifier::verify(class);
        ClassPreparation::prepare(class);
    }
}
