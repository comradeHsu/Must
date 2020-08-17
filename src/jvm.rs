use crate::class_loader::app_class_loader::ClassLoader;
use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;
use crate::class_path::class_path::ClassPath;
use crate::cmd::Cmd;
use crate::instructions::base::class_init_logic::init_class;



use crate::interpreter::interpret;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{JavaCall, ReturnType};
use crate::oops::class::Class;
use crate::oops::object::Object;
use crate::oops::string_pool::StringPool;

use crate::runtime::frame::Frame;
use crate::runtime::thread::JavaThread;
use crate::utils::{java_str_to_rust_str};
use chrono::Local;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Jvm {
    cmd: Cmd,
    boot_class_loader: BootstrapClassLoader,
    ext_class_loader: Option<Object>,
    app_class_loader: Option<Object>,
    main_thread: JavaThread,
}

pub static mut JVM: Option<Jvm> = None;

impl Jvm {
    pub fn new(mut cmd: Cmd) -> &'static mut Jvm {
        let mut cp = ClassPath::parse(&cmd.x_jre_option, &cmd.cp_option);
        if cmd.exec_jar_path().is_some() {
            cp.handle_jar(&mut cmd);
        }
        let class_path = Rc::new(cp);
        let class_loader = BootstrapClassLoader::new(class_path);
        let jvm = Jvm {
            cmd,
            boot_class_loader: class_loader,
            ext_class_loader: None,
            main_thread: JavaThread::new_main_thread(),
            app_class_loader: None,
        };
        jvm.main_thread.set();
        unsafe {
            JVM = Some(jvm);
            return JVM.as_mut().unwrap();
        }
    }

    #[inline]
    pub fn main_thread(&self) -> JavaThread {
        return self.main_thread.clone();
    }

    #[inline]
    pub fn boot_class_loader() -> &'static BootstrapClassLoader {
        return &Self::instance().unwrap().boot_class_loader;
    }

    #[inline]
    pub fn instance() -> Option<&'static Self> {
        unsafe {
            return JVM.as_ref();
        }
    }

    pub fn start(&mut self) {
        //        let builder = (*self.main_thread).borrow_mut().std_thread();
        //        let join_handler = builder.spawn(move || {
        self.boot_class_loader.post_constructor();
        self.init_vm();
        println!("init vm! {:?}", Local::now());
        self.exec_main();
        //        }).unwrap();
        //        join_handler.join().expect_err("thread::spawn failed");
    }

    fn init_vm(&mut self) {
        let vm_class = self
            .boot_class_loader
            .find_or_create("sun/misc/VM")
            .unwrap();
        init_class(vm_class);
        interpret(self.main_thread.clone());

        let ext_class = self
            .boot_class_loader
            .find_or_create("sun/misc/Launcher$ExtClassLoader")
            .unwrap();
        init_class(ext_class.clone());

        let app_class = self
            .boot_class_loader
            .find_or_create("sun/misc/Launcher$AppClassLoader")
            .unwrap();
        init_class(app_class.clone());

        interpret(self.main_thread.clone());
        self.ext_class_loader = self.create_ext_loader(ext_class);
        self.app_class_loader = self.create_app_loader(app_class, self.ext_class_loader.clone());
        display_loader_url(self.app_class_loader.clone());
    }

    fn exec_main(&self) {
        let class_name = self.cmd.class.clone().replace('.', "/");
        //let class_name = self.cmd.class.clone();

        let main_class =
            ClassLoader::load_class(self.app_class_loader.clone(), class_name.as_str());
        let main_method = (*main_class).borrow().get_main_method();
        if main_method.is_none() {
            println!("Main method not found in class {}", self.cmd.class.as_str());
            return;
        }
        let args_arr = self.create_args_array();
        let frame = Frame::new(main_method.unwrap());
        frame.set_ref(0, Some(args_arr));
        self.main_thread.push_frame(frame);
        interpret(self.main_thread.clone());
    }

    fn create_args_array(&self) -> Object {
        let string_class = self
            .boot_class_loader
            .find_or_create("java/lang/String")
            .unwrap();
        let args_arr_class = (*string_class).borrow().array_class();
        let args_arr = Class::new_array(&args_arr_class, self.cmd.args.len());
        args_arr.mut_references(|java_args| {
            for i in 0..java_args.len() {
                java_args[i] = Some(StringPool::java_string(self.cmd.args[i].clone()));
            }
        });
        return args_arr;
    }

    fn create_ext_loader(&self, ext_class: Rc<RefCell<Class>>) -> Option<Object> {
        let method = Class::get_static_method(
            ext_class,
            "getExtClassLoader",
            "()Lsun/misc/Launcher$ExtClassLoader;",
        );
        let value = JavaCall::invoke(method.unwrap(), None, ReturnType::Object);
        return value.object();
    }

    fn create_app_loader(
        &self,
        app_class: Rc<RefCell<Class>>,
        parent: Option<Object>,
    ) -> Option<Object> {
        let method = Class::get_static_method(
            app_class,
            "getAppClassLoader",
            "(Ljava/lang/ClassLoader;)Ljava/lang/ClassLoader;",
        );
        let params = Parameters::with_parameters(vec![Parameter::Object(parent)]);
        let value = JavaCall::invoke(method.unwrap(), Some(params), ReturnType::Object).object();
        return value;
    }
}

fn display_loader_url(class_loader: Option<Object>) {
    let obj = class_loader.unwrap();
    let ucp = obj.get_ref_var("ucp", "Lsun/misc/URLClassPath;");

    let parent = obj.get_ref_var("parent", "Ljava/lang/ClassLoader;");
    if parent.is_some() {
        let parent = parent.unwrap().class();
        println!("parent:{}", (*parent).borrow().java_name());
    }

    let boot_loader = Jvm::boot_class_loader();
    let class = boot_loader.find_or_create("java/net/URL").unwrap();
    let method = Class::get_instance_method(class, "toString", "()Ljava/lang/String;").unwrap();
    if ucp.is_some() {
        let ucp = ucp.unwrap();
        let path = ucp.get_ref_var("path", "Ljava/util/ArrayList;").unwrap();
        let data = path
            .get_ref_var("elementData", "[Ljava/lang/Object;")
            .unwrap();
        data.references(|objs| {
            for ob in objs {
                if ob.is_some() {
                    let param = Parameters::with_parameters(vec![Parameter::Object(ob.clone())]);
                    let string =
                        JavaCall::invoke(method.clone(), Some(param), ReturnType::Object).object();
                    let rust_str = java_str_to_rust_str(string.unwrap());
                    println!("URL:{}", rust_str);
                }
            }
        });
    }
}
