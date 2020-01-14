use crate::runtime_data_area::heap::class_member::ClassMember;
use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::member_info::MemberInfo;

pub struct Field {
    class_member:ClassMember
}

impl Field {
    #[inline]
    pub fn new() -> Field {
        return Field{ class_member: ClassMember::new() };
    }

    pub fn new_fields(class:Rc<Class>,infos:&Vec<MemberInfo>) -> Vec<Field> {
        let mut fields = Vec::with_capacity(infos.len());
        for info in infos {
            let mut field = Field::new();
            field.class_member.set_class(class.clone());
            field.class_member.copy_member_info(info);
            fields.push(field);
        }
        return fields;
    }
}