use crate::class_file::constant_pool::{ConstantPool, get_class_name, read_constant_pool};
use crate::class_file::member_info::MemberInfo;
use crate::class_file::class_reader::ClassReader;
use crate::class_file::attribute_info::{AttributeInfo, read_attributes};
use std::vec::Vec;

struct ClassFile<'a> {
    minor_version:u16,
    major_version:u16,
    constant_pool:ConstantPool,
    access_flags:u16,
    this_class:u16,
    super_class:u16,
    interfaces:Vec<u16>,
    fields:Vec<MemberInfo<'a>>,
    methods:Vec<MemberInfo<'a>>,
    attributes:Vec<Box<dyn AttributeInfo>>
}

impl<'a> ClassFile<'a> {
    pub fn parse(class_data:Vec<u8>) {
        let mut class_reader = ClassReader{ data: class_data };
        let class_file = ClassFile{
            minor_version: 0,
            major_version: 0,
            constant_pool: vec![],
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        };
    }

    fn read(&'a mut self, reader:&mut ClassReader) {
        self.read_and_check_magic(reader);
        self.read_and_check_version(reader);
        self.constant_pool = read_constant_pool(reader);
        self.access_flags = reader.read_u16();
        self.this_class = reader.read_u16();
        self.super_class = reader.read_u16();
        self.interfaces = reader.read_u16_table();
        self.fields = MemberInfo::read_members(reader, &self.constant_pool);
        self.methods = MemberInfo::read_members(reader, &self.constant_pool);
        self.attributes = read_attributes(reader, &self.constant_pool)
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

    pub fn constant_pool(&self) -> &ConstantPool {
        return &self.constant_pool;
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

    pub fn class_name(&self) -> &str {
        return get_class_name(&self.constant_pool,self.this_class as usize);
    }

    pub fn super_class_name(&self) -> &str {
        if self.super_class > 0 {
            return get_class_name(&self.constant_pool,self.super_class as usize);
        }
        return "" // 只有 java.lang.Object没有超类
    }

    pub fn interface_names(&self) -> Vec<&str> {
        let mut interface_names = Vec::new();
        for index in self.interfaces {
            interface_names.push(get_class_name(&self.constant_pool,index as usize));
        }
        return interface_names;
    }
}