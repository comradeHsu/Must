use crate::native::registry::Registry;
use crate::oops::string_pool::StringPool;
use crate::runtime::frame::Frame;

pub fn init() {
    Registry::register("java/lang/String", "intern", "()Ljava/lang/String;", intern);
}

pub fn intern(frame: &Frame) {
    let this = frame.get_this();
    if this.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let interned = StringPool::intern_string(this.unwrap());
    frame.push_ref(Some(interned));
}
