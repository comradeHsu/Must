use crate::runtime_data_area::heap::member_ref::MemberRef;
use crate::runtime_data_area::heap::method::Method;
use std::rc::Rc;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::class_file::constant_pool::ConstantMethodRefInfo;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;

#[derive(Debug)]
pub struct MethodRef {
    member_ref:MemberRef,
    method:Option<Rc<Method>>
}

impl MethodRef {
    pub fn new_method_ref(cp:Rc<RefCell<ConstantPool>>,info:&ConstantMethodRefInfo) -> MethodRef {
        let mut field_ref = MethodRef{
            member_ref: MemberRef::with_pool(cp),
            method: None
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
    pub fn set_constant_pool(&mut self,pool:Rc<RefCell<ConstantPool>>) {
        self.member_ref.set_constant_pool(pool);
    }

    pub fn resolved_method(&self) -> Rc<Method> {
        if self.method.is_none() {

        }
        return self.method;
    }

    pub fn resolved_method_ref(&self) {
        let c = self.member_ref.constant_pool().class();
        let class = self.member_ref.resolved_class();
        if (*class).borrow().is_interface() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

    }

    pub fn look_up_method(class:Rc<RefCell<Class>>,name:&str,desc:&str) -> Method {

    }

    pub fn look_up_method_in_class(class:Rc<RefCell<Class>>,name:&str,desc:&str) -> Method {
        let super_class = (*class).borrow().super_class();
        while super_class.is_some() {

        }

    }

    pub fn look_up_method_in_interfaces(class:Rc<RefCell<Class>>,name:&str,desc:&str) -> Method {

    }
}