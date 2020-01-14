use std::rc::Rc;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::member_info::MemberInfo;

pub struct ClassMember {
    access_flags:u16,
    name:String,
    descriptor:String,
    class:Rc<Class>
}

impl ClassMember {

    #[inline]
    pub fn new() -> ClassMember {
        return ClassMember{
            access_flags: 0,
            name: "".to_string(),
            descriptor: "".to_string(),
            class: Rc::new(Class::new())
        };
    }

    pub fn copy_member_info(&mut self,info:&MemberInfo) {
        self.access_flags = info.access_flags();
        self.name = info.name().to_string();
        self.descriptor = info.descriptor().to_string();
    }

    pub fn set_class(&mut self,class:Rc<Class>) {
        self.class = class;
    }
}