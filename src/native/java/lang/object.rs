use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::oops::object::{Object, Data};
use crate::utils::boxed;
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
    let this = frame
        .get_this()
        .unwrap();
    let class = this.class();
    let java_class = (*class).borrow().get_java_class();
    frame.push_ref(java_class);
}

pub fn hash_code(frame: &Frame) {
    let this = frame
        .get_this()
        .unwrap();
    let ptr = (this.data).borrow();
    let ptr = ptr.deref() as *const Data;
    let hash = ptr as usize;
    frame.push_int(hash as i32);
}

pub fn clone(frame: &Frame) {
    let this = frame
        .get_this()
        .unwrap();
    let this_class = this.class();
    let cloneable = Jvm::boot_class_loader()
        .find_or_create("java/lang/Cloneable")
        .unwrap();

    let borrow = cloneable.borrow();
    if !(*this_class).borrow().is_implements(borrow.deref()) {
        panic!("java.lang.CloneNotSupportedException");
    }
    frame.push_ref(Some(this.deep_clone()));
}

#[cfg(test)]
mod object {
    use crate::oops::class::Class;
    use crate::oops::object::DataType::Ints;
    use crate::oops::object::Object;
    use crate::utils::boxed;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    #[test]
    fn test_rc_ptr() {
    }
}
