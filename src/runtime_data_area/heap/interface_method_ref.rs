use crate::runtime_data_area::heap::member_ref::MemberRef;
use crate::runtime_data_area::heap::method::Method;
use std::rc::Rc;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::class_file::constant_pool::ConstantInterfaceMethodRefInfo;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;

#[derive(Debug)]
pub struct InterfaceMethodRef {
    member_ref:MemberRef,
    method:Rc<Method>
}

impl InterfaceMethodRef {
    pub fn new_method_ref(cp:Rc<RefCell<ConstantPool>>,info:&ConstantInterfaceMethodRefInfo) -> InterfaceMethodRef {
        let mut field_ref = InterfaceMethodRef{
            member_ref: MemberRef::with_pool(cp),
            method: Rc::new(Method::new())
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }

    pub fn resolved_interface_method(&self) -> Rc<Method> {
        if self.method.is_none() {

        }
        return self.method;
    }

    pub fn resolved_interface_method_ref(&self) -> Rc<Method> {

    }

    pub fn look_up_interface_method(class:Rc<RefCell<Class>>,name:&str,desc:&str) -> Method {

    }


    #[inline]
    pub fn set_constant_pool(&mut self,pool:Rc<RefCell<ConstantPool>>) {
        self.member_ref.set_constant_pool(pool);
    }
}