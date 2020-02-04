use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("sun/reflect/Reflection", "getCallerClass",
                       "()Ljava/lang/Class;", getCallerClass);
}

pub fn getCallerClass(frame:&mut Frame) {
    let method = frame.method();
    let class = method.class();
    let java_class = (*class).borrow().get_java_class();
    frame.operand_stack().expect("stack is none").push_ref(java_class);
}