use super::class_path::{Entry, PATH_LIST_SEPARATOR};
use crate::class_path::class_path::FindClassError;
use crate::class_path::zip_entry::ZipEntry;
use std::fs;
use std::path::Path;

type CompositeEntry = Vec<Box<dyn Entry>>;

pub fn new(paths: &String) -> CompositeEntry {
    let mut composites = Vec::new();
    let mut_paths = paths.clone();
    let ps: Vec<&str> = mut_paths.as_str().split(PATH_LIST_SEPARATOR).collect();
    for s in ps {
        composites.push(super::class_path::new_entry(&String::from(s)));
    }
    return composites;
}

pub fn new_wildcard_entry(path: &String) -> CompositeEntry {
    let mut mut_path = path.clone();
    mut_path.pop();
    let mut composites: Vec<Box<dyn Entry>> = Vec::new();
    let mut walk_fn = |info: &Path| {
        let info_path = info.to_str().unwrap();
        if info.is_dir() && &info_path != &mut_path.as_str() {
            return;
        }
        if info_path.ends_with(".jar") || info_path.ends_with(".JAR") {
            let jar_entry = ZipEntry::new(&info_path.to_string());
            composites.push(Box::new(jar_entry));
        }
    };
    let paths = fs::read_dir(&mut_path).unwrap();
    for path_file in paths {
        let pf = path_file.unwrap().path();
        walk_fn(pf.as_path());
    }
    return composites;
}

impl Entry for CompositeEntry {
    fn read_class(&self, class_name: &str) -> Result<(Vec<u8>, Box<dyn Entry>), FindClassError> {
        let mut rs = Err(FindClassError("don't find class".to_string()));
        for e in self {
            rs = e.read_class(class_name);
            if rs.is_ok() {
                return rs;
            }
        }
        return rs;
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for e in self {
            s.push('\n');
            s.push_str(e.to_string().as_str());
        }
        return s;
    }
}

#[cfg(test)]
mod tests {
    use crate::class_path::class_path::new_entry;

    #[test]
    fn new() {
        let composites = new_entry(&"C:\\Users\\xuhui\\Desktop\\force.jar".to_string());
        println!("entry:{}", composites.to_string());
        let composite = new_entry(&"C:\\Users\\xuhui\\Desktop\\*".to_string());
        println!("entry:{}", composite.to_string());
    }
}
