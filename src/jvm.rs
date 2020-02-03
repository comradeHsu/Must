use crate::cmd::Cmd;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::thread::Thread;
use crate::class_path::class_path::ClassPath;
use crate::utils::boxed;
use crate::instructions::base::class_init_logic::init_class;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::runtime_data_area::heap::object::Object;
use crate::interpreter::interpret;

pub struct Jvm {
    cmd:Cmd,
    class_loader:Rc<RefCell<ClassLoader>>,
    main_thread:Rc<RefCell<Thread>>
}

impl Jvm {

    pub fn new(cmd:Cmd) -> Jvm {
        let cp = ClassPath::parse(&cmd.x_jre_option,&cmd.cp_option);
        let class_path = Rc::new(cp);
        let class_loader = ClassLoader::new(class_path,cmd.verbose_class);
        return Jvm{
            cmd,
            class_loader,
            main_thread: boxed(Thread::new_thread())
        };
    }

    pub fn start(&self) {
        self.init_vm();
        self.exec_main();
    }

    fn init_vm(&self) {
        let vm_class = ClassLoader::load_class(self.class_loader.clone(),"sun/misc/VM");
        init_class(self.main_thread.clone(),vm_class);
        interpret(self.main_thread.clone());
    }

    fn exec_main(&self) {
        let class_name = self.cmd.class.clone().replace('.',"/");
        let main_class = ClassLoader::load_class(
            self.class_loader.clone(),
            class_name.as_str()
        );
        let main_method = (*main_class).borrow().get_main_method();
        if main_method.is_none() {
            println!("Main method not found in class {}", self.cmd.class.as_str());
            return;
        }
        let args_arr = self.create_args_array();
        let mut frame = Thread::new_frame(self.main_thread.clone(),main_method.unwrap());
        frame.local_vars().expect("vars is none").set_ref(0,Some(args_arr));
        (*self.main_thread).borrow_mut().push_frame(frame);
        interpret(self.main_thread.clone());
    }

    fn create_args_array(&self) -> Rc<RefCell<Object>> {
        let string_class = ClassLoader::load_class(
            self.class_loader.clone(),
            "java/lang/String"
        );
        let args_arr_class = (*string_class).borrow().array_class();
        let mut args_arr = Class::new_array(&args_arr_class,self.cmd.args.len());
        let java_args = args_arr.mut_references();
        for i in 0..java_args.len() {
            java_args[i] = Some(StringPool::java_string(self.class_loader.clone(),self.cmd.args[i].clone()));
        }
        return boxed(args_arr);
    }
}