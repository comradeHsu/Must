use crate::class_loader::app_class_loader::ClassLoader;
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{invoke, ReturnType};
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::object::Object;
use crate::utils::boxed;
use std::cell::RefCell;
use std::rc::Rc;

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

pub fn do_privileged(frame: &mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    if this.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let class = (*this.clone().unwrap()).borrow().class();
    let method = Class::get_instance_method(class, "run", "()Ljava/lang/Object;").unwrap();
    frame.operand_stack().expect("stack is none").push_ref(this);
    invoke_method(frame, method);
}

pub fn run(frame: &mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    if this.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let class = (*this.clone().unwrap()).borrow().class();
    let method = Class::get_instance_method(class, "run", "()I").unwrap();
    frame.operand_stack().expect("stack is none").push_ref(this);
    invoke_method(frame, method);
}

/// private static native AccessControlContext getStackAccessControlContext();
/// ()Ljava/security/AccessControlContext;
pub fn get_stack_access_control_context(frame: &mut Frame) {
    // todo
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(create());
}

fn create() -> Option<Rc<RefCell<Object>>> {
    let class = ClassLoader::load_class(None, "[Ljava/security/ProtectionDomain");
    let args = boxed(Class::new_array(&class, 0));
    let class = ClassLoader::load_class(None, "java/security/AccessControlContext");

    let object = boxed(Object::new(class.clone()));

    let method =
        Class::get_instance_method(class, "<init>", "([Ljava/security/ProtectionDomain;)V");
    let params = Parameters::with_parameters(vec![
        Parameter::Object(Some(object.clone())),
        Parameter::Object(Some(args)),
    ]);
    invoke(method.unwrap(), Some(params), ReturnType::Void);
    return Some(object);
}
