use std::rc::Rc;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::member_info::MemberInfo;
use crate::runtime_data_area::heap::access_flags::{PUBLIC, FINAL, PRIVATE, PROTECTED, STATIC, SYNTHETIC};
use std::cell::RefCell;
use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Debug)]
pub struct ClassMember {
    access_flags:u16,
    name:String,
    descriptor:String,
    class:Rc<RefCell<Class>>
}

impl ClassMember {

    #[inline]
    pub fn new() -> ClassMember {
        return ClassMember{
            access_flags: 0,
            name: "".to_string(),
            descriptor: "".to_string(),
            class: Rc::new(RefCell::new(Class::none()))
        };
    }

    pub fn copy_member_info(&mut self,info:&MemberInfo) {
        self.access_flags = info.access_flags();
        self.name = info.name().to_string();
        self.descriptor = info.descriptor().to_string();
    }

    #[inline]
    pub fn set_class(&mut self,class:Rc<RefCell<Class>>) {
        self.class = class;
    }

    #[inline]
    pub fn descriptor(&self) -> &str{
        return self.descriptor.as_str();
    }

    #[inline]
    pub fn name(&self) -> &str{
        return self.name.as_str();
    }

    #[inline]
    pub fn class(&self) -> Rc<RefCell<Class>> {
        return self.class.clone();
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        return 0 != self.access_flags & PUBLIC;
    }

    #[inline]
    pub fn is_private(&self) -> bool {
        return 0 != self.access_flags & PRIVATE;
    }

    #[inline]
    pub fn is_protected(&self) -> bool {
        return 0 != self.access_flags & PROTECTED;
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        return 0 != self.access_flags & STATIC;
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        return 0 != self.access_flags & FINAL;
    }

    #[inline]
    pub fn is_synthetic(&self) -> bool {
        return 0 != self.access_flags & SYNTHETIC;
    }

    pub fn is_accessible_to(&self, class:&Class) -> bool {
        if self.is_public() {
            return true;
        }
        let o = self.class.clone();
        let other = (*o).borrow();
        if self.is_protected() {
            return class == other.deref() || class.is_sub_class_of(other.deref()) ||
                other.package_name() == class.package_name();
        }
        if !self.is_private() {
            return other.package_name() == class.package_name();
        }
        return class == other.deref();
    }
}