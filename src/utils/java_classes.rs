use crate::oops::object::Object;
use crate::runtime::thread::JavaThread;

pub struct JavaLangThread();

impl JavaLangThread {

    pub fn set_thread_status(java_thread: Object, status: i32) {

    }

    pub fn stack_size(java_thread: &Object) -> i64 {
        return java_thread.get_long_var("stackSize","J");
    }

    pub fn thread(java_thread: &Object) -> Option<JavaThread> {
        return java_thread.meta_data().thread();
    }

    pub fn set_thread(java_thread: &Object, thread: JavaThread) {
        return java_thread.set_thread(thread);
    }
}