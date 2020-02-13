use crate::instructions::base::method_invoke_logic::hack_invoke_method;
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::array_object::ArrayObject;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::runtime_data_area::operand_stack::OperandStack;
use crate::utils::java_str_to_rust_str;
use chrono::Local;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn init() {
    Registry::register(
        "java/lang/System",
        "arraycopy",
        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
        array_copy,
    );
    Registry::register(
        "java/lang/System",
        "setOut0",
        "(Ljava/io/PrintStream;)V",
        set_out0,
    );
    Registry::register(
        "java/lang/System",
        "initProperties",
        "(Ljava/util/Properties;)Ljava/util/Properties;",
        init_properties,
    );
    Registry::register(
        "java/lang/System",
        "setIn0",
        "(Ljava/io/InputStream;)V",
        set_in0,
    );
    Registry::register(
        "java/lang/System",
        "setErr0",
        "(Ljava/io/PrintStream;)V",
        set_err0,
    );
    Registry::register(
        "java/lang/System",
        "currentTimeMillis",
        "()J",
        current_time_millis,
    );
    Registry::register(
        "java/lang/System",
        "mapLibraryName",
        "(Ljava/lang/String;)Ljava/lang/String;",
        map_library_name,
    );
}

pub fn array_copy(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let src = vars.get_ref(0);
    let src_pos = vars.get_int(1) as usize;
    let dest = vars.get_ref(2);
    let dest_pos = vars.get_int(3) as usize;
    let length = vars.get_int(4) as usize;
    if src.is_none() || dest.is_none() {
        panic!("java.lang.NullPointerException");
    }
    let src = src.unwrap();
    let dest = dest.unwrap();
    if !check_array_copy(src.clone(), dest.clone()) {
        panic!("java.lang.ArrayStoreException");
    }
    if src_pos + length > (*src).borrow().array_length()
        || dest_pos + length > (*dest).borrow().array_length()
    {
        panic!("java.lang.IndexOutOfBoundsException");
    }
    ArrayObject::array_copy(src, dest, src_pos, dest_pos, length);
}

fn check_array_copy(src: Rc<RefCell<Object>>, dest: Rc<RefCell<Object>>) -> bool {
    let src_class = (*src).borrow().class();
    let dest_class = (*dest).borrow().class();
    if !(*src_class).borrow().is_array() || !(*dest_class).borrow().is_array() {
        return false;
    }
    let src_component = (*src_class).borrow().component_class();
    let dest_component = (*dest_class).borrow().component_class();
    if (*src_component).borrow().is_primitive() || (*dest_component).borrow().is_primitive() {
        return src_class == dest_class;
    }
    return true;
}

pub fn set_out0(frame: &mut Frame) {
    let out = frame.local_vars().expect("vars is none").get_this();
    let system_class = frame.method().class();
    Class::set_ref_var(system_class, "out", "Ljava/io/PrintStream;", out);
}

pub fn init_properties(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let props = vars.get_ref(0);

    let stack = frame.operand_stack().expect("stcak is none");
    stack.push_ref(props.clone());

    // public synchronized Object setProperty(String key, String value)
    let class = (*props.clone().unwrap()).borrow().class();
    let set_prop_method = Class::get_instance_method(
        class,
        "setProperty",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
    );
    let loader = (*frame.method().class()).borrow().loader();
    let thread = frame.thread();
    for (key, val) in _sys_props() {
        let j_key = StringPool::java_string(loader.clone(), key);
        let j_val = StringPool::java_string(loader.clone(), val);
        let mut ops = OperandStack::new(3).unwrap();
        ops.push_ref(props.clone());
        ops.push_ref(Some(j_key));
        ops.push_ref(Some(j_val));
        let shim_frame = Frame::new_shim_frame(thread.clone(), ops);
        (*thread).borrow_mut().push_frame(shim_frame);

        hack_invoke_method(thread.clone(), set_prop_method.clone().unwrap());
    }
}

fn _sys_props() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("java.version".to_owned(), "1.8.0".to_owned());
    map.insert("java.vendor".to_owned(), "jvm.rust".to_owned());
    map.insert("java.home".to_owned(), "D:\\java8\\JDK".to_owned());
    map.insert("java.class.version".to_owned(), "52.0".to_owned());
    map.insert("java.class.path".to_owned(), "todo".to_owned());
//    map.insert(
//        "java.library.path".to_owned(),
//        "D:\\java8\\JDK\\bin;C:\\WINDOWS\\Sun\\Java\\bin;C:\\WINDOWS\\system32;C:\\WINDOWS;C:\\ProgramData\\Oracle\\Java\\javapath;C:\\Program Files\\Docker\\Docker\\Resources\\bin;C:\\Program Files (x86)\\Intel\\iCLS Client\\;C:\\Program Files\\Intel\\iCLS Client\\;C:\\windows\\system32;C:\\windows;C:\\windows\\System32\\Wbem;C:\\windows\\System32\\WindowsPowerShell\\v1.0\\;C:\\Program Files (x86)\\Intel\\Intel(R) Management Engine Components\\DAL;C:\\Program Files\\Intel\\Intel(R) Management Engine Components\\DAL;C:\\Program Files (x86)\\Intel\\Intel(R) Management Engine Components\\IPT;C:\\Program Files\\Intel\\Intel(R) Management Engine Components\\IPT;C:\\Program Files (x86)\\NVIDIA Corporation\\PhysX\\Common;D:\\MinGw\\bin;D:\\Git\\cmd;D:\\NodeJs\\;C:\\WINDOWS\\system32;C:\\WINDOWS;C:\\WINDOWS\\System32\\Wbem;C:\\WINDOWS\\System32\\WindowsPowerShell\\v1.0\\;D:\\Maven\\apache-maven-3.5.3\\bin;C:\\WINDOWS\\System32\\OpenSSH\\;D:\\androidSDK\\Sdk\\platform-tools;D:\\androidSDK\\Sdk\\tools;D:\\Gradle\\gradle-5.6.3\\bin;D:\\java8\\JDK\\bin;D:\\java8\\JDK\\jre\\bin;D:\\Rust\\bin;C:\\Program Files\\dotnet\\;C:\\Program Files\\Microsoft SQL Server\\130\\Tools\\Binn\\;C:\\Program Files\\Microsoft SQL Server\\Client SDK\\ODBC\\170\\Tools\\Binn\\;C:\\Users\\xuhui\\.cargo\\bin;D:\\Python\\Scripts\\;D:\\Python\\;C:\\Users\\xuhui\\AppData\\Local\\Microsoft\\WindowsApps;C:\\Users\\xuhui\\AppData\\Roaming\\npm;D:\\NodeJs\\node_global;C:\\Users\\xuhui\\AppData\\Local\\BypassRuntm;D:\\VSCode\\Microsoft VS Code\\bin;C:\\Users\\xuhui\\AppData\\Local\\Microsoft\\WindowsApps;;.
//".to_owned()
//    );
    map.insert(
        "java.awt.graphicsenv".to_owned(),
        "sun.awt.CGraphicsEnvironment".to_owned(),
    );
    map.insert("os.name".to_owned(), std::env::consts::OS.to_owned());
    map.insert("os.arch".to_owned(), std::env::consts::ARCH.to_owned());
    map.insert("os.version".to_owned(), "".to_owned());
    map.insert("file.separator".to_owned(), "/".to_owned());
    map.insert("path.separator".to_owned(), ":".to_owned());
    map.insert("line.separator".to_owned(), "\n".to_owned());
    map.insert("user.name".to_owned(), "".to_owned());
    map.insert("user.home".to_owned(), "".to_owned());
    map.insert("user.dir".to_owned(), "".to_owned());
    map.insert("user.country".to_owned(), "CN".to_owned());
    map.insert("file.encoding".to_owned(), "UTF-8".to_owned());
    map.insert("sun.stdout.encoding".to_owned(), "UTF-8".to_owned());
    map.insert("sun.stderr.encoding".to_owned(), "UTF-8".to_owned());

    return map;
    //    return map[string]string{
    //    "java.version":         "1.8.0",
    //    "java.vendor":          "jvm.go",
    //    "java.vendor.url":      "https://github.com/zxh0/jvm.go",
    //    "java.home":            "todo",
    //    "java.class.version":   "52.0",
    //    "java.class.path":      "todo",
    //    "java.awt.graphicsenv": "sun.awt.CGraphicsEnvironment",
    //    "os.name":              runtime.GOOS,   // todo
    //    "os.arch":              runtime.GOARCH, // todo
    //    "os.version":           "",             // todo
    //    "file.separator":       "/",            // todo os.PathSeparator
    //    "path.separator":       ":",            // todo os.PathListSeparator
    //    "line.separator":       "\n",           // todo
    //    "user.name":            "",             // todo
    //    "user.home":            "",             // todo
    //    "user.dir":             ".",            // todo
    //    "user.country":         "CN",           // todo
    //    "file.encoding":        "UTF-8",
    //    "sun.stdout.encoding":  "UTF-8",
    //    "sun.stderr.encoding":  "UTF-8",
    //    }
}

// private static native void setIn0(InputStream in);
// (Ljava/io/InputStream;)V
pub fn set_in0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let in_object = vars.get_ref(0);

    let sys_class = frame.method().class();
    Class::set_ref_var(sys_class, "in", "Ljava/io/InputStream;", in_object);
}

// private static native void setErr0(PrintStream err);
// (Ljava/io/PrintStream;)V
pub fn set_err0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let err = vars.get_ref(0);

    let sys_class = frame.method().class();
    Class::set_ref_var(sys_class, "err", "Ljava/io/PrintStream;", err);
}

// public static native long currentTimeMillis();
// ()J
pub fn current_time_millis(frame: &mut Frame) {
    let millis = Local::now().timestamp_millis();
    let stack = frame.operand_stack().expect("stack is none");
    stack.push_long(millis)
}

/// public static native String mapLibraryName(String name);
/// java/lang/System.mapLibraryName(Ljava/lang/String;)Ljava/lang/String;
pub fn map_library_name(frame: &mut Frame) {
    let name = frame.local_vars().expect("vars is none").get_ref(0);
    let mut rust_name = java_str_to_rust_str(name.clone().unwrap());
    rust_name.push_str(".dll");
    let class = frame.method_by_clone().class();
    let loader = (*class).borrow().loader();
    let stack = frame.operand_stack().expect("stack is none");
    stack.push_ref(Some(StringPool::java_string(loader, rust_name)));
}
