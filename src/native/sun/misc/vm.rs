use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::class::Class;
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("sun/misc/VM", "initialize",
                       "()V", initialize);
}

pub fn initialize(frame:&mut Frame) {
    let vm_class = frame.method().class();
    let saved_props = Class::get_ref_var(
        vm_class.clone(),
        "savedProps",
        "Ljava/util/Properties;"
    );
    let loader = (*vm_class).borrow().loader();
    let key = StringPool::java_string(loader.clone(),"foo".to_string());
    let val = StringPool::java_string(loader.clone(),"bar".to_string());
    let stack = frame.operand_stack().expect("stack is none");
    stack.push_ref(saved_props);
    stack.push_ref(Some(key));
    stack.push_ref(Some(val));
    let props_class = ClassLoader::load_class(loader,"java/util/Properties");
    let set_prop_method = Class::get_instance_method(
        props_class,
        "setProperty",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;"
    );
    invoke_method(frame,set_prop_method.unwrap());
}