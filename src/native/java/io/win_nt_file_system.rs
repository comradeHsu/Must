use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("java/io/WinNTFileSystem", "initIDs",
                       "()V", init_ids);
}

/// java/io/WinNTFileSystem.initIDs()V
pub fn init_ids(frame:&mut Frame) {

}