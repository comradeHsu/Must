use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;

pub fn init() {
    Registry::register("sun/reflect/Reflection", "getCallerClass",
                       "()Ljava/lang/Class;", getCallerClass);
    Registry::register("sun/reflect/Reflection", "getClassAccessFlags",
                       "(Ljava/lang/Class;)I", getClassAccessFlags)
}

pub fn getCallerClass(frame:&mut Frame) {
    let method = frame.method();
    if !method.has_annotation("Lsun/reflect/CallerSensitive;") {
        let class = method.class();
        let java_class = (*class).borrow().get_java_class();
//        println!("\tmethod name:{},first method",method.name());
        frame.operand_stack().expect("stack is none").push_ref(java_class);
    } else {
        let thread = frame.thread();
        let borrow = (*thread).borrow();
        let frames = borrow.get_frames();
        let mut index = frames.len() - 2;
        while index >= 0 {
            let pre_frame = frames.get(index).unwrap();
            let method = (**pre_frame).borrow().method_ptr();
//            println!("method name:{}",method.name());
            if !method.has_annotation("Lsun/reflect/CallerSensitive;") {
                let class = method.class();
                let java_class = (*class).borrow().get_java_class();
                frame.operand_stack().expect("stack is none").push_ref(java_class);
                return;
            }
            index -= 1;
        }
    }
}

// public static native int getClassAccessFlags(Class<?> type);
// (Ljava/lang/Class;)I
pub fn getClassAccessFlags(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let type_ = vars.get_ref(0).unwrap();

    let class = (*type_).borrow().meta().unwrap();
    let flags = (*class).borrow().access_flags();

    let stack = frame.operand_stack().expect("stack is none");
    stack.push_int(flags as i32);
}