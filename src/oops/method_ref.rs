use crate::oops::class::{Class, Interfaces, MethodType};
use crate::oops::member_ref::MemberRef;
use crate::oops::method::Method;
use lark_classfile::constant_pool::ConstantMethodRefInfo;
use std::sync::RwLock;

pub struct MethodRef {
    member_ref: MemberRef,
    method: RwLock<Option<Method>>,
}

impl MethodRef {
    #[inline]
    pub fn new_method_ref(info: &ConstantMethodRefInfo) -> MethodRef {
        let mut field_ref = MethodRef {
            member_ref: MemberRef::new(),
            method: RwLock::new(None),
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }

    #[inline]
    pub fn name(&self) -> &str {
        return self.member_ref.name();
    }

    #[inline]
    pub fn descriptor(&self) -> &str {
        return self.member_ref.descriptor();
    }

    #[inline]
    pub fn resolved_class(&self, holder: &Class) -> Class {
        return self.member_ref.resolved_class(holder);
    }

    pub fn resolved_method(&self, holder: &Class) -> Method {
        let method_op = {
            let method = self.method.read().unwrap();
            method.clone()
        };
        match method_op {
            Some(method) => method,
            None => self.resolved_method_ref(holder)
        }
    }

    fn resolved_method_ref(&self, holder: &Class) -> Method {
        let class = self.member_ref.resolved_class(holder);
        if class.is_interface() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let method = MethodRef::look_up_method(&class, self.name(), self.descriptor());
        if method.is_none() {
            panic!("java.lang.NoSuchMethodError");
        }
        if !(method.as_ref().unwrap()).is_accessible_to(&class) {
            panic!("java.lang.IllegalAccessError");
        }
        let mut raw = self.method.write().unwrap();
        *raw = method.clone();
        method.unwrap()
    }

    pub fn look_up_method(class: &Class, name: &str, desc: &str) -> Option<Method> {
        let mut method = MethodRef::look_up_method_in_class(class, name, desc);
        if method.is_none() {
            method = class.interfaces_with(|is|{
                MethodRef::look_up_method_in_interfaces(
                    is.unwrap(),
                    name,
                    desc,
                )
            });
        }
        return method;
    }

    pub fn look_up_method_in_class(
        class: &Class,
        name: &str,
        desc: &str,
    ) -> Option<Method> {
        let mut super_class = Some(class.clone());
        while super_class.is_some() {
            let value = super_class.unwrap();
            if let Some(m) = value.find_method(name,desc,MethodType::Unlimited){
                return Some(m)
            }
            super_class = value.super_class();
        }
        return None;
    }

    pub fn look_up_method_in_interfaces(
        interfaces: &Interfaces,
        name: &str,
        desc: &str,
    ) -> Option<Method> {
        for interface in interfaces {
            if let Some(m) = interface.find_method(name,desc,MethodType::Unlimited){
                return Some(m)
            }
            let method = interface.interfaces_with(|is|{
                MethodRef::look_up_method_in_interfaces(
                    is.unwrap(),
                    name,
                    desc,
                )
            });
            if method.is_some() {
                return method;
            }
        }
        return None;
    }
}
