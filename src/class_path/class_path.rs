use super::composite_entry;
use super::dir_entry::DirEntry;
use crate::class_path::composite_entry::new_wildcard_entry;
use crate::class_path::zip_entry::ZipEntry;
use crate::cmd::Cmd;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{env, fmt};

pub static PATH_LIST_SEPARATOR: char = ';';

pub trait Entry {
    fn read_class(&self, class_name: &str) -> Result<(Vec<u8>, Box<dyn Entry>), FindClassError>;

    fn to_string(&self) -> String;
}

pub fn new_entry(path: &String) -> Box<dyn Entry> {
    if path.contains(PATH_LIST_SEPARATOR) {
        return Box::new(composite_entry::new(path));
    } else if path.ends_with('*') {
        println!("wildcard");
        return Box::new(composite_entry::new_wildcard_entry(path));
    } else if path.ends_with(".zip")
        || path.ends_with(".ZIP")
        || path.ends_with(".jar")
        || path.ends_with(".JAR")
    {
        return Box::new(ZipEntry::new(path));
    }
    return Box::new(DirEntry::new(path));
}

pub fn read_to_vec(file: File) -> Vec<u8> {
    let mut zip_bytes = Vec::new();
    file.bytes().for_each(|x| zip_bytes.push(x.unwrap()));
    return zip_bytes;
}

#[derive(Debug)]
pub struct FindClassError(pub String);

impl Error for FindClassError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl Display for FindClassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct ClassPath {
    boot_class_path: Option<Box<dyn Entry>>,
}

impl ClassPath {
    pub fn new() -> ClassPath {
        return ClassPath {
            boot_class_path: None,
        };
    }

    pub fn parse(jre_option: &String, cp_option: &Vec<String>) -> ClassPath {
        let mut class_path = ClassPath {
            boot_class_path: None,
        };
        class_path.boot_and_ext_class_path(jre_option);
        //        class_path.user_class_path(cp_option);
        return class_path;
    }

    fn boot_and_ext_class_path(&mut self, jre_option: &String) {
        let jre_dir = ClassPath::get_jre_dir(jre_option);
        let jre_lib_path = jre_dir.clone() + "/lib" + "/*";
        self.boot_class_path = Some(Box::new(new_wildcard_entry(&jre_lib_path)));
        println!(
            "boot:{}\n",
            self.boot_class_path.as_ref().unwrap().to_string()
        );
    }

    fn get_jre_dir(jre_option: &String) -> String {
        let path = Path::new(jre_option);
        if jre_option.is_empty() && path.exists() {
            return String::from(jre_option);
        }
        if Path::new("./jre").exists() {
            return "./jre".to_string();
        }
        let java_home =
            env::vars_os().find(|(key, _value)| return key == &OsString::from("JAVA_HOME"));
        if java_home.is_some() {
            let (_key, value) = java_home.unwrap();
            return value.to_str().unwrap().to_string() + "/jre";
        }
        panic!("Can not find jre folder!")
    }

    fn user_class_path(&mut self, cp_option: &Vec<String>) {
        let mut class_paths = Vec::with_capacity(cp_option.len());
        for cp in cp_option {
            if cp == "" {
                let entry = new_entry(&".".to_string());
                class_paths.push(entry);
            } else {
                let entry = new_entry(cp);
                class_paths.push(entry);
            }
        }
        //        self.user_class_path = Some(class_paths);
    }

    pub fn handle_jar(&mut self, cmd: &mut Cmd) {
        if let Some(jar) = cmd.exec_jar_path() {
            let entry = ZipEntry::new(jar);
            cmd.set_class(entry.get_main_class().expect("jar中没有主清单属性"));
            //            self.user_class_path.as_mut().unwrap().push(Box::new(entry));
        }
    }
}

impl Entry for ClassPath {
    fn read_class(&self, class_name: &str) -> Result<(Vec<u8>, Box<dyn Entry>), FindClassError> {
        let class = class_name.to_string() + ".class";
        let boot_read_rs = self.boot_class_path.as_ref().unwrap().read_class(&class);
        if boot_read_rs.is_ok() {
            return boot_read_rs;
        }
        //        let ext_read_rs = self.ext_class_path.as_ref().unwrap().read_class(&class);
        //        if ext_read_rs.is_ok() {
        //            return boot_read_rs;
        //        }
        //        for path in self.user_class_path.as_ref().unwrap() {
        //            let user_read_rs = path.read_class(&class);
        //            if user_read_rs.is_ok() {
        //                return user_read_rs;
        //            }
        //        }
        return Err(FindClassError(
            "java.lang.ClassNotFindException".to_string(),
        ));
    }

    fn to_string(&self) -> String {
        return self.boot_class_path.as_ref().unwrap().to_string();
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::fs::File;
    use std::io::{Error, Read};
    use std::path::Path;
    use std::time::SystemTime;
    use std::{env, time};
    use zip::read::ZipFile;

    #[test]
    fn get_env() {
        for (key, value) in env::vars_os() {
            println!("{:?}: {:?}", key, value);
        }
    }

    #[test]
    fn load_jar() {
        let java_home =
            env::vars_os().find(|(key, _value)| return key == &OsString::from("JAVA_HOME"));
        let path = java_home.expect("no java home").1;
        let jar = path.to_str().unwrap().to_string() + "/jre/lib/rt.jar";
        let path = Path::new(jar.as_str());
        let time = std::time::SystemTime::now();
        println!("start:{:?}", time);
        let zip_file = File::open(path).unwrap();
        let time = SystemTime::now();
        println!("file-open:{:?}", time);
        //        let mut reader = std::io::Cursor::new(read_to_vec(zip_file));
        let time = SystemTime::now();
        println!("file-to-vec:{:?}", time);
        let mut zip = zip::ZipArchive::new(zip_file).unwrap();
        let time = SystemTime::now();
        println!("zip:{:?}", time);
        for i in 0..zip.len() {
            let mut file: ZipFile = zip.by_index(i).unwrap();
            println!("class_name:{}", file.name());
        }
    }

    fn read_to_vec(file: File) -> Vec<u8> {
        let bytes: Result<Vec<u8>, Error> = file.bytes().collect();
        return bytes.unwrap();
    }
}
