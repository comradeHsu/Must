use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::utils::boxed;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("java/lang/Thread", "currentThread",
                       "()Ljava/lang/Thread;", currentThread);
}

pub fn currentThread(frame:&mut Frame) {
    let class = frame.method().class();
    let loader = (*class).borrow().loader();
    let thread_class = ClassLoader::load_class(loader.clone(),"java/lang/Thread");
    let mut java_thread = Class::new_object(&thread_class);
    java_thread.set_ref_var("name","Ljava/lang/String;",
                            StringPool::java_string(loader,"Main".to_string()));
    frame.operand_stack().expect("stack is none").push_ref(Some(boxed(java_thread)));
}