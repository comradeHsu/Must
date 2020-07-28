use crate::class_file::constant_pool::ConstantInterfaceMethodRefInfo;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::member_ref::MemberRef;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::method_ref::MethodRef;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct InterfaceMethodRef {
    member_ref: MemberRef,
    method: Option<Rc<Method>>,
}

impl InterfaceMethodRef {
    pub fn new_method_ref(info: &ConstantInterfaceMethodRefInfo) -> InterfaceMethodRef {
        let mut field_ref = InterfaceMethodRef {
            member_ref: MemberRef::new(),
            method: None,
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }

    pub fn resolved_interface_method(&mut self,holder:Rc<RefCell<Class>>) -> Option<Rc<Method>> {
        if self.method.is_none() {
            self.resolved_interface_method_ref(holder);
        }
        return self.method.clone();
    }

    pub fn resolved_interface_method_ref(&mut self,holder:Rc<RefCell<Class>>) {
        let class = self.member_ref.resolved_class(holder);
        if !(*class).borrow().is_interface() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let method = Self::look_up_interface_method(
            class.clone(),
            self.name(),
            self.descriptor(),
        );
        if method.is_none() {
            panic!("java.lang.NoSuchMethodError");
        }
        let point = method.clone().unwrap();
        if !(*point).is_accessible_to((*class).borrow().deref()) {
            panic!("java.lang.IllegalAccessError");
        }
        self.method = method;
    }

    pub fn look_up_interface_method(
        class: Rc<RefCell<Class>>,
        name: &str,
        desc: &str,
    ) -> Option<Rc<Method>> {
        let borrow = class.borrow();
        let methods = borrow.methods();
        for method in methods {
            if method.name() == name && method.descriptor() == desc {
                return Some(method.clone());
            }
        }
        return MethodRef::look_up_method_in_interfaces(
            (*class).borrow().interfaces().unwrap(),
            name,
            desc,
        );
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
    pub fn resolved_class(&mut self,holder:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        return self.member_ref.resolved_class(holder);
    }
}
