use crate::class_file::constant_pool::ConstantMethodRefInfo;
use crate::runtime_data_area::heap::class::{Class, Interfaces};
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::member_ref::MemberRef;
use crate::runtime_data_area::heap::method::Method;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct MethodRef {
    member_ref: MemberRef,
    method: Option<Rc<Method>>,
}

impl MethodRef {
    #[inline]
    pub fn new_method_ref(info: &ConstantMethodRefInfo) -> MethodRef {
        let mut field_ref = MethodRef {
            member_ref: MemberRef::new(),
            method: None,
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
    pub fn resolved_class(&mut self) -> Rc<RefCell<Class>> {
        return self.member_ref.resolved_class();
    }

    pub fn resolved_method(&mut self) -> Option<Rc<Method>> {
        if self.method.is_none() {
            self.resolved_method_ref();
        }
        return self.method.clone();
    }

    fn resolved_method_ref(&mut self) {
        let class = self.member_ref.resolved_class();
        if (*class).borrow().is_interface() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let method = MethodRef::look_up_method(class.clone(), self.name(), self.descriptor());
        if method.is_none() {
            panic!("java.lang.NoSuchMethodError");
        }
        if !(*method.clone().unwrap()).is_accessible_to((*class).borrow().deref()) {
            panic!("java.lang.IllegalAccessError");
        }
        self.method = method;
    }

    pub fn look_up_method(class: Rc<RefCell<Class>>, name: &str, desc: &str) -> Option<Rc<Method>> {
        let mut method = MethodRef::look_up_method_in_class(class.clone(), name, desc);
        if method.is_none() {
            method = MethodRef::look_up_method_in_interfaces(
                (*class).borrow().interfaces().unwrap(),
                name,
                desc,
            );
        }
        return method;
    }

    pub fn look_up_method_in_class(
        class: Rc<RefCell<Class>>,
        name: &str,
        desc: &str,
    ) -> Option<Rc<Method>> {
        let mut super_class = Some(class);
        while super_class.is_some() {
            let value = super_class.unwrap().clone();
            let borrow_value = (*value).borrow();
            let methods = borrow_value.methods();
            for method in methods {
                if method.name() == name && method.descriptor() == desc {
                    return Some(method.clone());
                }
            }
            super_class = borrow_value.super_class();
        }
        return None;
    }

    pub fn look_up_method_in_interfaces(
        interfaces: &Interfaces,
        name: &str,
        desc: &str,
    ) -> Option<Rc<Method>> {
        for interface in interfaces {
            let borrow = (**interface).borrow();
            let methods = borrow.methods();
            for method in methods {
                if method.name() == name && method.descriptor() == desc {
                    return Some(method.clone());
                }
            }
            let method = MethodRef::look_up_method_in_interfaces(
                (**interface).borrow().interfaces().unwrap(),
                name,
                desc,
            );
            if method.is_some() {
                return method;
            }
        }
        return None;
    }

    #[inline]
    pub fn set_holder(&mut self, holder: Rc<RefCell<Class>>) {
        self.member_ref.set_holder(holder);
    }
}
