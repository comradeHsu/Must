use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;
use crate::runtime_data_area::heap::class::Class;
use crate::instructions::base::method_invoke_logic::invoke_method;

pub fn init() {
    Registry::register("java/security/AccessController", "doPrivileged",
                       "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;", do_privileged);
    Registry::register("java/security/AccessController", "doPrivileged",
                       "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;", do_privileged);
    Registry::register("testJava/LambdaTest", "run",
                       "(LtestJava/Action;)I", run);
    Registry::register("java/security/AccessController", "getStackAccessControlContext",
                       "()Ljava/security/AccessControlContext;", get_stack_access_control_context);
}

pub fn do_privileged(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    if this.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let class = (*this.clone().unwrap()).borrow().class();
    let method = Class::get_instance_method(class,"run","()Ljava/lang/Object;").unwrap();
    frame.operand_stack().expect("stack is none").push_ref(this);
    invoke_method(frame,method);
}

pub fn run(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    if this.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let class = (*this.clone().unwrap()).borrow().class();
    let method = Class::get_instance_method(class,"run","()I").unwrap();
    frame.operand_stack().expect("stack is none").push_ref(this);
    invoke_method(frame,method);
}

/// private static native AccessControlContext getStackAccessControlContext();
/// ()Ljava/security/AccessControlContext;
pub fn get_stack_access_control_context(frame:&mut Frame) {
    // todo
    frame.operand_stack().expect("stack is none").push_ref(None);
}