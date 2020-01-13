use crate::class_file::class_reader::ClassReader;
use std::any::Any;
use std::mem;
use std::rc::Rc;
use crate::class_file::makers_attribute::DeprecatedAttribute;
use crate::class_file::member_info::display_16;
use crate::class_file::constant_pool::ConstantInfoEnum::*;

//pub type ConstantPool = Vec<ConstantInfoEnum>;

pub struct ConstantPool {
    vec:Vec<ConstantInfoEnum>
}

impl ConstantPool {

    pub fn new() -> ConstantPool {
        return ConstantPool{ vec: vec![] };
    }

    pub fn read_constant_pool(reader: &mut ClassReader) -> Rc<ConstantPool> {
        let cp_count = reader.read_u16();
        let mut cp = Rc::new(ConstantPool::new());
        let mut vec: Vec<ConstantInfoEnum> = Vec::new();
        let mut i = 1;
        while i < cp_count {
            let constant_info = read_constant_info(reader, cp.clone());
            match &constant_info {
                Long(info) => {
                    i = i + 1;
                    vec.push(None);
                    println!("Long")
                },
                Double(info) => {
                    i = i + 1;
                    vec.push(None);
                    println!("Double")
                },
                _ => {}
            }
            vec.push(constant_info);
            i += 1;
        }
        let mut c = Rc::new(ConstantPool{vec});
        mem::swap(&mut c, &mut cp);
        return cp;
    }

    pub fn get_constant_info(&self, index: usize) -> &ConstantInfoEnum {
        let info = self.vec.get(index - 1);
        if info.is_none() {
            println!("index is {}, vec len : {}",index,self.vec.len());
            panic!("Invalid constant pool index!");
        }
        return info.unwrap();
    }

    pub fn get_name_and_type(&self, index: usize) -> (&str, &str) {
        let info = self.get_constant_info(index);
        let mut inf = match info {
            NameAndType(name_and_type) => name_and_type,
            _ => panic!("info is not NameAndType")
        };
        let name = self.get_utf8(inf.name_index as usize);
        let desc = self.get_utf8(inf.desc_index as usize);
        return (name, desc);
    }

    pub fn get_class_name(&self, index: usize) -> &str {
        let info = self.get_constant_info(index);
        let mut class = match info {
            Class(class) => class,
            _ => panic!("info is not NameAndType")
        };
        return self.get_utf8(class.name_index as usize);
    }

    pub fn get_utf8(&self, index: usize) -> &str {
        let info = self.get_constant_info(index);
        let utf8 = match info {
            Utf8(utf) => utf,
            _ => panic!("info is not NameAndType")
        };
        return utf8.val.as_str();
    }

    #[inline]
    pub fn len(&self) -> usize {
        return self.vec.len();
    }
}

enum ConstantInfoTag {
    ConstantClass = 7,
    ConstantFieldref = 9,
    ConstantMethodref = 10,
    ConstantInterfaceMethodref = 11,
    ConstantString = 8,
    ConstantInteger = 3,
    ConstantFloat = 4,
    ConstantLong = 5,
    ConstantDouble = 6,
    ConstantNameAndType = 12,
    ConstantUtf8 = 1,
    ConstantMethodHandle = 15,
    ConstantMethodType = 16,
    ConstantInvokeDynamic = 18
}

impl ConstantInfoTag {
    pub fn from(v: u8) -> Self {
        let tag = match v {
            1 => ConstantInfoTag::ConstantUtf8,
            3 => ConstantInfoTag::ConstantInteger,
            4 => ConstantInfoTag::ConstantFloat,
            5 => ConstantInfoTag::ConstantLong,
            6 => ConstantInfoTag::ConstantDouble,
            7 => ConstantInfoTag::ConstantClass,
            8 => ConstantInfoTag::ConstantString,
            9 => ConstantInfoTag::ConstantFieldref,
            10 => ConstantInfoTag::ConstantMethodref,
            11 => ConstantInfoTag::ConstantInterfaceMethodref,
            12 => ConstantInfoTag::ConstantNameAndType,
            15 => ConstantInfoTag::ConstantMethodHandle,
            16 => ConstantInfoTag::ConstantMethodType,
            18 => ConstantInfoTag::ConstantInvokeDynamic,
            _ => {
                println!("tag:{}",v);
                panic!("java.lang.ClassFormatError: constant pool tag!")
            }
        };
        return tag;
    }
}

pub enum ConstantInfoEnum {
    None,
    Utf8(ConstantUtf8Info),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    Class(ConstantClassInfo),
    Str(ConstantStringInfo),
    FieldRef(ConstantFieldRefInfo),
    MethodRef(ConstantMethodRefInfo),
    InterfaceMethodRef(ConstantInterfaceMethodRefInfo),
    NameAndType(ConstantNameAndTypeInfo),
    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo)
}

impl ConstantInfoEnum {
    pub fn read_info(&mut self,reader: &mut ClassReader) {
        match self {
            Utf8(utf8) => utf8.read_info(reader),
            Integer(integer) => integer.read_info(reader),
            Float(float) => float.read_info(reader),
            Long(long) => long.read_info(reader),
            Double(double) => double.read_info(reader),
            Class(class) => class.read_info(reader),
            Str(string) => string.read_info(reader),
            FieldRef(field) => field.read_info(reader),
            MethodRef(method_ref) => method_ref.read_info(reader),
            InterfaceMethodRef(interface) => interface.read_info(reader),
            NameAndType(name_and_type) => name_and_type.read_info(reader),
            MethodHandle(methodHandle) => methodHandle.read_info(reader),
            MethodType(methodType) => methodType.read_info(reader),
            InvokeDynamic(invoke) => invoke.read_info(reader),
            _ => {}
        }
    }
}

pub trait ConstantInfo {
    fn read_info(&mut self,reader: &mut ClassReader);
}

pub fn read_constant_info(reader:&mut ClassReader,cp:Rc<ConstantPool>) -> ConstantInfoEnum {
    let dat = display_16(reader.data.clone());
    let tag = reader.read_u8();
    let mut constant_info = new(tag,cp);
    constant_info.read_info(reader);
    return constant_info;
}

pub fn new(tag:u8, cp: Rc<ConstantPool>) -> ConstantInfoEnum {
    let constant_info:ConstantInfoEnum = match ConstantInfoTag::from(tag) {
        ConstantInfoTag::ConstantUtf8 => Utf8(ConstantUtf8Info{ val: String::new() }),
        ConstantInfoTag::ConstantInteger => Integer(ConstantIntegerInfo{ val: 0 }),
        ConstantInfoTag::ConstantFloat => Float(ConstantFloatInfo{ val: 0.0 }),
        ConstantInfoTag::ConstantLong => Long(ConstantLongInfo{ val: 0 }),
        ConstantInfoTag::ConstantDouble => Double(ConstantDoubleInfo{ val: 0.0 }),
        ConstantInfoTag::ConstantClass => Class(ConstantClassInfo{ cp:cp.clone(), name_index: 0 }),
        ConstantInfoTag::ConstantString => Str(ConstantStringInfo{ cp:cp.clone(), string_index: 0 }),
        ConstantInfoTag::ConstantFieldref => FieldRef(ConstantFieldRefInfo(ConstantMemberRefInfo{
            cp:cp.clone(),
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantMethodref => MethodRef(ConstantMethodRefInfo(ConstantMemberRefInfo{
            cp:cp.clone(),
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantInterfaceMethodref => InterfaceMethodRef(
            ConstantInterfaceMethodRefInfo(ConstantMemberRefInfo{
            cp,
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantNameAndType => NameAndType(ConstantNameAndTypeInfo{ name_index: 0,desc_index: 0 }),
        //待完善
        ConstantInfoTag::ConstantMethodHandle => MethodHandle(ConstantMethodHandleInfo{
            reference_kind: 0, reference_index: 0
        }),
        ConstantInfoTag::ConstantMethodType => MethodType(ConstantMethodTypeInfo{ descriptor_index: 0 }),
        ConstantInfoTag::ConstantInvokeDynamic => InvokeDynamic(ConstantInvokeDynamicInfo{
            bootstrap_method_attr_index: 0,
            name_and_type_index: 0
        }),
    };
    return constant_info;
}

struct ConstantIntegerInfo {
    val:i32
}

impl ConstantIntegerInfo {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.val = reader.read_u32() as i32;
    }
}

impl ConstantInfo for ConstantIntegerInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantFloatInfo {
    val:f32
}

impl ConstantFloatInfo{
    pub fn read_info(&mut self,reader:&mut ClassReader){
        let byte = reader.read_u32();
        self.val = f32::from_bits(byte);
    }
}

impl ConstantInfo for ConstantFloatInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantLongInfo {
    val:i64
}

impl ConstantLongInfo{
    pub fn read_info(&mut self,reader:&mut ClassReader){
        let byte = reader.read_u64();
        self.val = byte as i64;
    }
}

impl ConstantInfo for ConstantLongInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantDoubleInfo {
    val:f64
}

impl ConstantDoubleInfo{
    pub fn read_info(&mut self,reader:&mut ClassReader){
        let byte = reader.read_u64();

        self.val = f64::from_bits(byte);
        println!("double:{}",self.val);
    }
}

impl ConstantInfo for ConstantDoubleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantUtf8Info {
    val:String
}

impl ConstantUtf8Info{
    pub fn read_info(&mut self,reader:&mut ClassReader){
        let len = reader.read_u16() as usize;
        let bytes = reader.read_bytes(len);
        self.val = String::from_utf8(bytes).unwrap();
    }
}

impl ConstantInfo for ConstantUtf8Info {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantStringInfo {
    cp:Rc<ConstantPool>,
    string_index:u16
}

impl ConstantStringInfo {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.string_index = reader.read_u16();
    }

    pub fn string(&self) -> &str {
        return "";
    }
}

impl ConstantInfo for ConstantStringInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantClassInfo {
    cp:Rc<ConstantPool>,
    name_index:u16
}

impl ConstantClassInfo {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.name_index = reader.read_u16();
    }

    pub fn name(&self) -> &str {
        return "";
    }
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantNameAndTypeInfo {
    name_index:u16,
    desc_index:u16
}

impl ConstantNameAndTypeInfo{
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.name_index = reader.read_u16();
        self.desc_index = reader.read_u16();
    }
}

impl ConstantInfo for ConstantNameAndTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantMemberRefInfo {
    cp:Rc<ConstantPool>,
    class_index:u16,
    name_and_type_index:u16
}

impl ConstantMemberRefInfo {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();
    }

    pub fn class_name(&self) -> &str {
        return "";
    }

    pub fn name_and_descriptor(&self) -> &str {
        return "";
    }
}

impl ConstantInfo for ConstantMemberRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantFieldRefInfo(ConstantMemberRefInfo);

impl ConstantInfo for ConstantFieldRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}

struct ConstantMethodRefInfo(ConstantMemberRefInfo);

impl ConstantInfo for ConstantMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}

struct ConstantInterfaceMethodRefInfo(ConstantMemberRefInfo);

impl ConstantInfo for ConstantInterfaceMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}

struct ConstantInvokeDynamicInfo {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16
}

impl ConstantInfo for ConstantInvokeDynamicInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.bootstrap_method_attr_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();
    }
}

struct ConstantMethodHandleInfo {
    reference_kind:u8,
    reference_index:u16
}

impl ConstantInfo for ConstantMethodHandleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.reference_kind = reader.read_u8();
        self.reference_index = reader.read_u16();
    }
}

struct ConstantMethodTypeInfo {
    descriptor_index:u16
}

impl ConstantInfo for ConstantMethodTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.descriptor_index = reader.read_u16();
    }
}