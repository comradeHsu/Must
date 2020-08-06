use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::oops::string_pool::StringPool;

pub fn init() {
    Registry::register("java/lang/String", "intern", "()Ljava/lang/String;", intern);
}

pub fn intern(frame: &mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    if this.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let interned = StringPool::intern_string(this.unwrap());
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(Some(interned));
}
