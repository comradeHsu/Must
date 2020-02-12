use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::thread::JavaThread;
use std::cell::RefCell;
use std::rc::Rc;

pub fn invoke_method(frame: &mut Frame, method: Rc<Method>) {
    let thread = frame.thread();
    let mut new_frame = JavaThread::new_frame(thread.clone(), method.clone());
    let arg_slot_count = method.arg_slot_count();
    if arg_slot_count > 0 {
        let stack = frame.operand_stack().expect("stack is none");
        for size in 0..arg_slot_count {
            let slot = stack.pop_slot();
            new_frame
                .local_vars()
                .expect("vars is none")
                .set_slot((arg_slot_count - 1 - size), slot);
        }
    }
    (*thread).borrow_mut().push_frame(new_frame);
    //    // hack!
    //    if method.is_native() {
    //        if method.name() == "registerNatives" {
    //            (*thread).borrow_mut().pop_frame();
    //        } else {
    //            panic!("native method:{} {} {}",(*method.class()).borrow().name(),method.name(),method.descriptor());
    //        }
    //    }
}

pub fn hack_invoke_method(thread_ptr: Rc<RefCell<JavaThread>>, method: Rc<Method>) {
    let mut thread = (*thread_ptr).borrow_mut();
    let frame_ptr = thread.current_frame();
    let mut frame = (*frame_ptr).borrow_mut();
    let mut new_frame = JavaThread::new_frame(thread_ptr.clone(), method.clone());
    let arg_slot_count = method.arg_slot_count();
    if arg_slot_count > 0 {
        let stack = frame.operand_stack().expect("stack is none");
        for size in 0..arg_slot_count {
            let slot = stack.pop_slot();
            new_frame
                .local_vars()
                .expect("vars is none")
                .set_slot((arg_slot_count - 1 - size), slot);
        }
    }
    thread.push_frame(new_frame);
    //    // hack!
    //    if method.is_native() {
    //        if method.name() == "registerNatives" {
    //            (*thread).borrow_mut().pop_frame();
    //        } else {
    //            panic!("native method:{} {} {}",(*method.class()).borrow().name(),method.name(),method.descriptor());
    //        }
    //    }
}
