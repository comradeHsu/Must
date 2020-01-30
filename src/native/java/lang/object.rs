use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("java/lang/Object", "getClass",
                       "()Ljava/lang/Class;", get_class);
}

pub fn get_class(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_ref(0).unwrap();
    let class = (*this).borrow().class();
    let java_class = (*class).borrow().get_java_class();
    frame.operand_stack().expect("stack is none").push_ref(java_class);
}