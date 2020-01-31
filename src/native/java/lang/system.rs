use crate::runtime_data_area::frame::Frame;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::heap::array_object::ArrayObject;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("java/lang/System", "arraycopy",
                       "(Ljava/lang/Object;ILjava/lang/Object;II)V", array_copy);
}

pub fn array_copy(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let src = vars.get_ref(0);
    let src_pos = vars.get_int(1) as usize;
    let dest = vars.get_ref(2);
    let dest_pos = vars.get_int(3) as usize;
    let length = vars.get_int(4) as usize;
    if src.is_none() || dest.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let src = src.unwrap();
    let dest = dest.unwrap();
    if !check_array_copy(src.clone(), dest.clone()) {
        panic!("java.lang.ArrayStoreException");
    }
    if src_pos < 0 || dest_pos < 0 || length < 0 || src_pos+length > (*src).borrow().array_length() ||
        dest_pos+length > (*dest).borrow().array_length() {
        panic!("java.lang.IndexOutOfBoundsException");
    }
    ArrayObject::array_copy(src,dest,src_pos,dest_pos,length);
}

fn check_array_copy(src:Rc<RefCell<Object>>, dest:Rc<RefCell<Object>>) -> bool {
    let src_class = (*src).borrow().class();
    let dest_class = (*dest).borrow().class();
    if !(*src_class).borrow().is_array() || !(*dest_class).borrow().is_array() {
        return false;
    }
    let src_component = (*src_class).borrow().component_class();
    let dest_component = (*dest_class).borrow().component_class();
    if (*src_component).borrow().is_primitive() ||  (*dest_component).borrow().is_primitive() {
        return src_class == dest_class;
    }
    return true
}
