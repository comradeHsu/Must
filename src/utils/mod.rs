use crate::runtime_data_area::heap::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub mod numbers;

pub fn boxed<T>(data: T) -> Rc<RefCell<T>> {
    return Rc::new(RefCell::new(data));
}

pub fn java_str_to_rust_str(name_obj: Rc<RefCell<Object>>) -> String {
    let mete_str = (*name_obj)
        .borrow()
        .get_ref_var("value", "[C")
        .expect("str is null");
    let borrow = (*mete_str).borrow();
    let string = borrow.chars();
    let target = String::from_utf16(string).expect("u16 seqs has mistake");
    target
}
