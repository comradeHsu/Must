use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::thread::Thread;
use std::rc::Rc;

pub fn invoke_method(frame:&mut Frame, method:Rc<Method>) {
    let thread = frame.thread();
    let mut new_frame = Thread::new_frame(thread.clone(),method.clone());
    let arg_slot_count = method.arg_slot_count();
    if arg_slot_count > 0 {
        let stack = frame.operand_stack().expect("stack is none");
        for size in 0..arg_slot_count {
            let slot = stack.pop_slot();
            new_frame.local_vars().expect("stack is none")
                .set_slot((arg_slot_count-1-size),slot);
        }
    }
    (*thread).borrow_mut().push_frame(new_frame);
}
