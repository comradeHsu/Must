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
        let mut zip = zip::ZipArchive::new(zip_file).unwrap();
        let mut bytes = Vec::new();
        for i in 0..zip.len() {
            let mut file:ZipFile = zip.by_index(i).unwrap();
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

#[cfg(test)]
mod test {
    use std::{fs, io};
    use crate::class_path::zip_entry::ZipEntry;
    use podio::ReadPodExt;
    use std::io::Read;

    #[test]
    fn test_zip() {
        let fname = std::path::Path::new("D:\\test\\com\\compile\\Main.zip");
        let file = fs::File::open(fname).unwrap();

        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = file.sanitized_name();

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {} comment: {}", i, comment);
                }
            }

            if (&*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
                fs::create_dir_all(&outpath).unwrap();
            } else {
                println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }

    #[test]
    fn test_object() {
        let zip_object = get_bytes();
        let fname = std::path::Path::new("D:/java8\\jdk-class\\java\\lang\\Object.class");
        let file = fs::File::open(fname).unwrap();
        let mut object = Vec::new();
        file.bytes().for_each(|a| {object.push(a.unwrap())});
        println!("zip_o:{},o:{}", zip_object.len(),object.len());
        for i in 0..zip_object.len() {
            println!("zip_o:{},o:{}", zip_object[i],object[i]);
        }
    }

    pub fn get_bytes() -> Vec<u8> {
        let fname = std::path::Path::new("D:/java8\\JDK\\jre\\lib\\rt.jar");
        let file = fs::File::open(fname).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut bytes = Vec::new();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let size = file.size() as usize;
            if file.name() == "java/lang/Object.class" {
                bytes = podio::ReadPodExt::read_exact(&mut file,size).unwrap();
                return bytes;
            }
        }
        return bytes;
    }
}
