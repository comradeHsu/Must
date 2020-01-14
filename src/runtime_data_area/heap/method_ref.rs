use crate::runtime_data_area::heap::member_ref::MemberRef;
use crate::runtime_data_area::heap::method::Method;
use std::rc::Rc;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::class_file::constant_pool::ConstantMethodRefInfo;

pub struct MethodRef {
    member_ref:MemberRef,
    method:Rc<Method>
}

impl MethodRef {
    pub fn new_method_ref(cp:Rc<ConstantPool>,info:&ConstantMethodRefInfo) -> MethodRef {
        let mut field_ref = MethodRef{
            member_ref: MemberRef::with_pool(cp),
            method: Rc::new(Method::new())
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }
}