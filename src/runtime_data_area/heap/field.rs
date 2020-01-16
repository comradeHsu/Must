use crate::runtime_data_area::heap::class_member::ClassMember;
use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::member_info::MemberInfo;
use std::cell::RefCell;

pub struct Field {
    class_member:ClassMember,
    const_value_index:usize,
    slot_id:usize
}

impl Field {
    #[inline]
    pub fn new() -> Field {
        return Field{
            class_member: ClassMember::new(),
            const_value_index: 0,
            slot_id: 0
        };
    }

    pub fn new_fields(class:Rc<RefCell<Class>>,infos:&Vec<MemberInfo>) -> Vec<Field> {
        let mut fields = Vec::with_capacity(infos.len());
        for info in infos {
            let mut field = Field::new();
            field.class_member.set_class(class.clone());
            field.class_member.copy_member_info(info);
            field.copy_const_attribute(info);
            fields.push(field);
        }
        return fields;
    }

    fn copy_const_attribute(&mut self,info:&MemberInfo) {
        let const_attr = info.constant_value_attr();
        if const_attr.is_some() {
            self.const_value_index = const_attr.unwrap().value_index() as usize;
        }
    }

    #[inline]
    pub fn parent(&self) -> &ClassMember {
        return &self.class_member;
    }

    #[inline]
    pub fn const_value_index(&self) -> usize {
        return self.const_value_index;
    }

    #[inline]
    pub fn slot_id(&self) -> usize {
        return self.slot_id;
    }

    #[inline]
    pub fn name(&self) -> &str {
        return self.class_member.name();
    }

    #[inline]
    pub fn descriptor(&self) -> &str {
        return self.class_member.descriptor();
    }

    #[inline]
    pub fn set_slot(&mut self,slot_id:usize) {
        self.slot_id = slot_id;
    }

    #[inline]
    pub fn is_long_or_double(&self) -> bool {
        let descriptor = self.class_member.descriptor();
        return descriptor == "J" || descriptor == "D";
    }

    #[inline]
    pub fn is_accessible_to(&self, class:&Class) -> bool {
        return self.class_member.is_accessible_to(class);
    }

}