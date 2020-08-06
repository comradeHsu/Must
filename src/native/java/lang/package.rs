use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::jvm::Jvm;
use std::collections::HashSet;
use crate::oops::string_pool::StringPool;
use std::rc::Rc;
use std::cell::RefCell;
use crate::oops::object::{Object, DataType};
use crate::oops::array_object::ArrayObject;
use crate::utils::{boxed, java_str_to_rust_str};

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
pub fn get_system_packages0(frame: &mut Frame) {
    let boot_class_loader = Jvm::boot_class_loader();
    let class_loader = boot_class_loader.basic_loader();
    let borrow_loader = (*class_loader).borrow();
    let class_map = borrow_loader.class_map_immutable();
    let mut set = HashSet::with_capacity(class_map.len());
    for (key,_) in class_map {
        if !key.starts_with('[') {
            let index = key.rfind('/').unwrap();
            let (package, name) = key.split_at(index + 1);
            set.insert(package);
        }
    }
    let string_class = boot_class_loader.find_or_create("[java/lang/String").unwrap();
    let mut data = Vec::with_capacity(set.len());
    for iter in set {
        data.push(Some(StringPool::java_string(iter.to_string())));
    }
    let packages = ArrayObject::from_data(string_class,DataType::References(data));
    frame.operand_stack().expect("stack is none").push_ref(Some(boxed(packages)));
}

/// private static native String getSystemPackage0(String name);
pub fn get_system_package0(frame: &mut Frame) {
    let name = frame
        .local_vars()
        .expect("vars is none")
        .get_ref(0)
        .unwrap();
    let rust_name = java_str_to_rust_str(name.clone());
    let boot_class_loader = Jvm::boot_class_loader();
    let class_loader = boot_class_loader.basic_loader();
    let borrow_loader = (*class_loader).borrow();
    let class_map = borrow_loader.class_map_immutable();
    let mut set = HashSet::with_capacity(class_map.len());
    for (key,_) in class_map {
        if !key.starts_with('[') {
            let index = key.rfind('/');
            if index.is_some() {
                let (package, _) = key.split_at(index.unwrap() + 1);
                set.insert(package.to_string());
            }
        }
    }
    if set.contains(&rust_name) {
        frame.operand_stack().expect("stack is none").push_ref(Some(name));
    } else {
        frame.operand_stack().expect("stack is none").push_ref(None);
    }
}