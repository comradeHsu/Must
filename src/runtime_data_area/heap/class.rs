use std::rc::Rc;
use crate::class_file::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::slots::Slots;
use crate::class_file::class_file::ClassFile;
use crate::runtime_data_area::heap::access_flags::{AccessFlag, PUBLIC, FINAL, SUPER, INTERFACE, ABSTRACT, SYNTHETIC, ANNOTATION, ENUM};

pub struct Class {
    access_flags:u16,
    name:String,
    super_class_name:String,
    interfaces_name:Vec<&'static str>,
    constant_pool:Rc<ConstantPool>,
    fields:Vec<Field>,
    methods:Vec<Method>,
    loader:Rc<ClassLoader>,
    super_class:Rc<Class>,
    interfaces:Vec<Rc<Class>>,
    instance_slot_count:u32,
    static_slot_count:u32,
    static_vars:Slots
}

impl Class {

    #[inline]
    pub fn none() -> Class {
        return Class{
            access_flags: 0,
            name: "".to_string(),
            super_class_name: "".to_string(),
            interfaces_name: vec![],
            constant_pool: Rc::new(()),
            fields: vec![],
            methods: vec![],
            loader: Rc::new(()),
            super_class: Rc::new(Class {}),
            interfaces: vec![],
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: ()
        };
    }

    #[inline]
    pub fn new(class_file:ClassFile) -> Class {
        return Class{
            access_flags: class_file.access_flags(),
            name: class_file.class_name().to_string(),
            super_class_name: class_file.super_class_name().to_string(),
            interfaces_name: class_file.interface_names(),
            constant_pool: Rc::new(()),
            fields: vec![],
            methods: vec![],
            loader: Rc::new(()),
            super_class: Rc::new(Class {}),
            interfaces: vec![],
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: ()
        };
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        return 0 != self.access_flags & PUBLIC;
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        return 0 != self.access_flags & FINAL;
    }

    #[inline]
    pub fn is_super(&self) -> bool {
        return 0 != self.access_flags & SUPER;
    }

    #[inline]
    pub fn is_interface(&self) -> bool {
        return 0 != self.access_flags & INTERFACE;
    }

    #[inline]
    pub fn is_abstract(&self) -> bool {
        return 0 != self.access_flags & ABSTRACT;
    }

    #[inline]
    pub fn is_synthetic(&self) -> bool {
        return 0 != self.access_flags & SYNTHETIC;
    }

    #[inline]
    pub fn is_annotation(&self) -> bool {
        return 0 != self.access_flags & ANNOTATION;
    }

    #[inline]
    pub fn is_enum(&self) -> bool {
        return 0 != self.access_flags & ENUM;
    }
}