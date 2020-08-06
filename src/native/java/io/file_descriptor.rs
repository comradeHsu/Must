use crate::native::registry::Registry;
use crate::runtime::frame::Frame;

pub fn init() {
    Registry::register("java/io/FileDescriptor", "initIDs", "()V", init_ids);
    Registry::register("java/io/FileDescriptor", "set", "(I)J", set);
}

pub fn init_ids(frame: &mut Frame) {}

pub fn set(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let fd = vars.get_int(0);
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(fd as i64);
}
