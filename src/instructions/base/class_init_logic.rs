use crate::oops::class::Class;
use crate::runtime::frame::Frame;
use crate::runtime::thread::JavaThread;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init_class(class: Rc<RefCell<Class>>) {
    (*class).borrow_mut().set_initialized();
    schedule_clinit(class.clone());
    init_super_class(class);
}

fn schedule_clinit(class: Rc<RefCell<Class>>) {
    let clinit = Class::get_clinit_method(class);
    if clinit.is_some() {
        let new_frame = Frame::new(clinit.unwrap());
        let thread = JavaThread::current();
        thread.push_frame(new_frame);
    }
}

fn init_super_class(class: Rc<RefCell<Class>>) {
    if !(*class).borrow().is_interface() {
        let super_class = (*class).borrow().super_class();
        if super_class.is_none() {
            return;
        }
        let super_class = super_class.unwrap();
        if !(*super_class).borrow().initialized() {
            init_class(super_class);
        }
    }
}
