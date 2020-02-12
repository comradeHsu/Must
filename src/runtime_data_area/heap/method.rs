use crate::class_file::attribute_info::Attribute::{Code, Exceptions, RuntimeVisibleAnnotations};
use crate::class_file::exceptions_attribute::ExceptionsAttribute;
use crate::class_file::line_number_table_attribute::LineNumberTableAttribute;
use crate::class_file::member_info::MemberInfo;
use crate::class_file::runtime_visible_annotations_attribute::AnnotationAttribute;
use crate::runtime_data_area::heap::access_flags::NATIVE;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::class_member::ClassMember;
use crate::runtime_data_area::heap::class_name_helper::PrimitiveTypes;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;
use crate::runtime_data_area::heap::exception_table::ExceptionTable;
use crate::runtime_data_area::heap::method_descriptor::{MethodDescriptor, MethodDescriptorParser};
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

#[derive(Debug)]
pub struct Method {
    class_member: ClassMember,
    max_stack: usize,
    max_locals: usize,
    code: Vec<u8>,
    arg_slot_count: usize,
    exception_table: ExceptionTable,
    line_number_table: Option<LineNumberTableAttribute>,
    annotations: Option<Vec<AnnotationAttribute>>,
    exceptions: Vec<u16>,
    method_desc: MethodDescriptor,
}

impl Method {
    #[inline]
    pub fn new() -> Method {
        return Method {
            class_member: ClassMember::new(),
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            arg_slot_count: 0,
            exception_table: ExceptionTable::none(),
            line_number_table: None,
            annotations: None,
            exceptions: vec![],
            method_desc: MethodDescriptor::new(),
        };
    }

    pub fn new_methods(class: Rc<RefCell<Class>>, infos: &Vec<MemberInfo>) -> Vec<Rc<Method>> {
        let mut methods = Vec::with_capacity(infos.len());
        for info in infos {
            methods.push(Method::new_method(class.clone(), info));
        }
        return methods;
    }

    fn new_method(class: Rc<RefCell<Class>>, info: &MemberInfo) -> Rc<Method> {
        let mut method = Method::new();
        method.class_member.set_class(class.clone());
        method.class_member.copy_member_info(info);
        method.copy_attributes(info);
        let md = MethodDescriptorParser::parse_method_descriptor(method.descriptor());
        method.calc_arg_slot_count(md.parameter_types());
        if method.is_native() {
            method.inject_code_attribute(md.return_type());
        }
        method.method_desc = md;
        return Rc::new(method);
    }

    /// clone cast,waiting improve
    pub fn copy_attributes(&mut self, info: &MemberInfo) {
        let attributes = info.attributes();
        for attribute in attributes {
            match attribute {
                Code(attr) => {
                    self.max_locals = attr.max_locals() as usize;
                    self.max_stack = attr.max_stack() as usize;
                    self.code = attr.code().clone();
                    self.line_number_table = attr.line_number_table_attribute();
                    self.exception_table = ExceptionTable::new(
                        attr.exception_table(),
                        (*self.class()).borrow().constant_pool(),
                    );
                }
                RuntimeVisibleAnnotations(attr) => {
                    let clone = attr.annotations().clone();
                    self.annotations = Some(clone)
                }
                Exceptions(attr) => {
                    self.exceptions = attr.unsafe_copy();
                }
                _ => {}
            }
        }
    }

    fn calc_arg_slot_count(&mut self, parameter_types: &Vec<String>) {
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
    fn inject_code_attribute(&mut self, return_type: &String) {
        self.max_stack = 4;
        self.max_locals = self.arg_slot_count;
        let first = return_type.chars().next().unwrap();
        match first {
            'V' => self.code = vec![0xfe, 0xb1],       // return
            'D' => self.code = vec![0xfe, 0xaf],       // dreturn
            'F' => self.code = vec![0xfe, 0xae],       // freturn
            'J' => self.code = vec![0xfe, 0xad],       // lreturn
            'L' | '[' => self.code = vec![0xfe, 0xb0], // areturn
            _ => self.code = vec![0xfe, 0xac],         // ireturn
        }
    }

    pub fn find_exception_handler(&self, class: Rc<RefCell<Class>>, pc: i32) -> i32 {
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
    pub fn code(&self) -> Vec<u8> {
        unsafe {
            let count = self.code.len();
            let mut data = Vec::with_capacity(count);
            ptr::copy_nonoverlapping(self.code.as_ptr(), data.as_mut_ptr(), count);
            data.set_len(count);
            return data;
        }
    }

    #[inline]
    pub fn is_accessible_to(&self, class: &Class) -> bool {
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

    pub fn get_line_number(&self, pc: i32) -> i32 {
        if self.is_native() {
            return -2;
        }
        if self.line_number_table.is_none() {
            return -1;
        }
        return self
            .line_number_table
            .as_ref()
            .unwrap()
            .get_line_number(pc as u16);
    }

    pub fn has_annotation(&self, name: &str) -> bool {
        if self.annotations.is_none() {
            return false;
        }
        let annotations = self.annotations.as_ref().unwrap();
        for annotation in annotations {
            if annotation.name() == name {
                return true;
            }
        }
        return false;
    }

    #[inline]
    pub fn is_constructor(&self) -> bool {
        return !self.is_static() && self.name() == "<init>";
    }

    #[inline]
    pub fn is_clinit(&self) -> bool {
        return self.is_static() && self.name() == "<clinit>";
    }

    #[inline]
    pub fn access_flags(&self) -> u16 {
        return self.class_member.access_flags();
    }

    #[inline]
    pub fn signature(&self) -> &str {
        return self.class_member.signature();
    }

    // reflection
    pub fn parameter_types(&self) -> Option<Vec<Rc<RefCell<Class>>>> {
        if self.arg_slot_count == 0 {
            return None;
        }
        let class_loader = (*self.class()).borrow().loader();
        let param_types = self.method_desc.parameter_types();
        let mut param_classes = Vec::with_capacity(param_types.len());
        for param_type in param_types {
            let param_class_name = PrimitiveTypes::instance()
                .unwrap()
                .to_class_name(param_type.as_str());
            param_classes.push(ClassLoader::load_class(
                class_loader.clone(),
                param_class_name.as_str(),
            ));
        }

        return Some(param_classes);
    }

    pub fn exception_types(&self) -> Option<Vec<Rc<RefCell<Class>>>> {
        if self.exceptions.len() == 0 {
            return None;
        }

        let mut ex_classes = Vec::with_capacity(self.exceptions.len());
        let class = self.class();
        let cp = (*class).borrow().constant_pool();
        let mut borrow = (*cp).borrow_mut();

        for i in 0..self.exceptions.len() {
            let ex_index = self.exceptions[i];
            let constant = borrow.get_constant(ex_index as usize);
            let class_ref = match constant {
                ClassReference(reff) => reff,
                _ => panic!("Not ClassReference"),
            };
            ex_classes.push(class_ref.resolved_class(class.clone()));
        }

        return Some(ex_classes);
    }

    pub fn return_type(&self) -> Rc<RefCell<Class>> {
        let return_type = self.method_desc.return_type();
        let return_class_name = PrimitiveTypes::instance()
            .unwrap()
            .to_class_name(return_type);
        let class_loader = (*self.class()).borrow().loader();
        return ClassLoader::load_class(class_loader, return_class_name.as_str());
    }

    pub fn shim_return_method() -> Method {
        let mut class = Class::none();
        return Method {
            class_member: ClassMember::shim(class),
            max_stack: 0,
            max_locals: 0,
            code: vec![0xb1],
            arg_slot_count: 0,
            exception_table: ExceptionTable::none(),
            line_number_table: None,
            annotations: None,
            exceptions: vec![],
            method_desc: MethodDescriptor::new(),
        };
    }
}
