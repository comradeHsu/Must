use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::member_info::{MemberInfo, display_16};
use crate::class_file::class_reader::ClassReader;
use crate::class_file::attribute_info::{AttributeInfo, read_attributes, Attribute};
use std::vec::Vec;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::borrow::Borrow;
use crate::class_file::attribute_info::Attribute::SourceFile;
use crate::class_file::source_file_attribute::SourceFileAttribute;

pub struct ClassFile {
    minor_version:u16,
    major_version:u16,
    constant_pool:Rc<RefCell<ConstantPool>>,
    access_flags:u16,
    this_class:u16,
    super_class:u16,
    interfaces:Vec<u16>,
    fields:Vec<MemberInfo>,
    methods:Vec<MemberInfo>,
    attributes:Vec<Attribute>
}

impl ClassFile {
    pub fn parse(class_data:Vec<u8>) -> ClassFile {
        let mut class_reader = ClassReader::new(class_data);
        let mut class_file = ClassFile{
            minor_version: 0,
            major_version: 0,
            constant_pool: Rc::new(RefCell::new(ConstantPool::new())),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        };
        class_file.read(&mut class_reader);
        return class_file;
    }

    fn read(& mut self, reader:&mut ClassReader) {
        self.read_and_check_magic(reader);
        self.read_and_check_version(reader);
        self.constant_pool = ConstantPool::read_constant_pool(reader);
        self.access_flags = reader.read_u16();
        self.this_class = reader.read_u16();
        self.super_class = reader.read_u16();
        self.interfaces = reader.read_u16_table();
        self.fields = MemberInfo::read_members(reader, self.constant_pool.clone());
        self.methods = MemberInfo::read_members(reader, self.constant_pool.clone());
        self.attributes = read_attributes(reader, self.constant_pool.clone())
    }

    fn read_and_check_magic(&mut self, reader:&mut ClassReader) {
        let magic = reader.read_u32();
        if magic != 0xCAFEBABE {
            panic!("java.lang.ClassFormatError: magic!")
        }
    }

    fn read_and_check_version(&mut self, reader:&mut ClassReader) {
        self.minor_version = reader.read_u16();
        self.major_version = reader.read_u16();
        match self.major_version {
            45 => return,
            46..=52 => {
                if self.minor_version == 0 {
                    return
                }
            }
            _ => {}
        }
        panic!("java.lang.UnsupportedClassVersionError!")
    }

    pub fn minor_version(&self) -> u16 {
        return self.minor_version;
    }

    pub fn major_version(&self) -> u16 {
        return self.major_version;
    }

    pub fn constant_pool(&self) -> Rc<RefCell<ConstantPool>> {
        return self.constant_pool.clone();
    }

    pub fn access_flags(&self) -> u16 {
        return self.access_flags;
    }

    pub fn fields(&self) -> &Vec<MemberInfo> {
        return &self.fields;
    }

    pub fn methods(&self) -> &Vec<MemberInfo> {
        return &self.methods;
    }

    pub fn class_name(&self) -> String {
        let clone = self.constant_pool.clone();
        let borrow = (*clone).borrow();
        return borrow.get_class_name(self.this_class as usize).to_owned();
    }

    pub fn super_class_name(&self) -> Option<String> {
        if self.super_class > 0 {
            let borrow = (*self.constant_pool).borrow();
            return Some(borrow.get_class_name(self.super_class as usize).to_owned());
        }
        return None // 只有 java.lang.Object没有超类
    }

    pub fn interface_names(&self) -> Vec<String> {
        let mut interface_names = Vec::new();
        for index in &self.interfaces {
            interface_names.push((*self.constant_pool).borrow().get_class_name(*index as usize).to_string());
        }
        return interface_names;
    }

    pub fn source_file_attribute(&self) -> Option<&SourceFileAttribute> {
        for attr in &self.attributes {
            match attr {
                SourceFile(r) => return Some(r),
                _ => {}
            }
        }
        return None;
    }

    #[inline]
    pub fn attributes(&self) -> &Vec<Attribute> {
        return self.attributes.as_ref();
    }

    pub fn display(&self) {
        println!("ClassFile:");
        println!("  minor_version:{}",self.minor_version);
        println!("  major_version:{}",self.major_version);
        println!("  constant_pool count:{}",(*self.constant_pool).borrow().len());
        println!("  access_flags:{}",self.access_flags);
        println!("  this_class:{}",self.class_name());
        println!("  super_class_name:{:?}",self.super_class_name());
        println!("  interface_names:[");
        let interface_names = self.interface_names();
        for interface_name in interface_names {
            println!("{},",interface_name);
        }
        println!("  ]");
        println!("  fields:[");
        for field in self.fields() {
            println!("    {},",field.name());
        }
        println!("  ]");
        println!("  methods:[");
        for field in self.methods() {
            println!("    {},",field.name());
        }
        println!("  ]");
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use core::mem;
    use std::cell::RefCell;

    #[test]
    fn test_swap() {
        let mut rc_1 = Rc::new("123");
        let rc_2 = rc_1.clone();
        let mut rc_3 = Rc::new("456");
        mem::swap(&mut rc_1, &mut rc_3);
        println!("rc_1:{},rc_2:{},rc_3:{}",rc_1,rc_2,rc_3);
    }

    #[test]
    fn test_into_inner() {
//        let mut rc_1 = Rc::new(RefCell::new("123"));
//        let x = rc_1.into_inner();
//        println!("rc_1:{:?},x:{}",rc_1,x);
    }
}