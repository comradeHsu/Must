use std::path::Path;
use std::fs::File;
use std::fmt::{Display, Formatter, Error};
use core::fmt;
use podio::ReadPodExt;
use std::io::Read;

#[derive(Debug)]
struct DirEntry {

    pub abs_dir:String

}

impl DirEntry {
    pub fn new(class_path: &String) -> DirEntry {
        let path = Path::new(class_path);
        if !path.exists() {
            panic!("error")
        }
        return DirEntry{
            abs_dir: String::from(class_path)
        };
    }
}

impl super::class_path::Entry for DirEntry {
    fn read_class(self,class_name: String) -> (Vec<u8>,Box<dyn super::class_path::Entry>){
        let path = Path::new(&self.abs_dir);
        path.join(class_name);
        let result = File::open(path);
        let file = match result {
            Err(e) => panic!(e),
            Ok(f) => f
        };
        let mut bytes = Vec::new();
        file.bytes().for_each(|x| bytes.push(x.unwrap()));
        return (bytes,Box::new(self));
    }
}

impl ToString for DirEntry {
    fn to_string(&self) -> String {
        return String::from(&self.abs_dir);
    }
}