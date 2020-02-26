use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::utils::java_str_to_rust_str;
use std::fs::File;
use zip::ZipArchive;

pub fn init() {
    Registry::register("java/util/zip/ZipFile", "initIDs", "()V", init_ids);
    Registry::register(
        "java/util/zip/ZipFile",
        "open",
        "(Ljava/lang/String;IJZ)J",
        open,
    );
}

pub fn init_ids(_frame: &mut Frame) {}

///private static native long open(String name, int mode, long lastModified,
///                                    boolean usemmap) throws IOException;
/// (Ljava/lang/String;IJZ)J
pub fn open(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let java_name = vars.get_ref(0);
    let name = java_str_to_rust_str(java_name.unwrap());
    let zip_file = File::open(&name).unwrap();
    let mut zip = zip::ZipArchive::new(zip_file).unwrap();
    let point = &zip as *const ZipArchive<File>;
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(point as usize as i64);
}
