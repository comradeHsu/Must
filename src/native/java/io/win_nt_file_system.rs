use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::utils::java_str_to_rust_str;
use std::path::Path;
use std::fs::File;
use chrono::Local;
use std::time::UNIX_EPOCH;

pub fn init() {
    Registry::register("java/io/WinNTFileSystem", "initIDs", "()V", init_ids);
    Registry::register(
        "java/io/WinNTFileSystem",
        "canonicalize0",
        "(Ljava/lang/String;)Ljava/lang/String;",
        canonicalize0,
    );
    Registry::register(
        "java/io/WinNTFileSystem",
        "getBooleanAttributes",
        "(Ljava/io/File;)I",
        get_boolean_attributes,
    );
    Registry::register(
        "java/io/WinNTFileSystem",
        "getLastModifiedTime",
        "(Ljava/io/File;)J",
        get_last_modified_time,
    );
}

/// java/io/WinNTFileSystem.initIDs()V
pub fn init_ids(_frame: &mut Frame) {}

/// private native String canonicalize0(String path) throws IOException;
/// (Ljava/lang/String;)Ljava/lang/String;
pub fn canonicalize0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let java_path = vars.get_ref(1);
    let mut path = java_str_to_rust_str(java_path.unwrap());
    let file_path = Path::new(&path).canonicalize();
    if file_path.is_ok() {
        path = file_path.unwrap().to_str().unwrap().to_string();
    }
    let java_string = StringPool::java_string(path);
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(Some(java_string));
}

/// @Native public static final int BA_EXISTS    = 0x01;
/// @Native public static final int BA_REGULAR   = 0x02;
/// @Native public static final int BA_DIRECTORY = 0x04;
/// @Native public static final int BA_HIDDEN    = 0x08;
/// public native int getBooleanAttributes(File f);
/// (Ljava/io/File;)I
pub fn get_boolean_attributes(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let java_file = vars.get_ref(1).unwrap();
    let java_path = (*java_file)
        .borrow()
        .get_ref_var("path", "Ljava/lang/String;");
    let native_path = java_str_to_rust_str(java_path.unwrap());
    let path = Path::new(&native_path);
    let mut attribute = 0;
    if path.exists() {
        attribute |= 0x01;
    }
    if path.is_file() {
        attribute |= 0x02;
    }
    if path.is_dir() {
        attribute |= 0x04;
    }
    if is_hidden(native_path.as_str()) {
        attribute |= 0x08;
    }
    frame
        .operand_stack()
        .expect("stack is none")
        .push_int(attribute);
}

fn is_hidden(filename: &str) -> bool {
    if std::env::consts::OS != "windows" {
        // unix/linux file or directory that starts with . is hidden
        if filename.starts_with('.') {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
    return false;
}

/// public native long getLastModifiedTime(File f);
/// (Ljava/io/File;)J
pub fn get_last_modified_time(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let java_file = vars.get_ref(1).expect("java.lang.NullPointerException");
    let java_path = (*java_file).borrow().get_ref_var("path", "Ljava/lang/String;");
    let rust_path = java_str_to_rust_str(java_path.unwrap());
    let path = Path::new(&rust_path);
    let file = File::open(path).expect("can not find file");
    let meta_data = file.metadata().unwrap();
    let modify_time = meta_data.modified().unwrap();
    let time = modify_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(time.as_millis() as i64);
}