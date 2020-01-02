use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::member_info::MemberInfo;
use crate::class_file::class_reader::ClassReader;

struct ClassFile {
    minor_version:u16,
    major_version:u16,
    constant_pool:ConstantPool,
    access_flags:u16,
    this_class:u16,
    super_class:u16,
    interfaces:Vec<u16>,
    fields:Vec<MemberInfo>,
    methods:Vec<MemberInfo>,
    attributes:()
}

impl ClassFile {
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
            attributes: ()
        };
    }

    fn read(&mut self, reader:&mut ClassReader) {
        self.read_and_check_magic(reader);
        self.read_and_check_version(reader);
        self.constant_pool = ();
        self.accessFlags = reader.readUint16();
        self.thisClass = reader.readUint16();
        self.superClass = reader.readUint16();
        self.interfaces = reader.readUint16s();
        self.fields = readMembers(reader, self.constantPool);
        self.methods = readMembers(reader, self.constantPool);
        self.attributes = readAttributes(reader, self.constantPool)
    }

    fn read_and_check_magic(&mut self, reader:&mut ClassReader) {
        let magic = reader.read_u32();
        if magic != 0xCAFEBABE {
            panic!("java.lang.ClassFormatError: magic!")
        }
    }

    fn read_and_check_version(&mut self, reader:&mut ClassReader) {
        self.minorVersion = reader.read_u16();
        self.majorVersion = reader.read_u16();
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
        return "";
    }

    pub fn super_class_name(&self) -> &str {
        return "";
    }

    pub fn interface_names(&self) -> Vec<&str> {
        return Vec::new();
    }
}