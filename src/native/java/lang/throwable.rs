use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::thread::Thread;

pub fn init() {
    Registry::register("java/lang/Throwable", "fillInStackTrace",
                       "(I)Ljava/lang/Throwable;", fill_in_stack_trace);
}

pub fn fill_in_stack_trace(frame:&mut Frame) {

}

struct StackTraceElement {
    file_name:String,
    class_name:String,
    method_name:String,
    line_number:i32
}

impl StackTraceElement {
    fn create_stack_trace_elements(object:Rc<RefCell<Object>>, thread:Rc<RefCell<Thread>>) {

    }
}