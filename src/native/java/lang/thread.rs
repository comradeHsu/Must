
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::oops::class::Class;
use crate::oops::string_pool::StringPool;
use crate::runtime::frame::Frame;

use std::{thread, time};
use crate::runtime::thread::JavaThread;
use crate::utils::java_classes::JavaLangThread;

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

pub fn current_thread(frame: &Frame) {
    let thread = JavaThread::current();
    frame.push_ref(thread.java_thread());
}

// private native void setPriority0(int newPriority);
// (I)V
pub fn set_priority0(_frame: &Frame) {
    // vars := frame.LocalVars()
    // this := vars.GetThis()
    // newPriority := vars.GetInt(1))
    // todo
}

// public final native boolean isAlive();
// ()Z
pub fn is_alive(frame: &Frame) {
    let this = frame.get_this();
    if this.is_none() {

    }
    let thread = JavaLangThread::thread(this.as_ref().unwrap());
    frame.push_boolean(thread.is_some());
}

/// private native void start0();
/// ()V
pub fn start0(frame: &Frame) {
    let this = frame.get_this();
    if this.is_none() { }
    let thread = JavaThread::new_thread(this.unwrap());
    JavaThread::start(thread);
}

// public static native void sleep(long millis) throws InterruptedException;
// (J)V
pub fn sleep(frame: &Frame) {
    let millis = frame.get_long(0);
    let ten_millis = time::Duration::from_millis(millis as u64);
    thread::sleep(ten_millis);
}

//  public static native void yield();
// ()V
pub fn java_yield(_frame: &Frame) {
    thread::yield_now();
}

// private native boolean isInterrupted(boolean ClearInterrupted);
// (Z)Z
/// waiting for impl
pub fn is_interrupted(frame: &Frame) {
    frame.push_boolean(false);
}
