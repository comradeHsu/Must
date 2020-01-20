use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::thread::Thread;

pub fn invoke_method(frame:&Frame, method:&Method) {
    let thread = frame.thread();
    let new_frame = Thread::new_frame(thread);
}
