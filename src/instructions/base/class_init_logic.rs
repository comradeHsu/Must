use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::thread::JavaThread;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init_class(thread: Rc<RefCell<JavaThread>>, class: Rc<RefCell<Class>>) {
    (*class).borrow_mut().set_initialized();
    schedule_clinit(thread.clone(), class.clone());
    init_super_class(thread, class);
}

fn schedule_clinit(thread: Rc<RefCell<JavaThread>>, class: Rc<RefCell<Class>>) {
    let clinit = Class::get_clinit_method(class);
    if clinit.is_some() {
        let new_frame = JavaThread::new_frame(thread.clone(), clinit.unwrap());
        (*thread).borrow_mut().push_frame(new_frame);
    }
}

fn init_super_class(thread: Rc<RefCell<JavaThread>>, class: Rc<RefCell<Class>>) {
    if !(*class).borrow().is_interface() {
        let super_class = (*class).borrow().super_class();
        if super_class.is_none() {
            return;
        }
        let super_class = super_class.unwrap();
        if !(*super_class).borrow().initialized() {
            init_class(thread, super_class);
        }
    }
}
