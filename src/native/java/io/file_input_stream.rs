use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;

pub fn init() {
    Registry::register("java/io/FileInputStream", "initIDs", "()V", init_ids);
}

pub fn init_ids(frame: &mut Frame) {}
