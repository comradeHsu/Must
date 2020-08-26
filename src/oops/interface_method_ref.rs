use crate::oops::class::{Class, MethodType};
use crate::oops::member_ref::MemberRef;
use crate::oops::method::Method;
use crate::oops::method_ref::MethodRef;
use lark_classfile::constant_pool::ConstantInterfaceMethodRefInfo;
use std::sync::RwLock;

pub struct InterfaceMethodRef {
    member_ref: MemberRef,
    method: RwLock<Option<Method>>,
}

impl InterfaceMethodRef {
    pub fn new_method_ref(info: &ConstantInterfaceMethodRefInfo) -> InterfaceMethodRef {
        let mut field_ref = InterfaceMethodRef {
            member_ref: MemberRef::new(),
            method: RwLock::new(None),
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }

    pub fn resolved_interface_method(&self, holder: &Class ) -> Method {
        let method_op = {
            let method = self.method.read().unwrap();
            method.clone()
        };
        match method_op {
            Some(method) => method,
            None => self.resolved_interface_method_ref(holder)
        }
    }

    pub fn resolved_interface_method_ref(&self, holder: &Class) -> Method {
        let class = self.member_ref.resolved_class(holder);
        if !class.is_interface() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let method = Self::look_up_interface_method(&class, self.name(), self.descriptor());
        if method.is_none() {
            panic!("java.lang.NoSuchMethodError");
        }
        let real_method = method.unwrap();
        if !real_method.is_accessible_to(&class) {
            panic!("java.lang.IllegalAccessError");
        }
        let mut raw = self.method.write().unwrap();
        *raw = Some(real_method.clone());
        real_method
    }

    pub fn look_up_interface_method(
        class: &Class,
        name: &str,
        desc: &str,
    ) -> Option<Method> {
        if let Some(m) = class.find_method(name,desc,MethodType::Unlimited){
            return Some(m)
        }
        class.interfaces_with(|is|{
            MethodRef::look_up_method_in_interfaces(
                is.unwrap(),
                name,
                desc,
            )
        })
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
}
