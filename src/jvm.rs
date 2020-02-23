use crate::class_loader::app_class_loader::ClassLoader;
use crate::class_loader::bootstrap_class_loader::BootstrapClassLoader;
use crate::class_path::class_path::ClassPath;
use crate::cmd::Cmd;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::Instruction;
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::instructions::references::athrow::AThrow;
use crate::interpreter::{interpret, invoke_java_method};
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{invoke, ReturnType};
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::runtime_data_area::thread::JavaThread;
use crate::utils::{boxed, java_str_to_rust_str};
use chrono::Local;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Jvm {
    cmd: Cmd,
    boot_class_loader: BootstrapClassLoader,
    ext_class_loader: Option<Rc<RefCell<Object>>>,
    app_class_loader: Option<Rc<RefCell<Object>>>,
    main_thread: Rc<RefCell<JavaThread>>,
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
            main_thread: boxed(JavaThread::new_main_thread()),
            app_class_loader: None,
        };
        unsafe {
            JVM = Some(jvm);
            return JVM.as_mut().unwrap();
        }
    }

    #[inline]
    pub fn main_thread(&self) -> Rc<RefCell<JavaThread>> {
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
        let vm_class = self.boot_class_loader.find_or_create("sun/misc/VM");
        init_class(self.main_thread.clone(), vm_class);
        interpret(self.main_thread.clone());

        let ext_class = self
            .boot_class_loader
            .find_or_create("sun/misc/Launcher$ExtClassLoader");
        init_class(self.main_thread.clone(), ext_class.clone());

        let app_class = self
            .boot_class_loader
            .find_or_create("sun/misc/Launcher$AppClassLoader");
        init_class(self.main_thread.clone(), app_class.clone());

        interpret(self.main_thread.clone());
        self.ext_class_loader = self.create_ext_loader(ext_class);
        self.app_class_loader = self.create_app_loader(app_class);
    }

    fn exec_main(&self) {
        let class_name = self.cmd.class.clone().replace('.', "/");

        let main_class =
            ClassLoader::load_class(self.app_class_loader.clone(), class_name.as_str());
        let main_method = (*main_class).borrow().get_main_method();
        if main_method.is_none() {
            println!("Main method not found in class {}", self.cmd.class.as_str());
            return;
        }
        let args_arr = self.create_args_array();
        let mut frame = JavaThread::new_frame(self.main_thread.clone(), main_method.unwrap());
        frame
            .local_vars()
            .expect("vars is none")
            .set_ref(0, Some(args_arr));
        (*self.main_thread).borrow_mut().push_frame(frame);
        interpret(self.main_thread.clone());
    }

    fn create_args_array(&self) -> Rc<RefCell<Object>> {
        let string_class = self.boot_class_loader.find_or_create("java/lang/String");
        let args_arr_class = (*string_class).borrow().array_class();
        let mut args_arr = Class::new_array(&args_arr_class, self.cmd.args.len());
        let java_args = args_arr.mut_references();
        for i in 0..java_args.len() {
            java_args[i] = Some(StringPool::java_string(self.cmd.args[i].clone()));
        }
        return boxed(args_arr);
    }

    pub fn throw_exception(frame: &mut Frame, class_name: &str, msg: Option<&str>) {
        let bootstrap_loader = Self::boot_class_loader();
        let class = bootstrap_loader.find_or_create(class_name.replace('.', "/").as_str());
        let mut object = Class::new_object(&class);
        if msg.is_some() {
            object.set_ref_var(
                "detailMessage",
                "Ljava/lang/String;",
                StringPool::java_string(msg.unwrap().to_string()),
            );
        }
        frame
            .operand_stack()
            .expect("stack is none")
            .push_ref(Some(boxed(object)));
        let mut athrow = AThrow::new();
        athrow.execute(frame);
    }

    fn create_ext_loader(&self, ext_class: Rc<RefCell<Class>>) -> Option<Rc<RefCell<Object>>> {
        let method = Class::get_static_method(
            ext_class,
            "getExtClassLoader",
            "()Lsun/misc/Launcher$ExtClassLoader;",
        );
        let value = invoke(method.unwrap(), None, ReturnType::Object);
        return value.object();
    }

    fn create_app_loader(&self, app_class: Rc<RefCell<Class>>) -> Option<Rc<RefCell<Object>>> {
        let method = Class::get_static_method(
            app_class,
            "getAppClassLoader",
            "(Ljava/lang/ClassLoader;)Ljava/lang/ClassLoader;",
        );
        let value = invoke(method.unwrap(), None, ReturnType::Object).object();
        return value;
    }
}

#[warn(unused)]
fn display_loader_url(class_loader: Option<Rc<RefCell<Object>>>) {
    let obj = class_loader.unwrap();
    let ucp = (*obj)
        .borrow()
        .get_ref_var("ucp", "Lsun/misc/URLClassPath;");
    let boot_loader = Jvm::boot_class_loader();
    let class = boot_loader.find_or_create("java/net/URL");
    let method = Class::get_instance_method(class, "toString", "()Ljava/lang/String;").unwrap();
    if ucp.is_some() {
        let ucp = ucp.unwrap();
        let path = (*ucp)
            .borrow()
            .get_ref_var("path", "Ljava/util/ArrayList;")
            .unwrap();
        let data = (*path)
            .borrow()
            .get_ref_var("elementData", "[Ljava/lang/Object;")
            .unwrap();
        let bor = (*data).borrow();
        let objs = bor.references();
        for ob in objs {
            if ob.is_some() {
                let param = Parameters::with_parameters(vec![Parameter::Object(ob.clone())]);
                let string = invoke(method.clone(), Some(param), ReturnType::Object).object();
                let rust_str = java_str_to_rust_str(string.unwrap());
                println!("URL:{}", rust_str);
            }
        }
    }
}
