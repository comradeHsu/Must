use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;

pub fn init() {
    Registry::register("java/util/zip/ZipFile", "initIDs", "()V", init_ids);
}

pub fn init_ids(frame: &mut Frame) {}
