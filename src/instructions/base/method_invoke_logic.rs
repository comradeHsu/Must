use crate::oops::method::Method;
use crate::runtime::frame::Frame;
use crate::runtime::thread::JavaThread;

use std::rc::Rc;

pub fn invoke_method(frame: &Frame, method: Method) {
    let thread = JavaThread::current();
    let new_frame = Frame::new(method.clone());
    let arg_slot_count = method.arg_slot_count();
    if arg_slot_count > 0 {
        for size in 0..arg_slot_count {
            let slot = frame.pop_slot();
            new_frame.set_slot(arg_slot_count - 1 - size, slot);
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
