use crate::class_loader::class_init_preparation::ClassPreparation;
use crate::class_loader::class_verifier::ClassVerifier;
use crate::oops::class::Class;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ClassLinker();

impl ClassLinker {
    pub fn link(class: &Class) {
        ClassVerifier::verify(class);
        ClassPreparation::prepare(class);
    }
}
