use crate::runtime::frame::Frame;
use crate::runtime::thread::JavaThread;

pub fn branch(frame: &Frame, offset: i32) {
    let thread = JavaThread::current();
    let pc = thread.get_pc();
    let next_pc = pc + offset;
    frame.set_next_pc(next_pc);
}
