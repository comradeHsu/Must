use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::object::{Object, MetaData};
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
    Registry::register(
        "java/io/FileInputStream",
        "close0",
        "()V",
        close0,
    );
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
    }
    (*this).borrow_mut().set_file_offset(0);
}

/// private native int readBytes(byte b[], int off, int len) throws IOException;
/// ([BII)I
pub fn read_bytes(frame: &mut Frame) {
    let vars = frame.local_vars().expect("LocalVars is none");
    let this = vars.get_ref(0).unwrap();
    let byte_array = vars.get_ref(1).unwrap();
    let offset = vars.get_int(2) as usize;
    let length = vars.get_int(3) as usize;

    let file_descriptor = (*this)
        .borrow()
        .get_ref_var("fd", "Ljava/io/FileDescriptor;");
    let path = (*this).borrow().get_ref_var("path", "Ljava/lang/String;");
    let mut file: Option<File> = None;
    if path.is_some() {
        let native_path = java_str_to_rust_str(path.unwrap());
        let path = Path::new(&native_path);
        println!("path:{}",native_path);
        file = Some(File::open(path).unwrap());
    } else if file_descriptor.is_some() {
        let fd = (*file_descriptor.unwrap()).borrow().get_int_var("fd", "I");
        //        file = match fd {
        //            0 => stdin(),
        //            1 => stdout(),
        //            2 => stderr(),
        //            _ => panic!()
        //        }
    }
    if file.is_none() {
        panic!("java/io/IOException File cannot open");
    }
    let mut bytes = vec![0u8;length];
    let mut file = file.unwrap();
    let file_offset = (*this).borrow().file_offset() + offset as u64;
    file.seek(SeekFrom::Start(file_offset))
        .expect("seek has error");
    let rs = file.read(bytes.as_mut_slice());
    let mut size = -1;
    let read_size = rs.expect("the file seek has error");
    if read_size != 0 {
        let mut mut_byte_array = (*byte_array).borrow_mut();
        let mut_array = mut_byte_array.mut_bytes();
        for i in 0..length {
            mut_array[offset + i] = bytes[i] as i8;
        }
        size = read_size as i32;
        (*this).borrow_mut().set_file_offset(file_offset + read_size as u64);
    }
    frame.operand_stack().expect("stack is none").push_int(size);
}

fn unique_path(path:String) -> String {
    let paths:Vec<&str> = path.split('/').collect();
    let mut path_str = String::new();
    for p in paths {
        if !path_str.contains(p) {
            path_str.push_str(p);
            path_str.push('/');
        }
    }
    assert_ne!(0,path_str.len());
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
