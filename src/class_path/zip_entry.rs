use std::path::Path;
use crate::class_path::class_path::Entry;
use std::fs::File;
use std::fmt::{Display, Formatter, Error};
use core::fmt;
use zip::read::ZipFile;
use podio::ReadPodExt;

#[derive(Debug)]
struct ZipEntry {

    abs_path:String

}

impl ZipEntry {
    pub fn new(class_path: &String) -> ZipEntry {
        let path = Path::new(class_path);
        if !path.exists() {
            panic!("error")
        }
        return ZipEntry{
            abs_path: String::from(class_path)
        };
    }
}

impl super::class_path::Entry for ZipEntry {
    fn read_class(self, class_name: String) -> (Vec<u8>, Box<dyn Entry>) {
        let path = Path::new(&self.abs_path);
        let zip_file = File::open(path).unwrap();
        let mut reader = std::io::Cursor::new(super::class_path::read_to_vec(zip_file));
        let mut zip = zip::ZipArchive::new(reader).unwrap();
        let mut bytes = Vec::new();
        for i in 0..zip.len() {
            let mut file:ZipFile = zip.by_index(i).unwrap();
            if file.name() == class_name.as_str() {
                bytes = file.read_exact(file.size() as usize).unwrap();
            }
        }
        return (bytes,Box::new(self));
    }
}

impl ToString for ZipEntry {
    fn to_string(&self) -> String {
        return String::from(&self.abs_path);
    }
}