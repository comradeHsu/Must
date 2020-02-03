use crate::runtime_data_area::frame::Frame;
use std::io;
use std::io::Write;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("java/io/FileOutputStream", "writeBytes",
                       "(IZ)", write_bytes);
}

pub fn write_bytes(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let b = vars.get_ref(1).unwrap();
    let off = vars.get_int(2);
    let len = vars.get_int(3);
    let borrow = (*b).borrow();
    let java_bytes = borrow.bytes();
    let bytes = byte_change(java_bytes);
    let mut out = io::stdout();
    out.write(bytes.as_slice());
}

fn byte_change(java_bytes:&Vec<i8>) -> Vec<u8> {
    let mut vec = Vec::with_capacity(java_bytes.len());
    for java_byte in java_bytes {
        vec.push(*java_byte as u8);
    }
    return vec;
}