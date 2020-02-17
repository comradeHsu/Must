use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::utils::java_str_to_rust_str;

pub fn init() {
    Registry::register(
        "java/lang/ClassLoader",
        "findBuiltinLib",
        "(Ljava/lang/String;)Ljava/lang/String;",
        find_built_in_lib,
    );
    Registry::register(
        "java/lang/ClassLoader$NativeLibrary",
        "load",
        "(Ljava/lang/String;Z)V",
        load,
    );
    Registry::register(
        "java/lang/ClassLoader",
        "findLoadedClass0",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        find_loaded_class0,
    );
}

/// public static native String findBuiltinLib(String name);
/// java/lang/ClassLoader.findBuiltinLib
/// (Ljava/lang/String;)Ljava/lang/String;
pub fn find_built_in_lib(frame: &mut Frame) {
    let name = frame.local_vars().expect("vars is none").get_ref(0);
    let stack = frame.operand_stack().expect("stack is none");
    stack.push_ref(name)
}

/// java/lang/ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V'
pub fn load(frame: &mut Frame) {
    let name = frame.local_vars().expect("vars is none").get_ref(1);
    println!("lib name:{}", java_str_to_rust_str(name.clone().unwrap()));
}

/// private native final Class<?> findLoadedClass0(String name);
/// (Ljava/lang/String;)Ljava/lang/Class;
pub fn find_loaded_class0(frame: &mut Frame) {
    let class = frame.method().class();
    let loader = (*class).borrow().loader();
    let name = frame.local_vars().expect("vars is none").get_ref(1);
    let class_name = java_str_to_rust_str(name.unwrap());
    let class = (*loader).borrow().get_class(class_name.as_str());
    if class.is_none() {
        println!("None CLass Is {}",class_name);
        frame.operand_stack().expect("stack is none").push_ref(None);
    } else {
        let java_class = (*class.unwrap()).borrow().get_java_class();
        frame.operand_stack().expect("stack is none").push_ref(java_class);
    }
}
