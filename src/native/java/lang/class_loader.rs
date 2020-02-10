use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;
use crate::utils::java_str_to_rust_str;

pub fn init() {
    Registry::register("java/lang/ClassLoader", "findBuiltinLib",
                       "(Ljava/lang/String;)Ljava/lang/String;", find_built_in_lib);
    Registry::register("java/lang/ClassLoader$NativeLibrary", "load",
                       "(Ljava/lang/String;Z)V", load);
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
    println!("lib name:{}",java_str_to_rust_str(name.clone().unwrap()));
}