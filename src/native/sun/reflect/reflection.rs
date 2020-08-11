use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::runtime::thread::JavaThread;

pub fn init() {
    Registry::register(
        "sun/reflect/Reflection",
        "getCallerClass",
        "()Ljava/lang/Class;",
        get_caller_class,
    );
    Registry::register(
        "sun/reflect/Reflection",
        "getClassAccessFlags",
        "(Ljava/lang/Class;)I",
        get_class_access_flags,
    )
}

pub fn get_caller_class(frame: &mut Frame) {
    let method = frame.method();
    if !method.has_annotation("Lsun/reflect/CallerSensitive;") {
        let class = method.class();
        let java_class = (*class).borrow().get_java_class();
        //        println!("\tmethod name:{},first method",method.name());
        frame
            .operand_stack()
            .expect("stack is none")
            .push_ref(java_class);
    } else {
        let thread = JavaThread::current();
        let class = thread.frames_with(|frames|{
            let mut index = frames.len() - 2;
            loop {
                let pre_frame = frames.get(index).unwrap();
                let method = (**pre_frame).borrow().method_ptr();
                //            println!("method name:{}",method.name());
                if !method.has_annotation("Lsun/reflect/CallerSensitive;") {
                    let class = method.class();
                    let java_class = (*class).borrow().get_java_class();
                    return java_class;
                }
                if index == 0 {
                    return None;
                }
                index -= 1;
            }
        });
        frame
            .operand_stack()
            .expect("stack is none")
            .push_ref(class);
    }
}

// public static native int getClassAccessFlags(Class<?> type);
// (Ljava/lang/Class;)I
pub fn get_class_access_flags(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let type_ = vars.get_ref(0).unwrap();

    let class = (*type_).borrow().meta().unwrap();
    let flags = (*class).borrow().access_flags();

    let stack = frame.operand_stack().expect("stack is none");
    stack.push_int(flags as i32);
}
