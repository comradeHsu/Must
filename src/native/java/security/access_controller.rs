use crate::class_loader::app_class_loader::ClassLoader;
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{JavaCall, ReturnType};
use crate::native::registry::Registry;
use crate::oops::class::Class;
use crate::oops::object::Object;
use crate::runtime::frame::Frame;




pub fn init() {
    Registry::register(
        "java/security/AccessController",
        "doPrivileged",
        "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
        do_privileged,
    );
    Registry::register(
        "java/security/AccessController",
        "doPrivileged",
        "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
        do_privileged,
    );
    Registry::register(
        "java/security/AccessController",
        "doPrivileged",
        "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
        do_privileged,
    );
    Registry::register("testJava/LambdaTest", "run", "(LtestJava/Action;)I", run);
    Registry::register(
        "java/security/AccessController",
        "getStackAccessControlContext",
        "()Ljava/security/AccessControlContext;",
        get_stack_access_control_context,
    );
    Registry::register(
        "java/security/AccessController",
        "getInheritedAccessControlContext",
        "()Ljava/security/AccessControlContext;",
        get_stack_access_control_context,
    );
}

pub fn do_privileged(frame: &Frame) {
    let this = frame.get_this();
    if this.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let class = this.as_ref().unwrap().class();
    let method = class.get_instance_method("run", "()Ljava/lang/Object;").unwrap();
    frame.push_ref(this);
    invoke_method(frame, method);
}

pub fn run(frame: &Frame) {
    let this = frame.get_this();
    if this.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let class = this.as_ref().unwrap().class();
    let method = class.get_instance_method("run", "()I").unwrap();
    frame.push_ref(this);
    invoke_method(frame, method);
}

/// private static native AccessControlContext getStackAccessControlContext();
/// ()Ljava/security/AccessControlContext;
pub fn get_stack_access_control_context(frame: &Frame) {
    // todo
    frame.push_ref(create());
}

fn create() -> Option<Object> {
    let class = ClassLoader::load_class(None, "[Ljava/security/ProtectionDomain");
    let args = Class::new_array(&class, 0);
    let class = ClassLoader::load_class(None, "java/security/AccessControlContext");

    let object = Object::new(&class);

    let method =
        class.get_instance_method("<init>", "([Ljava/security/ProtectionDomain;)V");
    let params = Parameters::with_parameters(vec![
        Parameter::Object(Some(object.clone())),
        Parameter::Object(Some(args)),
    ]);
    JavaCall::invoke(method.unwrap(), Some(params), ReturnType::Void);
    return Some(object);
}
