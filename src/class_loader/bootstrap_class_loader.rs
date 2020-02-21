use std::rc::Rc;
use crate::class_path::class_path::{ClassPath, Entry};
use std::cell::RefCell;
use crate::class_loader::app_class_loader::ClassLoader;
use crate::utils::boxed;
use crate::runtime_data_area::heap::class::Class;
use crate::class_loader::class_linker::ClassLinker;
use std::collections::HashMap;
use crate::runtime_data_area::heap::class_name_helper::PrimitiveTypes;

pub struct BootstrapClassLoader {
    lib_path:Rc<ClassPath>,
    class_loader:Rc<RefCell<ClassLoader>>
}

impl BootstrapClassLoader {
    #[inline]
    pub fn new(path:Rc<ClassPath>) -> BootstrapClassLoader {
        let loader = ClassLoader::new();
        let boot = BootstrapClassLoader{
            lib_path: path,
            class_loader: boxed(loader)
        };
//        boot.load_basic_classes();
//        boot.load_primitive_classes();
        return boot;
    }

    #[inline]
    pub fn post_constructor(&self) {
        self.load_basic_classes();
        self.load_primitive_classes();
    }

    fn load_basic_classes(&self) {
        let java_lang_class =self.find_or_create("java/lang/Class");
        let borrow = (*self.class_loader).borrow();
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

    fn load_primitive_classes(&self) {
        let primitives = PrimitiveTypes::instance().unwrap();
        let maps = primitives.primitive_types();
        for (k, v) in maps {
            self.load_primitive_class( k);
        }
    }

    fn load_primitive_class(&self, class_name: &str) {
        let mut class = Class::primitive_class(class_name);
        let class_class = self.find_or_create("java/lang/Class");
        let mut class_object = Class::new_object(&class_class);
        let boxed_class = boxed(class);
        class_object.set_meta(boxed_class.clone());
        (*boxed_class)
            .borrow_mut()
            .set_java_class(Some(boxed(class_object)));
        (*self.class_loader)
            .borrow_mut()
            .class_map
            .insert(class_name.to_string(), boxed_class);
    }

    pub fn find_or_create(&self,class_name:&str) -> Rc<RefCell<Class>> {
        let class_op: Option<Rc<RefCell<Class>>> = (*self.class_loader).borrow().find_class(class_name);
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }
        let mut class: Option<Rc<RefCell<Class>>> = None;
        if class_name.starts_with('[') {
            class = Some(ClassLoader::load_array_class(self.class_loader.clone(), class_name));
        } else {
            class = Some(self.load_non_array_class(class_name));
        }
        let value = class.unwrap();
        ClassLoader::setting_class_object(value.clone());
        return value;
    }

    fn load_non_array_class(
        &self,
        class_name: &str,
    ) -> Rc<RefCell<Class>> {
        let (bytes, entry) = self.read_class(class_name);
        let class = self.define_class(bytes);
        ClassLinker::link(&class);
        if (*self.class_loader).borrow().verbose_class {
            println!("Loaded {}.class from {}", class_name, entry.to_string());
        }
        return class;
    }

    fn read_class(&self, class_name: &str) -> (Vec<u8>, Box<dyn Entry>) {
        let result = self.lib_path.read_class(class_name);
        if result.is_err() {
            panic!("java.lang.ClassNotFoundException:{}", class_name);
        }
        return result.unwrap();
    }

    fn define_class(&self, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let mut class = ClassLoader::parse_class(data);
        (*class).borrow_mut().set_class_loader(self.class_loader.clone());
        self.resolve_super_class(class.clone());
        self.resolve_interfaces(class.clone());
        (*self.class_loader)
            .borrow_mut()
            .class_map
            .insert((*class).borrow().name().to_string(), class.clone());
        return class;
    }

    fn resolve_super_class(&self,class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let super_class_name = class.super_class_name();
        //        println!("resolve_super_class:{:?},super:{:?}",class.name(),super_class_name);
        if class.name() != "java/lang/Object" && super_class_name.is_some() {
            let super_class =
                self.find_or_create(super_class_name.unwrap().as_str());
            class.set_super_class(super_class);
        }
    }

    fn resolve_interfaces(&self,class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let interfaces_name = class.interfaces_name();
        let len = interfaces_name.len();
        if len > 0 {
            let mut interfaces = Vec::with_capacity(len);
            for name in interfaces_name {
                let interface = self.find_or_create(name);
                interfaces.push(interface);
            }
            class.set_interfaces(interfaces);
        }
    }

    #[inline]
    pub fn basic_loader(&self) -> Rc<RefCell<ClassLoader>> {
        return self.class_loader.clone();
    }

    #[inline]
    pub fn find_class(&self,class_name:&str) -> Option<Rc<RefCell<Class>>> {
        return (*self.class_loader).borrow().find_class(class_name);
    }
}