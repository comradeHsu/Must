use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::utils::boxed;
use std::{thread, time};

pub fn init() {
    Registry::register(
        "java/lang/Thread",
        "currentThread",
        "()Ljava/lang/Thread;",
        current_thread,
    );
    Registry::register("java/lang/Thread", "setPriority0", "(I)V", set_priority0);
    Registry::register("java/lang/Thread", "isAlive", "()Z", is_alive);
    Registry::register("java/lang/Thread", "start0", "()V", start0);
    Registry::register("java/lang/Thread", "sleep", "(J)V", sleep);
    Registry::register("java/lang/Thread", "yield", "()V", java_yield);
    Registry::register("java/lang/Thread", "isInterrupted", "(Z)Z", is_interrupted);
}

pub fn current_thread(frame: &mut Frame) {
    let class = frame.method().class();
    let loader = Jvm::boot_class_loader();
    let thread_class = loader.find_or_create("java/lang/Thread").unwrap();
    let mut java_thread = Class::new_object(&thread_class);
    java_thread.set_ref_var(
        "name",
        "Ljava/lang/String;",
        StringPool::java_string("Main".to_string()),
    );

    let thread_group_class = loader.find_or_create("java/lang/ThreadGroup").unwrap();
    let mut java_thread_group = Class::new_object(&thread_group_class);
    java_thread.set_ref_var("group", "Ljava/lang/ThreadGroup;", boxed(java_thread_group));
    java_thread.set_int_var("priority", "I", 1);

    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(Some(boxed(java_thread)));
}

// private native void setPriority0(int newPriority);
// (I)V
pub fn set_priority0(frame: &mut Frame) {
    // vars := frame.LocalVars()
    // this := vars.GetThis()
    // newPriority := vars.GetInt(1))
    // todo
}

// public final native boolean isAlive();
// ()Z
pub fn is_alive(frame: &mut Frame) {
    frame
        .operand_stack()
        .expect("stack is none")
        .push_boolean(false);
}

// private native void start0();
// ()V
pub fn start0(frame: &mut Frame) {
    // todo
}

// public static native void sleep(long millis) throws InterruptedException;
// (J)V
pub fn sleep(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let millis = vars.get_long(0);
    let ten_millis = time::Duration::from_millis(millis as u64);
    thread::sleep(ten_millis);
}

//  public static native void yield();
// ()V
pub fn java_yield(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    thread::yield_now();
}

// private native boolean isInterrupted(boolean ClearInterrupted);
// (Z)Z
/// waiting for impl
pub fn is_interrupted(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    frame
        .operand_stack()
        .expect("stack is none")
        .push_boolean(false);
}
