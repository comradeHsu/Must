use crate::runtime_data_area::frame::Frame;

pub fn branch(frame:&mut Frame,offset:i32) {
    let thread = frame.thread();
    let pc = (*thread).borrow().get_pc();
    let next_pc = pc + offset;
    frame.set_next_pc(next_pc);
}