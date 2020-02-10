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
    let loader = (*vm_class).borrow().loader();
    let system_class = ClassLoader::load_class(loader,"java/lang/System");
    let init_method = (*system_class).borrow()
        .get_static_method("initializeSystemClass", "()V");
    invoke_method(frame,init_method.unwrap());
}