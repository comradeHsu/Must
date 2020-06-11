use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::object::Object;
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

pub fn get_class(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().class();
    let java_class = (*class).borrow().get_java_class();
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(java_class);
}

pub fn hash_code(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let ptr = (*this).borrow();
    let ptr = ptr.deref() as *const Object;
    let hash = ptr as usize;
    frame
        .operand_stack()
        .expect("stack is none")
        .push_int(hash as i32);
}

pub fn clone(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let this_class = (*this).borrow().class();
    let cloneable = Jvm::boot_class_loader()
        .find_or_create("java/lang/Cloneable")
        .unwrap();

    let borrow = cloneable.borrow();
    if !(*this_class).borrow().is_implements(borrow.deref()) {
        panic!("java.lang.CloneNotSupportedException");
    }
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(Some(boxed((*this).borrow().clone())));
}

#[cfg(test)]
mod object {
    use crate::runtime_data_area::heap::class::Class;
    use crate::runtime_data_area::heap::object::DataType::Ints;
    use crate::runtime_data_area::heap::object::Object;
    use crate::utils::boxed;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    #[test]
    fn test_rc_ptr() {
        let mut object = Object::new(boxed(Class::none()));
        object.data = Ints(vec![1, 2, 3]);
        let ptr = boxed(object);
        let p = &ptr as *const Rc<RefCell<Object>>;
        println!("rc ptr:{}", p as usize);
        let ptr = (*ptr).borrow();
        let first = match ptr.data() {
            Ints(data) => data.get(0).unwrap(),
            _ => panic!("error"),
        };
        let first_ptr = first as *const i32;
        let ptr = ptr.deref() as *const Object;
        let hash = ptr as usize;
        println!(
            "object ptr:{}, first element ptr:{}",
            hash, first_ptr as usize
        );
        let i = 99;
        let p = &i as *const i32;
        let add = p as usize;
        let t = add as *const i32;
        println!("{}: {}", add, unsafe { *t });
    }
}
