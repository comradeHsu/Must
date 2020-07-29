use crate::jni::JObject;
use crate::jvm::Jvm;
use crate::native::java::util::zip_file::zip_file_cache::ZipFile;
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::array_object::ArrayObject;
use crate::runtime_data_area::heap::object::DataType::Bytes;
use crate::utils::{boxed, java_str_to_rust_str, jbytes_to_u8s};
use chrono::{DateTime, Utc};
use podio::ReadPodExt;
use rc_zip::{Archive, ReadZip, StoredEntry};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::time::SystemTime;
use zip::ZipArchive;

pub fn init() {
    Registry::register("java/util/zip/ZipFile", "initIDs", "()V", init_ids);
    Registry::register(
        "java/util/zip/ZipFile",
        "open",
        "(Ljava/lang/String;IJZ)J",
        open,
    );
    Registry::register("java/util/zip/ZipFile", "getTotal", "(J)I", get_total);
    Registry::register(
        "java/util/zip/ZipFile",
        "startsWithLOC",
        "(J)Z",
        starts_with_loc,
    );
    Registry::register("java/util/zip/ZipFile", "getEntry", "(J[BZ)J", get_entry);
    Registry::register(
        "java/util/zip/ZipFile",
        "getEntryFlag",
        "(J)I",
        get_entry_flag,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "getEntryBytes",
        "(JI)[B",
        get_entry_bytes,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "getEntryTime",
        "(J)J",
        get_entry_time,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "getEntryCrc",
        "(J)J",
        get_entry_crc,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "getEntrySize",
        "(J)J",
        get_entry_size,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "getEntryCSize",
        "(J)J",
        get_entry_csize,
    );
    Registry::register(
        "java/util/zip/ZipFile",
        "getEntryMethod",
        "(J)I",
        get_entry_method,
    );
    Registry::register("java/util/zip/ZipFile", "freeEntry", "(JJ)V", free_entry);
    Registry::register("java/util/zip/ZipFile", "read", "(JJJ[BII)I", read);
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
    let zip = zip_file.read_zip().expect("This File not ZIP");
    let point = &zip as *const Archive as usize;
    zip_file_cache::insert(point, zip_file_cache::ZipFile::new(metadata, zip, zip_file));
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(point as i64);
}

/// private static native int getTotal(long jzfile);
/// (J)I
pub fn get_total(frame: &mut Frame) {
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
pub fn starts_with_loc(frame: &mut Frame) {
    frame
        .operand_stack()
        .expect("stack is none")
        .push_boolean(true);
}

/// private static native long getEntry(long jzfile, byte[] name,
///                                        boolean addSlash);
/// (J[BZ)J
pub fn get_entry(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0) as usize;
    let name_bytes = vars.get_ref(2).unwrap();
    let add_slash = vars.get_boolean(3);
    let bytes = jbytes_to_u8s(name_bytes);
    let name = String::from_utf8(bytes).unwrap();
    let zip_file = zip_file_cache::get(address).expect("the file is not open");
    let index = zip_file.indexes.get(name.as_str());
    if index.is_none() {
        println!("The file is not exist:{}", name);
        frame.operand_stack().expect("stack is none").push_long(0);
    } else {
        println!("The file name:{}", name);
        let index = index.unwrap();
        let entry = zip_file.file.entries().get(*index).unwrap();
        let address = entry as *const StoredEntry as usize;
        frame
            .operand_stack()
            .expect("stack is none")
            .push_long(address as i64);
    }
}

/// private static native int getEntryFlag(long jzentry);
/// (J)I
pub fn get_entry_flag(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0);
    let pointer = address as *const StoredEntry;
    unsafe {
        let entry = &*pointer;
        frame
            .operand_stack()
            .expect("stack is none")
            .push_int(entry.flags as i32);
    }
}

/// private static native byte[] getEntryBytes(long jzentry, int type);
/// (JI)[B
pub fn get_entry_bytes(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0);
    let param_type = vars.get_int(2);
    let pointer = address as *const StoredEntry;
    unsafe {
        let entry = &*pointer;
        let name = match param_type {
            0 => entry.name(),
            1 => "entry.",
            2 => entry.comment().unwrap_or(""),
            _ => panic!("Illegal parameter value"),
        };
        let bytes: Vec<i8> = name.bytes().map(|x| x as i8).collect();
        let boot = Jvm::boot_class_loader();
        let object = ArrayObject::from_data(boot.find_or_create("[B").unwrap(), Bytes(bytes));
        frame
            .operand_stack()
            .expect("stack is none")
            .push_ref(Some(boxed(object)));
    }
}

/// private static native long getEntryTime(long jzentry);
/// (J)J
pub fn get_entry_time(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0);
    let pointer = address as *const StoredEntry;
    unsafe {
        let entry = &*pointer;
        let zero: DateTime<Utc> = DateTime::from(SystemTime::UNIX_EPOCH);
        let time: &DateTime<Utc> = entry.created().unwrap_or(&zero);
        frame
            .operand_stack()
            .expect("stack is none")
            .push_long(time.timestamp_millis());
    }
}

/// private static native long getEntryCrc(long jzentry);
/// (J)J
pub fn get_entry_crc(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0);
    let pointer = address as *const StoredEntry;
    unsafe {
        let entry = &*pointer;
        let crc = entry.crc32 as i64;
        frame.operand_stack().expect("stack is none").push_long(crc);
    }
}

/// private static native long getEntrySize(long jzentry);
/// (J)J
pub fn get_entry_size(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0);
    let pointer = address as *const StoredEntry;
    unsafe {
        let entry = &*pointer;
        let size = entry.uncompressed_size as i64;
        frame
            .operand_stack()
            .expect("stack is none")
            .push_long(size);
    }
}

/// private static native long getEntryCSize(long jzentry);
/// (J)J
pub fn get_entry_csize(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0);
    let pointer = address as *const StoredEntry;
    unsafe {
        let entry = &*pointer;
        let size = entry.compressed_size as i64;
        frame
            .operand_stack()
            .expect("stack is none")
            .push_long(size);
    }
}

/// private static native long getEntryMethod(long jzentry);
/// (J)I
pub fn get_entry_method(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let address = vars.get_long(0);
    let pointer = address as *const StoredEntry;
    unsafe {
        let entry = &*pointer;
        let method = entry.method();
        use rc_zip::Method::*;
        let value = match method {
            Store => 0,
            Deflate => 8,
            Bzip2 => 12,
            Lzma => 14,
            Unsupported(v) => v as i32,
        };
        frame
            .operand_stack()
            .expect("stack is none")
            .push_int(value);
    }
}

/// private static native void freeEntry(long jzentry);
/// (JJ)V
pub fn free_entry(_frame: &mut Frame) {}

/// private static native int read(long jzfile, long jzentry,
///                                   long pos, byte[] b, int off, int len);
/// (JJJ[BII)I
pub fn read(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let file_address = vars.get_long(0) as usize;
    let entry_address = vars.get_long(2);
    let position = vars.get_long(4);
    let buf = vars.get_ref(6);
    let offset = vars.get_int(7) as usize;
    let mut len = vars.get_int(8);

    const BUFF_SIZE: i32 = 8192;
    if len > BUFF_SIZE {
        len = BUFF_SIZE;
    }
    let mut buff = [0u8; 8192];
    let zip_file = zip_file_cache::get(file_address).expect("The file not exist");
    let entry = unsafe {
        let pointer = entry_address as *const StoredEntry;
        &*pointer
    };
    let length = zip_read(zip_file, entry, position, &mut buff, len as isize);
    println!(
        "entry:{}, length:{},data:{:?}",
        entry.name(),
        length,
        &buff[0..100]
    );
    if length != -1 {
        println!("offset:{},position:{},len:{}", offset, position, len);
        set_byte_array_region(buf, offset, length as usize, &buff);
    }

    frame
        .operand_stack()
        .expect("stack is none")
        .push_int(length as i32);
}

fn zip_read(
    zip: &ZipFile,
    entry: &StoredEntry,
    position: i64,
    buf: &mut [u8],
    mut len: isize,
) -> isize {
    let entry_size = match entry.compressed_size != 0 {
        true => entry.compressed_size,
        false => entry.uncompressed_size,
    } as i64;

    let mut read_len = 0;
    /* Check specified position */
    if position < 0 || position > (entry_size - 1) {
        //        zip->msg = "ZIP_Read: specified offset out of range";
        return -1;
    }
    if len < 0 {
        return 0;
    }
    if len > (entry_size - position) as isize {
        len = (entry_size - position) as isize;
    }
    if read_fully(zip, entry, buf, len as usize) == -1 {
        //zip->msg = "ZIP_Read: error reading zip file";
        return -1;
    }
    return len;
}

fn read_fully(zip: &ZipFile, entry: &StoredEntry, mut buf: &mut [u8], mut len: usize) -> isize {
    while len > 0 {
        let limit = (1 << 31) - 1;
        let count = match len < limit {
            true => len,
            false => limit,
        };
        let mut reader =
            entry.reader(|offset| positioned_io::Cursor::new_pos(&zip.meta_file, offset));
        let read_rs = reader.read(buf);

        match read_rs {
            Ok(size) => {
                if size > 0 {
                    buf = &mut buf[size..];
                    len -= size;
                }
            }
            Err(error) => match error.kind() {
                ErrorKind::Interrupted => continue,
                _ => return -1,
            },
        }
    }
    return 0;
}

fn set_byte_array_region(bytes: JObject, offset: usize, len: usize, buff: &[u8]) {
    let bytes = bytes.unwrap();
    let mut borrow = (*bytes).borrow_mut();
    let data = borrow.mut_bytes();
    for index in 0..len {
        data[index + offset] = buff[index] as i8;
    }
}

pub mod zip_file_cache {
    use rc_zip::Archive;
    use std::collections::HashMap;
    use std::fs::{File, Metadata};
    use std::io::Read;
    use zip::ZipArchive;

    pub struct ZipFile {
        pub metadata: Metadata,
        pub file: Archive,
        pub meta_file: File,
        pub indexes: HashMap<String, usize>,
    }

    impl ZipFile {
        pub fn new(metadata: Metadata, file: Archive, meta_file: File) -> ZipFile {
            let mut indexes = HashMap::new();
            let entries = file.entries();
            for index in 0..entries.len() {
                let entry = entries.get(index).unwrap();
                indexes.insert(entry.name().to_string(), index);
            }
            return ZipFile {
                metadata,
                file,
                meta_file,
                indexes,
            };
        }
    }

    static mut ZIP_FILE_CACHE: Option<HashMap<usize, ZipFile>> = None;

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
