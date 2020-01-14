use crate::runtime_data_area::heap::member_ref::MemberRef;
use std::rc::Rc;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::class_file::constant_pool::ConstantFieldRefInfo;

pub struct FieldRef {
    member_ref:MemberRef,
    field:Rc<Field>
}

impl FieldRef {
    pub fn new_field_ref(cp:Rc<ConstantPool>,info:&ConstantFieldRefInfo) -> FieldRef {
        let mut field_ref = FieldRef{
            member_ref: MemberRef::with_pool(cp),
            field: Rc::new(Field::new())
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }
}