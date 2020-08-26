use crate::oops::class::Class;
use crate::runtime::frame::Frame;
use crate::runtime::thread::JavaThread;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init_class(class:Class) {
    class.set_initialized();
    schedule_clinit(&class);
    init_super_class(class);
}

fn schedule_clinit(class: &Class) {
    let clinit = class.get_clinit_method();
    if clinit.is_some() {
        let new_frame = Frame::new(clinit.unwrap());
        let thread = JavaThread::current();
        thread.push_frame(new_frame);
    }
}

fn init_super_class(class: Class) {
    if !class.is_interface() {
        let super_class = class.super_class();
        if super_class.is_none() {
            return;
        }
        let super_class = super_class.unwrap();
        if !super_class.initialized() {
            init_class(super_class);
        }
    }
}
