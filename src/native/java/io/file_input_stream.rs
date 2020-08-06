use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::oops::object::{MetaData, Object};
use crate::utils::java_str_to_rust_str;
use std::fs::File;
use std::io::{stderr, stdin, stdout, Read, Seek, SeekFrom};
use std::path::Path;

pub fn init() {
    Registry::register("java/io/FileInputStream", "initIDs", "()V", init_ids);
    Registry::register(
        "java/io/FileInputStream",
        "open0",
        "(Ljava/lang/String;)V",
        open0,
    );
    Registry::register("java/io/FileInputStream", "close0", "()V", close0);
    Registry::register(
        "java/io/FileInputStream",
        "readBytes",
        "([BII)I",
        read_bytes,
    );
}

pub fn init_ids(frame: &mut Frame) {}

/// private native void open0(String name) throws FileNotFoundException;
/// (Ljava/lang/String;)V
pub fn open0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("LocalVars is none");
    let this = vars.get_ref(0).unwrap();
    let name = vars.get_ref(1);
    let rust_str = java_str_to_rust_str(name.unwrap());
    let path = Path::new(&rust_str);
    if !path.exists() {
        // throws FileNotFoundException;
        println!("path:{}",path.to_str().unwrap());
        panic!("FileNotFoundException");
    }
    let file = File::open(path);
    (*this).borrow_mut().set_file(file.unwrap());
}

/// private native int readBytes(byte b[], int off, int len) throws IOException;
/// ([BII)I
pub fn read_bytes(frame: &mut Frame) {
    let vars = frame.local_vars().expect("LocalVars is none");
    let this = vars.get_ref(0).unwrap();
    let byte_array = vars.get_ref(1).unwrap();
    let offset = vars.get_int(2) as usize;
    let length = vars.get_int(3);

    if length <= 0 {
        frame.operand_stack().expect("stack is none").push_int(0);
        return;
    }

    let length = length as usize;

    let mut bytes = vec![0u8; length];
    let file = (*this).borrow().file();
    let rs = file.borrow_mut().read(bytes.as_mut_slice());
    let mut size = -1;
    let read_size = rs.expect("the file seek has error");
    if read_size != 0 {
        let mut mut_byte_array = (*byte_array).borrow_mut();
        let mut_array = mut_byte_array.mut_bytes();
        for i in 0..read_size {
            mut_array[offset + i] = bytes[i] as i8;
        }
        size = read_size as i32;
    }
    frame.operand_stack().expect("stack is none").push_int(size);
}

fn unique_path(path: String) -> String {
    let paths: Vec<&str> = path.split('/').collect();
    let mut path_str = String::new();
    for p in paths {
        if !path_str.contains(p) {
            path_str.push_str(p);
            path_str.push('/');
        }
    }
    assert_ne!(0, path_str.len());
    path_str.pop();
    return path_str;
}

/// private native void close0() throws IOException;
/// ()V
pub fn close0(frame: &mut Frame) {
    //    let vars = frame.local_vars().expect("LocalVars is none");
    //    let name = vars.get_ref(1);
    //    let rust_str = java_str_to_rust_str(name.unwrap());
    //    let path = Path::new(&rust_str);
    //    if !path.exists() {
    //        // throws FileNotFoundException;
    //    }
}
