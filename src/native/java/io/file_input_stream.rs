use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::oops::object::{MetaData, Object};
use crate::utils::java_str_to_rust_str;
use std::fs::File;
use std::io::{stderr, stdin, stdout, Read, Seek, SeekFrom};
use std::path::Path;
use crate::invoke_support::throw_exception;
use std::error::Error;

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

pub fn init_ids(frame: &Frame) {}

/// private native void open0(String name) throws FileNotFoundException;
/// (Ljava/lang/String;)V
pub fn open0(frame: &Frame) {
    let (this,name) = frame.local_vars_get(|vars|{
        let this = vars.get_ref(0).unwrap();
        let name = vars.get_ref(1);
        return (this,name)
    });
    let rust_str = java_str_to_rust_str(name.unwrap());
    let path = Path::new(&rust_str);
    let file = File::open(path);
    if file.is_err() {
        let mut msg = String::new();
        let error = file.err().unwrap();
        msg.push_str(path.to_str().unwrap());
        msg.push(' ');
        msg.push_str(error.to_string().as_str());
        throw_exception(frame,"java/io/FileNotFoundException",Some(msg.as_str()));
    } else {
        (*this).borrow_mut().set_file(file.unwrap());
    }
}

/// private native int readBytes(byte b[], int off, int len) throws IOException;
/// ([BII)I
pub fn read_bytes(frame: &Frame) {
    let (this,byte_array,offset,length) = frame.local_vars_get(|vars|{
        let this = vars.get_ref(0).unwrap();
        let byte_array = vars.get_ref(1).unwrap();
        let offset = vars.get_int(2) as usize;
        let length = vars.get_int(3);
        return (this,byte_array,offset,length)
    });

    if length <= 0 {
        frame.push_int(0);
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
    frame.push_int(size);
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
pub fn close0(frame: &Frame) {
    //    let vars = frame.local_vars().expect("LocalVars is none");
    //    let name = vars.get_ref(1);
    //    let rust_str = java_str_to_rust_str(name.unwrap());
    //    let path = Path::new(&rust_str);
    //    if !path.exists() {
    //        // throws FileNotFoundException;
    //    }
}
