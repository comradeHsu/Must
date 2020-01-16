use std::rc::Rc;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::slots::Slots;
use crate::class_file::class_file::ClassFile;
use crate::runtime_data_area::heap::access_flags::{AccessFlag, PUBLIC, FINAL, SUPER, INTERFACE,
                                                   ABSTRACT, SYNTHETIC, ANNOTATION, ENUM};
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::slot::Slot;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use std::cell::RefCell;
use crate::runtime_data_area::heap::object::Object;

type Interfaces = Vec<Rc<RefCell<Class>>>;

pub struct Class {
    access_flags:u16,
    name:String,
    super_class_name:String,
    interfaces_name:Vec<&'static str>,
    constant_pool:Rc<ConstantPool>,
    fields:Vec<Rc<RefCell<Field>>>,
    methods:Vec<Method>,
    loader:Rc<RefCell<ClassLoader>>,
    super_class:Option<Rc<RefCell<Class>>>,
    interfaces:Option<Interfaces>,
    instance_slot_count:u32,
    static_slot_count:u32,
    static_vars:Option<Slots>
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
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None
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
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None
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

    pub fn is_accessible_to(&self,other:&Self) -> bool {
        return self.is_public() ||
            self.package_name() == other.package_name();
    }

    pub fn package_name(&self) -> &str {
        let index = self.name.rfind('/');
        let name = match index {
            Some(seq) => {
                let (package,_) = self.name.split_at(seq);
                package
            }
            None => ""
        };
        return name;
    }

    // self extends c
    pub fn is_sub_class_of(&self, other:&Self) -> bool {
        let mut super_class = self.super_class.as_ref();
        while super_class.is_some() {
            if other == (*super_class.unwrap()).borrow() {
                return true;
            }
            super_class = (*super_class.unwrap()).borrow().super_class.as_ref();
        }
        return false
    }

    #[inline]
    pub fn new_object(class:&Rc<RefCell<Class>>) -> Object {
        return Object::new(class);
    }

    #[inline]
    pub fn set_class_loader(&mut self,class_loader:Rc<RefCell<ClassLoader>>) {
        self.loader = class_loader;
    }

    #[inline]
    pub fn set_super_class(&mut self,super_class:Rc<RefCell<Class>>) {
        self.super_class = Some(super_class);
    }

    #[inline]
    pub fn set_interfaces(&mut self,interfaces:Interfaces) {
        self.interfaces = Some(interfaces);
    }

    #[inline]
    pub fn set_instance_slot_count(&mut self,count:u32) {
        self.instance_slot_count = count;
    }

    #[inline]
    pub fn set_static_slot_count(&mut self,count:u32) {
        self.static_slot_count = count;
    }

    #[inline]
    pub fn set_static_vars(&mut self,vars:Slots) {
        self.static_vars = Some(vars);
    }

    #[inline]
    pub fn name(&self) -> &str{
        return self.name.as_str();
    }

    #[inline]
    pub fn super_class_name(&self) -> &str{
        return self.super_class_name.as_str();
    }

    #[inline]
    pub fn interfaces_name(&self) -> &Vec<&str> {
        return &self.interfaces_name;
    }

    #[inline]
    pub fn loader(&self) -> Rc<RefCell<ClassLoader>>{
        return self.loader.clone();
    }

    #[inline]
    pub fn super_class(&self) -> Option<&Rc<RefCell<Class>>>{
        if self.super_class.is_some() {
            return self.super_class.as_ref();
        }
        return None;
    }

    #[inline]
    pub fn instance_slot_count(&self) -> u32 {
        return self.instance_slot_count;
    }

    #[inline]
    pub fn static_slot_count(&self) -> u32 {
        return self.static_slot_count;
    }

    #[inline]
    pub fn fields(&self) -> &Vec<Rc<RefCell<Field>>> {
        return &self.fields;
    }

    #[inline]
    pub fn interfaces(&self) -> &Option<Interfaces> {
        return &self.interfaces;
    }

    #[inline]
    pub fn constant_pool(&self) -> Rc<ConstantPool> {
        return self.constant_pool.clone();
    }

    #[inline]
    pub fn mut_fields(&mut self) -> &mut Vec<Rc<RefCell<Field>>> {
        return &mut self.fields;
    }

    #[inline]
    pub fn mut_static_vars(&mut self) -> Option<&mut Slots> {
        return self.static_vars.as_mut();
    }
}