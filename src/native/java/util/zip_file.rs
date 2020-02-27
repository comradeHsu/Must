use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::utils::{java_str_to_rust_str, jbytes_to_u8s};
use std::fs::File;
use zip::ZipArchive;
use zip::read::ZipFile;

pub fn init() {
    Registry::register("java/util/zip/ZipFile", "initIDs", "()V", init_ids);
    Registry::register(
        "java/util/zip/ZipFile",
        "open",
        "(Ljava/lang/String;IJZ)J",
        open,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "getTotal",
        "(J)I",
        get_total,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "startsWithLOC",
        "(J)Z",
        starts_with_loc,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "getEntry",
        "(J[BZ)J",
        get_entry,
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
    let metadata = zip_file.metadata().expect("not metadata");
    let mut zip = zip::ZipArchive::new(zip_file).unwrap();
    let point = &zip as *const ZipArchive<File> as usize;
    zip_file_cache::insert(point,zip_file_cache::ZipFile::new(metadata,zip));
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(point as i64);
}

/// private static native int getTotal(long jzfile);
/// (J)I
pub fn get_total(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0) as usize;
    let file = zip_file_cache::get(address).expect("the file is not open");
    let total = file.metadata.len();
    frame
        .operand_stack()
        .expect("stack is none")
        .push_int(total as i32);
}

/// private static native boolean startsWithLOC(long jzfile);
/// (J)Z
pub fn starts_with_loc(frame:&mut Frame) {
    frame
        .operand_stack()
        .expect("stack is none")
        .push_boolean(true);
}

/// private static native long getEntry(long jzfile, byte[] name,
///                                        boolean addSlash);
/// (J[BZ)J
pub fn get_entry(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0) as usize;
    let name_bytes = vars.get_ref(2).unwrap();
    let add_slash = vars.get_boolean(3);
    let bytes = jbytes_to_u8s(name_bytes);
    let name = String::from_utf8(bytes).unwrap();
    let zip_file = zip_file_cache::get_mut(address).expect("the file is not open");
//    for i in 0..zip_file.file.len() {
//        let file = zip_file.file.by_index(i).unwrap();
//        println!("zip file:{}",file.name());
//    }
    let file = zip_file.file.by_name(name.as_str());
    if file.is_err() {
        println!("The file is not exist");
        frame
            .operand_stack()
            .expect("stack is none")
            .push_long(0);
        return;
    }
    let file = file.unwrap();
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(0);
}

mod zip_file_cache {
    use std::collections::HashMap;
    use zip::ZipArchive;
    use std::fs::{File, Metadata};

    pub struct ZipFile {
        pub metadata:Metadata,
        pub file:ZipArchive<File>
    }

    impl ZipFile {
        pub fn new(metadata:Metadata, file:ZipArchive<File>) -> ZipFile {
            return ZipFile{ metadata, file };
        }
    }

    static mut ZIP_FILE_CACHE:Option<HashMap<usize,ZipFile>> = None;

    fn instance() -> &'static mut HashMap<usize, ZipFile> {
        unsafe {
            if ZIP_FILE_CACHE.is_none() {
                ZIP_FILE_CACHE = Some(HashMap::new());
            }
            return ZIP_FILE_CACHE.as_mut().unwrap();
        }
    }

    pub fn insert(key: usize, file: ZipFile) {
        instance().insert(key, file);
    }

    pub fn get(key: usize) -> Option<&'static ZipFile> {
        let size = instance().get(&key);
        return size;
    }

    pub fn get_mut(key: usize) -> Option<&'static mut ZipFile> {
        let size = instance().get_mut(&key);
        return size;
    }

    pub fn delete(key: usize) {
        instance().remove(&key);
    }
}