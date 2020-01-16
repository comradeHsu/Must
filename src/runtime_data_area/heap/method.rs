use crate::runtime_data_area::heap::class_member::ClassMember;
use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::member_info::MemberInfo;
use std::cell::RefCell;

pub struct Method {
    class_member:ClassMember,
    max_stack:usize,
    max_locals:usize,
    code:Vec<u8>
}

impl Method {

    #[inline]
    pub fn new() -> Method {
        return Method{ 
            class_member: ClassMember::new(), 
            max_stack: 0, 
            max_locals: 0, 
            code: vec![] 
        };
    }

    pub fn new_methods(class:Rc<RefCell<Class>>,infos:&Vec<MemberInfo>) -> Vec<Method> {
        let mut methods = Vec::with_capacity(infos.len());
        for info in infos {
            let mut method = Method::new();
            method.class_member.set_class(class.clone());
            method.class_member.copy_member_info(info);
            method.copy_attributes(info);
            methods.push(method);
        }
        return methods;
    }
    /// clone cast,waiting improve
    pub fn copy_attributes(&mut self,info:&MemberInfo) {
        let code = info.code_attributes();
        match code {
            Some(attr) => {
                self.max_locals = attr.max_locals() as usize;
                self.max_stack = attr.max_stack() as usize;
                self.code = attr.code().clone();
            },
            None => {}
        }
    }

    #[inline]
    pub fn class(&self) -> Rc<RefCell<Class>> {
        return self.class_member.class();
    }

    #[inline]
    pub fn name(&self) -> &str {
        return self.class_member.name();
    }
}