use crate::class_file::class_file::ClassFile;
use crate::class_loader::class_init_preparation::ClassPreparation;
use crate::class_path::class_path::{ClassPath, Entry};
use crate::instrument::java_lang_instrument::JavaLangInstrument;
use crate::interpreter::invoke_java_method;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{invoke, ReturnType};
use crate::jvm::{Jvm, JVM};
use crate::runtime_data_area::heap::access_flags::PUBLIC;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::class_name_helper::PrimitiveTypes;
use crate::runtime_data_area::heap::constant_pool::{Constant, ConstantPool};
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::runtime_data_area::thread::JavaThread;
use crate::utils::boxed;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;
use std::thread::Thread;

struct ClassLoader {
    class_path: Rc<ClassPath>,
    verbose_class: bool,
    class_map: HashMap<String, Rc<RefCell<Class>>>,
}

impl ClassLoader {
    #[inline]
    pub fn non_bootstrap_loader(verbose_class: bool) -> Rc<RefCell<ClassLoader>> {
        let class_loader = boxed(ClassLoader {
            class_path: Rc::new(ClassPath::new()),
            verbose_class,
            class_map: Default::default(),
        });
        return class_loader;
    }

    #[inline]
    pub fn new(class_path: Rc<ClassPath>, verbose_class: bool) -> Rc<RefCell<ClassLoader>> {
        let class_loader = boxed(ClassLoader {
            class_path,
            verbose_class,
            class_map: Default::default(),
        });
        ClassLoader::load_basic_classes(class_loader.clone());
        ClassLoader::load_primitive_classes(class_loader.clone());
        return class_loader;
    }

    fn load_basic_classes(loader: Rc<RefCell<ClassLoader>>) {
        let java_lang_class = ClassLoader::load_class(loader.clone(), "java/lang/Class");
        let borrow = (*loader).borrow();
        let maps = borrow.class_map_immutable();
        for (k, v) in maps {
            let mut borrow_class = (**v).borrow_mut();
            let j_l_class = borrow_class.java_class();
            if j_l_class.is_none() {
                let mut class_object = Class::new_object(&java_lang_class);
                class_object.set_meta(v.clone());
                let boxed = boxed(class_object);
                borrow_class.set_java_class(Some(boxed));
            }
        }
    }

    fn load_primitive_classes(loader: Rc<RefCell<ClassLoader>>) {
        let primitives = PrimitiveTypes::instance().unwrap();
        let maps = primitives.primitive_types();
        for (k, v) in maps {
            ClassLoader::load_primitive_class(loader.clone(), k);
        }
    }

    fn load_primitive_class(loader: Rc<RefCell<ClassLoader>>, class_name: &str) {
        let mut class = Class::primitive_class(loader.clone(), class_name);
        let class_class = (*loader).borrow().get_class("java/lang/Class");
        let mut class_object = Class::new_object(&class_class.unwrap());
        let boxed_class = boxed(class);
        class_object.set_meta(boxed_class.clone());
        (*boxed_class)
            .borrow_mut()
            .set_java_class(Some(boxed(class_object)));
        (*loader)
            .borrow_mut()
            .class_map
            .insert(class_name.to_string(), boxed_class);
    }

    #[inline]
    pub fn class_map(&mut self) -> &mut HashMap<String, Rc<RefCell<Class>>> {
        return &mut self.class_map;
    }

    #[inline]
    pub fn class_map_immutable(&self) -> &HashMap<String, Rc<RefCell<Class>>> {
        return &self.class_map;
    }

    #[inline]
    pub fn get_class(&self, name: &str) -> Option<Rc<RefCell<Class>>> {
        let rs = self.class_map.get(name);
        match rs {
            Some(r) => return Some(r.clone()),
            None => None,
        }
    }

    pub fn load_class(loader: Rc<RefCell<ClassLoader>>, class_name: &str) -> Rc<RefCell<Class>> {
        let clone_loader = loader.clone();
        let class_op: Option<Rc<RefCell<Class>>> = (*clone_loader).borrow().get_class(class_name);
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }
        let mut class: Option<Rc<RefCell<Class>>> = None;
        if class_name.starts_with('[') {
            class = Some(ClassLoader::load_array_class(loader.clone(), class_name));
        } else {
            class = Some(ClassLoader::load_non_array_class(
                loader.clone(),
                class_name,
            ));
        }
        let value = class.unwrap();
        let class_class = (*loader).borrow().get_class("java/lang/Class");
        if class_class.is_some() {
            let mut class_object = Class::new_object(&class_class.unwrap());
            class_object.set_meta(value.clone());
            let boxed = boxed(class_object);
            (*value).borrow_mut().set_java_class(Some(boxed));
        }
        return value;
    }

    ///load array's class
    fn load_array_class(loader: Rc<RefCell<ClassLoader>>, class_name: &str) -> Rc<RefCell<Class>> {
        let class = Class::new_array_class(loader.clone(), class_name);
        let class_ptr = boxed(class);
        (*loader)
            .borrow_mut()
            .class_map
            .insert(class_name.to_string(), class_ptr.clone());
        return class_ptr;
    }

    pub fn load_non_array_class(
        loader: Rc<RefCell<ClassLoader>>,
        class_name: &str,
    ) -> Rc<RefCell<Class>> {
        let (bytes, entry) = (*loader).borrow().read_class(class_name);
        let class = ClassLoader::define_class(loader.clone(), bytes);
        ClassLoader::link(&class);
        if (*loader).borrow().verbose_class {
            println!("Loaded {}.class from {}", class_name, entry.to_string());
        }
        return class;
    }

    pub fn read_class(&self, class_name: &str) -> (Vec<u8>, Box<dyn Entry>) {
        let result = self.class_path.read_class(class_name);
        if result.is_err() {
            panic!("java.lang.ClassNotFoundException:{}", class_name);
        }
        return result.unwrap();
    }

    pub fn define_class(loader: Rc<RefCell<ClassLoader>>, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let mut class = ClassLoader::parse_class(data);
        (*class).borrow_mut().set_class_loader(loader.clone());
        ClassLoader::resolve_super_class(class.clone());
        ClassLoader::resolve_interfaces(class.clone());
        (*loader)
            .borrow_mut()
            .class_map
            .insert((*class).borrow().name().to_string(), class.clone());
        return class;
    }

    pub fn parse_class(data: Vec<u8>) -> Rc<RefCell<Class>> {
        let class_file = ClassFile::parse(data);
        //        class_file.display();
        return Class::new(class_file);
    }

    pub fn resolve_super_class(class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let super_class_name = class.super_class_name();
        //        println!("resolve_super_class:{:?},super:{:?}",class.name(),super_class_name);
        if class.name() != "java/lang/Object" && super_class_name.is_some() {
            let super_class =
                ClassLoader::load_class(class.loader(), super_class_name.unwrap().as_str());
            class.set_super_class(super_class);
        }
    }

    pub fn resolve_interfaces(class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let interfaces_name = class.interfaces_name();
        let len = interfaces_name.len();
        if len > 0 {
            let mut interfaces = Vec::with_capacity(len);
            for name in interfaces_name {
                let interface = ClassLoader::load_class(class.loader(), name);
                interfaces.push(interface);
            }
            class.set_interfaces(interfaces);
        }
    }

    pub fn link(class: &Rc<RefCell<Class>>) {
        ClassLoader::verify(class);
        ClassPreparation::prepare(class);
    }

    fn verify(class: &Rc<RefCell<Class>>) {}

    pub fn load_class_by_bytes(
        java_loader: Rc<RefCell<Object>>,
        class_name: &str,
        protection_domain: Option<Rc<RefCell<Object>>>,
        bytes: Vec<u8>,
        byte_array: Option<Rc<RefCell<Object>>>,
    ) -> Rc<RefCell<Class>> {
        let loader = (*java_loader).borrow().get_class_loader();
        let clone_loader = loader.clone();
        let class_op: Option<Rc<RefCell<Class>>> = (*clone_loader).borrow().get_class(class_name);
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }

        let method = JavaLangInstrument::instance().get_transform_method();
        let instrument = JavaLangInstrument::instance().get_instrument();
        let params = vec![
            Parameter::Object(Some(instrument)),
            Parameter::Object(Some(java_loader.clone())),
            Parameter::Object(Some(StringPool::java_string(
                Jvm::instance().unwrap().boot_class_loader(),
                class_name.to_string(),
            ))),
            Parameter::Object(None),
            Parameter::Object(protection_domain),
            Parameter::Object(byte_array),
            Parameter::Boolean(false),
        ];
        let rs = invoke(
            method,
            Parameters::with_parameters(params),
            ReturnType::Object,
        );

        let mut class: Option<Rc<RefCell<Class>>> = None;
        if class_name.starts_with('[') {
            class = Some(ClassLoader::load_array_class(loader.clone(), class_name));
        } else {
            class = Some(ClassLoader::load_non_array_class_by_bytes(
                java_loader,
                class_name,
                bytes,
            ));
        }
        let value = class.unwrap();
        let class_class = (*loader).borrow().get_class("java/lang/Class");
        if class_class.is_some() {
            let mut class_object = Class::new_object(&class_class.unwrap());
            class_object.set_meta(value.clone());
            let boxed = boxed(class_object);
            (*value).borrow_mut().set_java_class(Some(boxed));
        }
        return value;
    }

    fn load_non_array_class_by_bytes(
        java_loader: Rc<RefCell<Object>>,
        class_name: &str,
        bytes: Vec<u8>,
    ) -> Rc<RefCell<Class>> {
        let loader = (*java_loader).borrow().get_class_loader();
        let class = ClassLoader::define_class_by_java(java_loader, bytes);
        ClassLoader::link(&class);
        if (*loader).borrow().verbose_class {
            //            println!("Loaded {}.class from {}", class_name, entry.to_string());
        }
        return class;
    }

    pub fn define_class_by_java(
        java_loader: Rc<RefCell<Object>>,
        data: Vec<u8>,
    ) -> Rc<RefCell<Class>> {
        let loader = (*java_loader).borrow().get_class_loader();
        let mut class = ClassLoader::parse_class(data);
        (*class).borrow_mut().set_class_loader(loader.clone());
        ClassLoader::resolve_super_class_by_java(java_loader.clone(), class.clone());
        ClassLoader::resolve_interfaces_by_java(java_loader, class.clone());
        (*loader)
            .borrow_mut()
            .class_map
            .insert((*class).borrow().name().to_string(), class.clone());
        return class;
    }

    pub fn load_class_by_java(loader: Rc<RefCell<Object>>, class_name: &str) -> Rc<RefCell<Class>> {
        let class_loader = (*loader).borrow().get_class_loader();
        let class_op: Option<Rc<RefCell<Class>>> = (*class_loader).borrow().get_class(class_name);
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }
        let mut class: Option<Rc<RefCell<Class>>> = None;
        if class_name.starts_with('[') {
            class = Some(ClassLoader::load_array_class(
                class_loader.clone(),
                class_name,
            ));
        } else {
            let loader_class = (*loader).borrow().class();
            let method = Class::get_instance_method(
                loader_class,
                "loadClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
            );
            let main_thread = boxed(JavaThread::new_thread());
            let mut dummy_frame =
                JavaThread::new_frame(main_thread.clone(), method.clone().unwrap());
            let mut frame = JavaThread::new_frame(main_thread.clone(), method.unwrap());
            let vars = frame.local_vars().expect("LocalVars is none");
            vars.set_this(Some(loader.clone()));
            vars.set_ref(
                1,
                Some(StringPool::java_string(
                    unsafe { JVM.as_ref().unwrap().boot_class_loader() },
                    class_name.to_string(),
                )),
            );
            (*main_thread).borrow_mut().push_frame(dummy_frame);
            (*main_thread).borrow_mut().push_frame(frame);
            let class_obj = invoke_java_method(main_thread.clone()).unwrap();
            let raw_class = (*class_obj).borrow().meta();
            class = raw_class;
        }
        let value = class.unwrap();
        let class_class = (*class_loader).borrow().get_class("java/lang/Class");
        if class_class.is_some() {
            let mut class_object = Class::new_object(&class_class.unwrap());
            class_object.set_meta(value.clone());
            let boxed = boxed(class_object);
            (*value).borrow_mut().set_java_class(Some(boxed));
        }
        return value;
    }

    pub fn load_non_array_class_by_java(
        loader: Rc<RefCell<ClassLoader>>,
        class_name: &str,
    ) -> Rc<RefCell<Class>> {
        let (bytes, entry) = (*loader).borrow().read_class(class_name);
        let class = ClassLoader::define_class(loader.clone(), bytes);
        ClassLoader::link(&class);
        if (*loader).borrow().verbose_class {
            println!("Loaded {}.class from {}", class_name, entry.to_string());
        }
        return class;
    }

    fn resolve_super_class_by_java(java_loader: Rc<RefCell<Object>>, class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let super_class_name = class.super_class_name();
        //        println!("resolve_super_class:{:?},super:{:?}",class.name(),super_class_name);
        if class.name() != "java/lang/Object" && super_class_name.is_some() {
            let super_class =
                Self::load_class_by_java(java_loader.clone(), super_class_name.unwrap().as_str());
            class.set_super_class(super_class);
        }
    }
    fn resolve_interfaces_by_java(java_loader: Rc<RefCell<Object>>, class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let interfaces_name = class.interfaces_name();
        let len = interfaces_name.len();
        if len > 0 {
            let mut interfaces = Vec::with_capacity(len);
            for name in interfaces_name {
                let interface = Self::load_class_by_java(java_loader.clone(), name);
                interfaces.push(interface);
            }
            class.set_interfaces(interfaces);
        }
    }
}

impl Debug for ClassLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}
