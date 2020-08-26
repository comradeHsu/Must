
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::oops::object::{Data};
use crate::runtime::frame::Frame;

use std::ops::Deref;

pub fn init() {
    Registry::register(
        "java/lang/Object",
        "getClass",
        "()Ljava/lang/Class;",
        get_class,
    );
    Registry::register("java/lang/Object", "hashCode", "()I", hash_code);
    Registry::register("java/lang/Object", "clone", "()Ljava/lang/Object;", clone);
}

pub fn get_class(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.class();
    let java_class = class.get_java_class();
    frame.push_ref(java_class);
}

pub fn hash_code(frame: &Frame) {
    let this = frame.get_this().unwrap();
    frame.push_int(this.hash_code());
}

pub fn clone(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let this_class = this.class();
    let cloneable = Jvm::boot_class_loader()
        .find_or_create("java/lang/Cloneable")
        .unwrap();

    if !this_class.is_implements(&cloneable) {
        panic!("java.lang.CloneNotSupportedException");
    }
    frame.push_ref(Some(this.deep_clone()));
}

#[cfg(test)]
mod object {
    
    
    
    
    
    
    

    #[test]
    fn test_rc_ptr() {}
}
