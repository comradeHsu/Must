use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::{Jvm, JVM};
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
    Registry::register(
        "java/lang/ClassLoader",
        "findBootstrapClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        find_bootstrap_class,
    );
    Registry::register(
        "java/lang/ClassLoader",
        "defineClass1",
        "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;",
        define_class1,
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
    let vars = frame.local_vars().expect("vars is none");
    let this = vars.get_this().unwrap();
    let loader = (*this).borrow().get_class_loader();
    let name = vars.get_ref(1);
    let class_name = java_str_to_rust_str(name.unwrap());
    let class = (*loader).borrow().find_class(class_name.as_str());
    if class.is_none() {
        println!("None CLass Is {}", class_name);
        frame.operand_stack().expect("stack is none").push_ref(None);
    } else {
        let java_class = (*class.unwrap()).borrow().get_java_class();
        frame
            .operand_stack()
            .expect("stack is none")
            .push_ref(java_class);
    }
}

/// private native Class<?> findBootstrapClass(String name);
/// (Ljava/lang/String;)Ljava/lang/Class;
pub fn find_bootstrap_class(frame: &mut Frame) {
    let class = frame.method().class();
    let loader = Jvm::boot_class_loader();
    let name = frame.local_vars().expect("vars is none").get_ref(1);
    let class_name = java_str_to_rust_str(name.unwrap());
    let class = loader.find_class(class_name.as_str());
    if class.is_none() {
        println!("None CLass Is {}", class_name);
        frame.operand_stack().expect("stack is none").push_ref(None);
    } else {
        let java_class = (*class.unwrap()).borrow().get_java_class();
        frame
            .operand_stack()
            .expect("stack is none")
            .push_ref(java_class);
    }
}
/// private native Class<?> defineClass1(String name, byte[] b, int off, int len,
///                                         ProtectionDomain pd, String source);
/// (Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;
pub fn define_class1(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let this = vars.get_ref(0).unwrap();
    let java_string = vars.get_ref(1);
    let byte_array = vars.get_ref(2);
    let offset = vars.get_int(3) as usize;
    let length = vars.get_int(4) as usize;
    let protection_domain = vars.get_ref(5);
    let source = vars.get_ref(6);

    let class_name = java_str_to_rust_str(java_string.unwrap());
    let class = ClassLoader::define_class_internal(class_name.as_str(),byte_array,offset,length,this,protection_domain);
    let java_class = (*class).borrow().get_java_class();

    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(java_class);
}
