use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::native::registry::Registry;
use crate::utils::java_str_to_rust_str;

pub fn init() {
    Registry::register("java/lang/Class", "getPrimitiveClass",
                       "(Ljava/lang/String;)Ljava/lang/Class;", get_primitive_class);
    Registry::register("java/lang/Class", "getName0",
                       "()Ljava/lang/String;", get_name0);
    Registry::register("java/lang/Class", "desiredAssertionStatus0",
                       "(Ljava/lang/Class;)Z", desired_assertion_status0);
}

pub fn get_primitive_class(frame:&mut Frame) {
    let name_obj = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let target = java_str_to_rust_str(name_obj);
    let class = frame.method().class();
    let loader = (*class).borrow().loader();
    let class = ClassLoader::load_class(loader,target.as_str());
    let java_class = (*class).borrow().get_java_class();
    frame.operand_stack().expect("stack null").push_ref(java_class);
}

pub fn get_name0(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta().unwrap();
    let name = (*class).borrow().java_name();
    let name_obj = StringPool::java_string((*class).borrow().loader(),name);
    frame.operand_stack().expect("stack null").push_ref(Some(name_obj));
}

pub fn desired_assertion_status0(frame:&mut Frame) {
    frame.operand_stack().expect("stack null").push_int(0);
}

