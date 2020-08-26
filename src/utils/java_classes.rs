use crate::oops::object::Object;

pub struct JavaLangThread {

}

impl JavaLangThread {

    pub fn set_thread_status(java_thread: Object, status: i32) {

    }

    pub fn stack_size(java_thread: &Object) -> i64 {
        return java_thread.get_long_var("stackSize","J");
    }
}