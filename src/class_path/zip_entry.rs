use std::path::Path;
use crate::class_path::class_path::{Entry, FindClassError};
use std::fs::File;
use zip::read::ZipFile;
use podio::ReadPodExt;

#[derive(Debug)]
pub struct ZipEntry {

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

impl Entry for ZipEntry {
    fn read_class(&self, class_name: &str) -> Result<(Vec<u8>,Box<dyn Entry>),FindClassError> {
        let path = Path::new(&self.abs_path);
        let zip_file = File::open(path).unwrap();
        let mut reader = std::io::Cursor::new(super::class_path::read_to_vec(zip_file));
        let mut zip = zip::ZipArchive::new(reader).unwrap();
        let mut bytes = Vec::new();
        for i in 0..zip.len() {
            let mut file:ZipFile = zip.by_index(i).unwrap();
            println!("zip_file_name:{}\n",file.name());
            if file.name() == class_name {
                println!("file_name:{}\n",class_name);
                bytes = file.read_exact(file.size() as usize).unwrap();
                return Ok((bytes,Box::new(ZipEntry{
                    abs_path: self.abs_path.to_string()
                })));
            }
        }
        return Err(FindClassError("don't find class".to_string()));
    }

    fn to_string(&self) -> String {
        return String::from(&self.abs_path);
    }
}