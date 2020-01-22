use crate::runtime_data_area::heap::class_member::ClassMember;
use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::member_info::MemberInfo;
use std::cell::RefCell;
use crate::runtime_data_area::heap::method_descriptor::MethodDescriptorParser;

#[derive(Debug)]
pub struct Method {
    class_member:ClassMember,
    max_stack:usize,
    max_locals:usize,
    code:Vec<u8>,
    arg_slot_count:usize
}

impl Method {

    #[inline]
    pub fn new() -> Method {
        return Method{ 
            class_member: ClassMember::new(), 
            max_stack: 0, 
            max_locals: 0, 
            code: vec![],
            arg_slot_count: 0
        };
    }

    pub fn new_methods(class:Rc<RefCell<Class>>,infos:&Vec<MemberInfo>) -> Vec<Rc<Method>> {
        let mut methods = Vec::with_capacity(infos.len());
        for info in infos {
            let mut method = Method::new();
            method.class_member.set_class(class.clone());
            method.class_member.copy_member_info(info);
            method.copy_attributes(info);
            method.calc_arg_slot_count();
            methods.push(Rc::new(method));
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

    fn calc_arg_slot_count(&mut self) {
        let parsed_desc = MethodDescriptorParser::parse_method_descriptor(self.descriptor());
        for parameter_type in parsed_desc.parameter_types() {
            self.arg_slot_count += 1;
            if parameter_type.as_str() == "J" || parameter_type.as_str() == "D" {
                self.arg_slot_count += 1;
            }
        }
        if !self.is_static() {
            self.arg_slot_count += 1;
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

    #[inline]
    pub fn descriptor(&self) -> &str {
        return self.class_member.descriptor();
    }

    #[inline]
    pub fn max_stack(&self) -> usize {
        return self.max_stack;
    }

    #[inline]
    pub fn max_locals(&self) -> usize {
        return self.max_locals;
    }

    #[inline]
    pub fn arg_slot_count(&self) -> usize {
        return self.arg_slot_count;
    }

    #[inline]
    pub fn code(&self) -> &Vec<u8> {
        return &self.code;
    }

    #[inline]
    pub fn is_accessible_to(&self, class:&Class) -> bool {
        return self.class_member.is_accessible_to(class);
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        return self.class_member.is_static();
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        return self.class_member.is_public();
    }

    #[inline]
    pub fn is_protected(&self) -> bool {
        return self.class_member.is_protected();
    }

    #[inline]
    pub fn is_abstract(&self) -> bool {
        return self.class_member.is_abstract();
    }
}