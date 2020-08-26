use crate::class_loader::class_init_preparation::ClassPreparation;
use crate::instrument::java_lang_instrument::JavaLangInstrument;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{JavaCall, ReturnType};
use crate::jvm::Jvm;
use crate::oops::class::Class;
use crate::oops::object::Object;
use crate::oops::string_pool::StringPool;
use crate::utils::{boxed};
use lark_classfile::class_file::ClassFile;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;
use std::sync::{Arc, RwLock};

struct Loader {
    pub(in crate::class_loader) verbose_class: bool,
    pub(in crate::class_loader) class_map: HashMap<String, Class>,
}

impl Loader {
    #[inline]
    pub fn new() -> Loader {
        return Loader {
            verbose_class: false,
            class_map: Default::default(),
        };
    }

    #[inline]
    pub fn with_verbose(verbose: bool) -> Loader {
        return Loader {
            verbose_class: verbose,
            class_map: Default::default(),
        };
    }
}

#[derive(Clone)]
pub struct ClassLoader {
    loader: Arc<RwLock<Loader>>
}

impl ClassLoader {
    #[inline]
    pub fn new() -> ClassLoader {
        return ClassLoader {
            loader: Arc::new(RwLock::new(Loader::new())),
        };
    }

    #[inline]
    pub fn with_verbose(verbose: bool) -> ClassLoader {
        return ClassLoader {
            loader: Arc::new(RwLock::new(Loader::with_verbose(verbose))),
        };
    }

    #[inline]
    pub fn find_class(&self, name: &str) -> Option<Class> {
        let inner = self.loader.read().unwrap();
        let rs = inner.class_map.get(name);
        match rs {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }

    pub fn insert_class(&self, name: String, class: Class) {
        let mut inner = self.loader.write().unwrap();
        inner.class_map.insert(name,class);
    }

    pub fn verbose(&self) -> bool {
        let inner = self.loader.read().unwrap();
        inner.verbose_class
    }

    #[inline]
    pub fn classes_with<F,R>(&self, fun: F) -> R
    where F:
        FnOnce(&HashMap<String,Class>) -> R
    {
        let inner = self.loader.read().unwrap();
        fun(&inner.class_map)
    }

    fn link(class: &Class) {
        Self::verify(&class);
        ClassPreparation::prepare(&class);
    }

    fn verify(_class: &Class) {}

    pub fn define_class_internal(
        class_name: &str,
        mut byte_array: Option<Object>,
        offset: usize,
        length: usize,
        class_loader: Object,
        protection_domain: Option<Object>,
    ) -> Class {
        let java_name = StringPool::java_string(class_name.to_string());
        let method = JavaLangInstrument::instance().get_transform_method();
        let instrument = JavaLangInstrument::instance().get_instrument();
        let params = vec![
            Parameter::Object(Some(instrument)),
            Parameter::Object(Some(class_loader.clone())),
            Parameter::Object(Some(java_name)),
            Parameter::Object(None),
            Parameter::Object(protection_domain),
            Parameter::Object(byte_array.clone()),
            Parameter::Boolean(false),
        ];
        let rs = JavaCall::invoke(
            method,
            Some(Parameters::with_parameters(params)),
            ReturnType::Object,
        )
        .object();
        if rs.is_some() {
            byte_array = rs;
        }
        let data = Self::extract_data(byte_array.unwrap(), offset, length);
        return Self::define_class(class_loader, data);
    }

    fn define_class(java_loader: Object, data: Vec<u8>) -> Class {
        let loader = java_loader.get_class_loader();
        let class = Self::parse_class(data);
        class.set_class_loader(loader.clone());
        Self::resolve_super_class(java_loader.clone(), &class);
        Self::resolve_interfaces(java_loader.clone(), &class);
        Self::link(&class);
        {
            let mut inner = loader.loader.write().unwrap();
            inner
                .class_map
                .insert(class.name().to_string(), class.clone());
        }
        Self::setting_class_object(Some(java_loader), &class);
        return class;
    }

    pub(in crate::class_loader) fn parse_class(data: Vec<u8>) -> Class {
        let class_file = ClassFile::parse(data);
        return Class::new(class_file);
    }

    fn extract_data(byte_array: Object, offset: usize, length: usize) -> Vec<u8> {
        byte_array.mut_bytes(|array| {
            let slice = &array[offset..(offset + length)];
            let mut bytes = vec![0u8; length];
            for i in 0..length {
                bytes[i] = slice[i] as u8;
            }
            return bytes;
        })
    }

    fn resolve_super_class(java_loader: Object, class: &Class) {
        let super_class_name = class.super_class_name();
        if class.name() != "java/lang/Object" && super_class_name.is_some() {
            let super_class = Self::load_class(
                Some(java_loader.clone()),
                super_class_name.unwrap().as_str(),
            );
            class.set_super_class(super_class);
        }
    }
    fn resolve_interfaces(java_loader: Object, class:&Class) {
        class.interfaces_name_with(|interfaces_name|{
            let len = interfaces_name.len();
            if len > 0 {
                let mut interfaces = Vec::with_capacity(len);
                for name in interfaces_name {
                    let interface = Self::load_class(Some(java_loader.clone()), name);
                    interfaces.push(interface);
                }
                class.set_interfaces(interfaces);
            }
        })
    }

    pub fn load_class(loader_object: Option<Object>, class_name: &str) -> Class {
        if loader_object.is_none() {
            let bootstrap_loader = Jvm::boot_class_loader();
            let class = bootstrap_loader.find_or_create(class_name);
            if class.is_none() {
                println!("class not found {}", class_name);
            }
            return class.unwrap();
        }
        let loader = loader_object.unwrap();
        let class_loader = loader.get_class_loader();
        let class_op: Option<Class> = class_loader.find_class(class_name);
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }
        let mut class: Option<Class> = None;
        if class_name.starts_with('[') {
            class = Some(class_loader.load_array_class(class_name));
        } else {
            class = Self::invoke_load_class(loader.clone(), class_name.replace('/', ".").as_str());
        }
        let value = class.unwrap();
        //Self::setting_class_object(Some(loader),value.clone());
        return value;
    }

    pub(in crate::class_loader) fn setting_class_object(
        loader_object: Option<Object>,
        value: &Class,
    ) {
        let boot_loader = Jvm::boot_class_loader();
        let class_class = boot_loader.find_class("java/lang/Class");
        if class_class.is_some() {
            let class_of_class = class_class.unwrap();
            let class_object = Class::new_object(&class_of_class);
            class_object.set_meta(value.clone());
            let constructor_desc = "(Ljava/lang/ClassLoader;)V";
            let constructor = class_of_class.get_constructor(constructor_desc);
            let object = Some(class_object);
            let parameters = vec![
                Parameter::Object(object.clone()),
                Parameter::Object(loader_object),
            ];
            JavaCall::invoke(
                constructor.unwrap(),
                Some(Parameters::with_parameters(parameters)),
                ReturnType::Void,
            );
            value.set_java_class(object);
        }
    }

    fn invoke_load_class(loader: Object, class_name: &str) -> Option<Class> {
        let loader_class = loader.class();
        let method = loader_class.get_instance_method(
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
        );
        let java_name = StringPool::java_string(class_name.to_string());
        let params = Parameters::with_parameters(vec![
            Parameter::Object(Some(loader)),
            Parameter::Object(Some(java_name)),
        ]);
        let return_value =
            JavaCall::invoke(method.unwrap(), Some(params), ReturnType::Object).object();
        return Some(return_value.unwrap().meta());
    }

    ///load array's class
    pub(in crate::class_loader) fn load_array_class(
        &self,
        class_name: &str,
    ) -> Class {
        let class = Class::new_array_class(&self, class_name);
        let mut inner = self.loader.write().unwrap();
        inner
            .class_map
            .insert(class_name.to_string(), class.clone());
        return class;
    }
}

impl Debug for ClassLoader {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}
