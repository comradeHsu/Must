use lark_classfile::class_file::ClassFile;
use crate::instrument::java_lang_instrument::JavaLangInstrument;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{ReturnType, JavaCall};
use crate::jvm::Jvm;
use crate::oops::class::Class;
use crate::oops::object::Object;
use crate::oops::string_pool::StringPool;
use crate::utils::{boxed, java_str_to_rust_str};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;
use crate::class_loader::class_init_preparation::ClassPreparation;

pub struct ClassLoader {
    pub(in crate::class_loader) verbose_class: bool,
    pub(in crate::class_loader) class_map: HashMap<String, Rc<RefCell<Class>>>,
}

impl ClassLoader {
    #[inline]
    pub fn new() -> ClassLoader {
        return ClassLoader {
            verbose_class: false,
            class_map: Default::default(),
        };
    }

    #[inline]
    pub fn with_verbose(verbose: bool) -> ClassLoader {
        return ClassLoader {
            verbose_class: verbose,
            class_map: Default::default(),
        };
    }

    #[inline]
    pub fn find_class(&self, name: &str) -> Option<Rc<RefCell<Class>>> {
        let rs = self.class_map.get(name);
        match rs {
            Some(r) => return Some(r.clone()),
            None => None,
        }
    }

    #[inline]
    pub fn class_map_immutable(&self) -> &HashMap<String, Rc<RefCell<Class>>> {
        return &self.class_map;
    }

    fn link(class: &Rc<RefCell<Class>>) {
        ClassLoader::verify(class);
        ClassPreparation::prepare(class);
    }

    fn verify(class: &Rc<RefCell<Class>>) {}

    pub fn define_class_internal(
        class_name: &str,
        mut byte_array: Option<Rc<RefCell<Object>>>,
        offset: usize,
        length: usize,
        class_loader: Rc<RefCell<Object>>,
        protection_domain: Option<Rc<RefCell<Object>>>,
    ) -> Rc<RefCell<Class>> {
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

    fn define_class(java_loader: Rc<RefCell<Object>>, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let loader = (*java_loader).borrow().get_class_loader();
        let class = Self::parse_class(data);
        (*class).borrow_mut().set_class_loader(loader.clone());
        Self::resolve_super_class(java_loader.clone(), class.clone());
        Self::resolve_interfaces(java_loader.clone(), class.clone());
        Self::link(&class);
        (*loader)
            .borrow_mut()
            .class_map
            .insert((*class).borrow().name().to_string(), class.clone());
        Self::setting_class_object(Some(java_loader),class.clone());
        return class;
    }

    pub(in crate::class_loader) fn parse_class(data: Vec<u8>) -> Rc<RefCell<Class>> {
        let class_file = ClassFile::parse(data);
        return Class::new(class_file);
    }

    fn extract_data(byte_array: Rc<RefCell<Object>>, offset: usize, length: usize) -> Vec<u8> {
        let mut borrow = (*byte_array).borrow_mut();
        let mut_bytes = borrow.mut_bytes();
        let slice = &mut_bytes[offset..(offset + length)];
        let mut bytes = vec![0u8; length];
        for i in 0..length {
            bytes[i] = slice[i] as u8;
        }
        return bytes;
    }

    fn resolve_super_class(java_loader: Rc<RefCell<Object>>, class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let super_class_name = class.super_class_name();
        if class.name() != "java/lang/Object" && super_class_name.is_some() {
            let super_class = Self::load_class(
                Some(java_loader.clone()),
                super_class_name.unwrap().as_str(),
            );
            class.set_super_class(super_class);
        }
    }
    fn resolve_interfaces(java_loader: Rc<RefCell<Object>>, class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let interfaces_name = class.interfaces_name();
        let len = interfaces_name.len();
        if len > 0 {
            let mut interfaces = Vec::with_capacity(len);
            for name in interfaces_name {
                let interface = Self::load_class(Some(java_loader.clone()), name);
                interfaces.push(interface);
            }
            class.set_interfaces(interfaces);
        }
    }

    pub fn load_class(
        loader_object: Option<Rc<RefCell<Object>>>,
        class_name: &str,
    ) -> Rc<RefCell<Class>> {
        if loader_object.is_none() {
            let bootstrap_loader = Jvm::boot_class_loader();
            let class =  bootstrap_loader.find_or_create(class_name);
            if class.is_none() {
                println!("class not found {}",class_name);
            }
            return class.unwrap();
        }
        let loader = loader_object.unwrap();
        let class_loader = (*loader).borrow().get_class_loader();
        let class_op: Option<Rc<RefCell<Class>>> = (*class_loader).borrow().find_class(class_name);
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }
        let mut class: Option<Rc<RefCell<Class>>> = None;
        if class_name.starts_with('[') {
            class = Some(Self::load_array_class(class_loader.clone(), class_name));
        } else {
            class = Self::invoke_load_class(loader.clone(), class_name.replace('/', ".").as_str());
        }
        let value = class.unwrap();
        //Self::setting_class_object(Some(loader),value.clone());
        return value;
    }

    pub(in crate::class_loader) fn setting_class_object(
        loader_object: Option<Rc<RefCell<Object>>>,
        value: Rc<RefCell<Class>>) {
        let boot_loader = Jvm::boot_class_loader();
        let class_class = boot_loader.find_class("java/lang/Class");
        if class_class.is_some() {
            let class_of_class = class_class.unwrap();
            let mut class_object = Class::new_object(&class_of_class);
            class_object.set_meta(value.clone());
            let constructor_desc = "(Ljava/lang/ClassLoader;)V";
            let constructor = Class::get_constructor(class_of_class.clone(), constructor_desc);
            let object = Some(boxed(class_object));
            let parameters = vec![
                Parameter::Object(object.clone()),
                Parameter::Object(loader_object)
            ];
            JavaCall::invoke(
                constructor.unwrap(),
                Some(Parameters::with_parameters(parameters)),
                ReturnType::Void,
            );
            (*value).borrow_mut().set_java_class(object);
        }
    }

    fn invoke_load_class(
        loader: Rc<RefCell<Object>>,
        class_name: &str,
    ) -> Option<Rc<RefCell<Class>>> {
        let loader_class = (*loader).borrow().class();
        let method = Class::get_instance_method(
            loader_class,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
        );
        let java_name = StringPool::java_string(class_name.to_string());
        let params = Parameters::with_parameters(vec![
            Parameter::Object(Some(loader)),
            Parameter::Object(Some(java_name)),
        ]);
        let return_value = JavaCall::invoke(method.unwrap(), Some(params), ReturnType::Object).object();
        return (*return_value.unwrap()).borrow().meta();
    }

    ///load array's class
    pub(in crate::class_loader) fn load_array_class(
        loader: Rc<RefCell<Self>>,
        class_name: &str,
    ) -> Rc<RefCell<Class>> {
        let class = Class::new_array_class(loader.clone(), class_name);
        let class_ptr = boxed(class);
        (*loader)
            .borrow_mut()
            .class_map
            .insert(class_name.to_string(), class_ptr.clone());
        return class_ptr;
    }
}

impl Debug for ClassLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}
