use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::oops::array_object::ArrayObject;
use crate::oops::object::{DataType};
use crate::oops::string_pool::StringPool;
use crate::runtime::frame::Frame;
use crate::utils::{java_str_to_rust_str};

use std::collections::HashSet;


pub fn init() {
    Registry::register(
        "java/lang/Package",
        "getSystemPackages0",
        "()[Ljava/lang/String;",
        get_system_packages0,
    );
    Registry::register(
        "java/lang/Package",
        "getSystemPackage0",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_system_package0,
    );
}

/// private static native String[] getSystemPackages0();
pub fn get_system_packages0(frame: &Frame) {
    let boot_class_loader = Jvm::boot_class_loader();
    let class_loader = boot_class_loader.basic_loader();

    let set = class_loader.classes_with(|maps|{
        let mut set = HashSet::with_capacity(maps.len());
        for (key, _) in maps {
            if !key.starts_with('[') {
                let index = key.rfind('/').unwrap();
                let (package, _name) = key.split_at(index + 1);
                set.insert(package.to_owned());
            }
        }
        set
    });

    let string_class = boot_class_loader
        .find_or_create("[java/lang/String")
        .unwrap();
    let mut data = Vec::with_capacity(set.len());
    for iter in set {
        data.push(Some(StringPool::java_string(iter)));
    }
    let packages = ArrayObject::from_data(&string_class, DataType::References(data));
    frame.push_ref(Some(packages));
}

/// private static native String getSystemPackage0(String name);
pub fn get_system_package0(frame: &Frame) {
    let name = frame.get_ref(0).unwrap();
    let rust_name = java_str_to_rust_str(name.clone());
    let boot_class_loader = Jvm::boot_class_loader();
    let class_loader = boot_class_loader.basic_loader();

    let set = class_loader.classes_with(|classes|{
        let mut set = HashSet::with_capacity(classes.len());
        for (key, _) in classes {
            if !key.starts_with('[') {
                let index = key.rfind('/');
                if index.is_some() {
                    let (package, _) = key.split_at(index.unwrap() + 1);
                    set.insert(package.to_owned());
                }
            }
        }
        set
    });

    if set.contains(&rust_name) {
        frame.push_ref(Some(name));
    } else {
        frame.push_ref(None);
    }
}
