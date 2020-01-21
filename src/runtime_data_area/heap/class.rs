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
use core::mem;
use std::ops::Deref;

pub type Interfaces = Vec<Rc<RefCell<Class>>>;

#[derive(Debug)]
pub struct Class {
    access_flags:u16,
    name:String,
    super_class_name:Option<String>,
    interfaces_name:Vec<String>,
    constant_pool:Rc<RefCell<ConstantPool>>,
    fields:Vec<Rc<RefCell<Field>>>,
    methods:Vec<Rc<Method>>,
    loader:Option<Rc<RefCell<ClassLoader>>>,
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
            super_class_name: None,
            interfaces_name: vec![],
            constant_pool: Rc::new(RefCell::new(ConstantPool::none())),
            fields: vec![],
            methods: vec![],
            loader: None,
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None
        };
    }

    #[inline]
    pub fn new(class_file:ClassFile) -> Rc<RefCell<Class>> {
        let super_name = class_file.super_class_name();
        let mut class = Class{
            access_flags: class_file.access_flags(),
            name: class_file.class_name().to_string(),
            super_class_name: super_name,
            interfaces_name: class_file.interface_names(),
            constant_pool: ConstantPool::new_constant_pool(None,class_file.constant_pool()),
            fields: vec![],
            methods: vec![],
            loader: None,
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None
        };
        println!("class:{:?}",class.name.as_str());
        let mut point = Rc::new(RefCell::new(class));
        (*point).borrow_mut().constant_pool.borrow_mut().set_class(point.clone());
        (*point).borrow_mut().methods = Method::new_methods(point.clone(),class_file.methods());
        (*point).borrow_mut().fields = Field::new_fields(point.clone(),class_file.fields());
        return point;
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
        let mut super_class = self.super_class.clone();
        while super_class.is_some() {
            let rc = super_class.unwrap();
            let rc_super_class = (*rc).borrow();
            if other == rc_super_class.deref() {
                return true;
            }
            super_class = rc_super_class.super_class.clone();
        }
        return false
    }

    pub fn is_assignable_from(&self, other:&Self) -> bool {
        if self == other {
            return true
        }
        if !self.is_interface() {
            return other.is_sub_class_of(self);
        } else {
            return other.is_implements(self);
        }
    }

    // self implements iface
    pub fn is_implements(&self, interface: &Self) -> bool {
        let mut super_class = self.super_class.clone();
        while super_class.is_some() {
            let rc = super_class.unwrap();
            let ref_class = (*rc).borrow();
            let interfaces = ref_class.interfaces.as_ref().unwrap();
            for i in interfaces {
                let interface_class = (*i).borrow();
                if interface_class.deref() == interface || interface_class.is_sub_interface_of(interface){
                    return true;
                }
            }
            super_class = ref_class.super_class.clone();
        }
        return false
    }

    ///
    pub fn is_sub_interface_of(&self, other:&Self) -> bool {
        let interfaces = self.interfaces.as_ref().unwrap();
        for interface in interfaces {
            let interface = interface.clone();
            if (*interface).borrow().deref() == other || (*interface).borrow().is_sub_interface_of(other){
                return true;
            }
        }
        return false
    }

    pub fn get_main_method(&self) -> Option<Rc<Method>> {
        for method in self.methods() {
            let method = method.clone();
            if  method.name() == "main" && method.descriptor() ==  "([Ljava/lang/String;)V" {
                return Some(method);
            }
        }
        return None;
    }

    #[inline]
    pub fn new_object(class:&Rc<RefCell<Class>>) -> Object {
        return Object::new(class.clone());
    }

    #[inline]
    pub fn set_class_loader(&mut self,class_loader:Rc<RefCell<ClassLoader>>) {
        self.loader = Some(class_loader);
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
    pub fn super_class_name(&self) -> Option<&String>{
        return self.super_class_name.as_ref();
    }

    #[inline]
    pub fn interfaces_name(&self) -> &Vec<String> {
        return &self.interfaces_name;
    }

    #[inline]
    pub fn loader(&self) -> Rc<RefCell<ClassLoader>>{
        let loader = self.loader.as_ref().unwrap();
        return loader.clone();
    }

    #[inline]
    pub fn super_class(&self) -> Option<Rc<RefCell<Class>>>{
        if self.super_class.is_some() {
            return self.super_class.clone();
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
    pub fn interfaces(&self) -> Option<&Interfaces> {
        return self.interfaces.as_ref();
    }

    #[inline]
    pub fn methods(&self) -> &Vec<Rc<Method>> {
        return &self.methods;
    }

    #[inline]
    pub fn constant_pool(&self) -> Rc<RefCell<ConstantPool>> {
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

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        if self.name() == other.name() {
            return true;
        }
        return false;
    }
}

//impl PartialEq for &Class {
//    fn eq(&self, other: &Self) -> bool {
//        if self.name() == other.name() {
//            return true;
//        }
//        return false;
//    }
//}