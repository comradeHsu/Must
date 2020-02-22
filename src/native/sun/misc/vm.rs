use crate::class_loader::app_class_loader::ClassLoader;
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::string_pool::StringPool;

pub fn init() {
    Registry::register("sun/misc/VM", "initialize", "()V", initialize);
}

pub fn initialize(frame: &mut Frame) {
    let system_class = Jvm::boot_class_loader().find_or_create("java/lang/System");
    let init_method = Class::get_static_method(system_class, "initializeSystemClass", "()V");
    invoke_method(frame, init_method.unwrap());
}
