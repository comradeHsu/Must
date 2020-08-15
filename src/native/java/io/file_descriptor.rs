use crate::native::registry::Registry;
use crate::runtime::frame::Frame;

pub fn init() {
    Registry::register("java/io/FileDescriptor", "initIDs", "()V", init_ids);
    Registry::register("java/io/FileDescriptor", "set", "(I)J", set);
}

pub fn init_ids(frame: &Frame) {}

pub fn set(frame: &Frame) {
    let fd = frame.get_int(0);
    frame.push_long(fd as i64);
}
