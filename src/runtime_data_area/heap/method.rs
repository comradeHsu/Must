use crate::runtime_data_area::heap::class_member::ClassMember;
use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::member_info::MemberInfo;
use std::cell::RefCell;
use crate::runtime_data_area::heap::method_descriptor::MethodDescriptorParser;
use crate::runtime_data_area::heap::access_flags::NATIVE;
use crate::runtime_data_area::heap::exception_table::ExceptionTable;
use crate::class_file::line_number_table_attribute::LineNumberTableAttribute;

#[derive(Debug)]
pub struct Method {
    class_member:ClassMember,
    max_stack:usize,
    max_locals:usize,
    code:Vec<u8>,
    arg_slot_count:usize,
    exception_table:ExceptionTable,
    line_number_table:Option<LineNumberTableAttribute>
}

impl Method {

    #[inline]
    pub fn new() -> Method {
        return Method{ 
            class_member: ClassMember::new(), 
            max_stack: 0, 
            max_locals: 0, 
            code: vec![],
            arg_slot_count: 0,
            exception_table: ExceptionTable::none(),
            line_number_table: None
        };
    }

    pub fn new_methods(class:Rc<RefCell<Class>>,infos:&Vec<MemberInfo>) -> Vec<Rc<Method>> {
        let mut methods = Vec::with_capacity(infos.len());
        for info in infos {
            methods.push(Method::new_method(class.clone(),info));
        }
        return methods;
    }

    fn new_method(class:Rc<RefCell<Class>>,info:&MemberInfo) -> Rc<Method> {
        let mut method = Method::new();
        method.class_member.set_class(class.clone());
        method.class_member.copy_member_info(info);
        method.copy_attributes(info);
        let md = MethodDescriptorParser::parse_method_descriptor(method.descriptor());
        method.calc_arg_slot_count(md.parameter_types());
        if method.is_native() {
            method.inject_code_attribute(md.return_type());
        }
        return Rc::new(method);
    }

    /// clone cast,waiting improve
    pub fn copy_attributes(&mut self,info:&MemberInfo) {
        let code = info.code_attributes();
        match code {
            Some(attr) => {
                self.max_locals = attr.max_locals() as usize;
                self.max_stack = attr.max_stack() as usize;
                self.code = attr.code().clone();
                self.line_number_table = attr.line_number_table_attribute();
                self.exception_table = ExceptionTable::new(attr.exception_table(),
                                                        (*self.class()).borrow().constant_pool());
            },
            None => {}
        }
    }

    fn calc_arg_slot_count(&mut self,parameter_types:&Vec<String>) {
//        let parsed_desc = MethodDescriptorParser::parse_method_descriptor(self.descriptor());
        for parameter_type in parameter_types {
            self.arg_slot_count += 1;
            if parameter_type.as_str() == "J" || parameter_type.as_str() == "D" {
                self.arg_slot_count += 1;
            }
        }
        if !self.is_static() {
            self.arg_slot_count += 1;
        }
    }

    /// construct native code inject to operand stack
    fn inject_code_attribute(&mut self, return_type:&String) {
        self.max_stack = 4;
        self.max_locals = self.arg_slot_count;
        let first = return_type.chars().next().unwrap();
        match first {
            'V' => self.code = vec![0xfe, 0xb1], // return
            'D' => self.code = vec![0xfe, 0xaf], // dreturn
            'F' => self.code = vec![0xfe, 0xae], // freturn
            'J' => self.code = vec![0xfe, 0xad], // lreturn
            'L' | '[' => self.code = vec![0xfe, 0xb0], // areturn
            _ => self.code = vec![0xfe, 0xac] // ireturn
        }
    }

    pub fn find_exception_handler(&self, class:Rc<RefCell<Class>>, pc:i32) -> i32 {
        let handler = self.exception_table.find_exception_handler(class, pc);
        if handler.is_some() {
            return handler.unwrap().handler_pc();
        }
        return -1;
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
    pub fn is_private(&self) -> bool {
        return self.class_member.is_private();
    }

    #[inline]
    pub fn is_abstract(&self) -> bool {
        return self.class_member.is_abstract();
    }

    #[inline]
    pub fn is_native(&self) -> bool {
        return 0 != self.class_member.access_flags() & NATIVE;
    }

    pub fn get_line_number(&self, pc:i32) -> i32 {
        if self.is_native() {
            return -2;
        }
        if self.line_number_table.is_none() {
            return -1;
        }
        return self.line_number_table.as_ref().unwrap().get_line_number(pc as u16);
    }

}