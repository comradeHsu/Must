use crate::class_file::class_reader::ClassReader;
use std::any::Any;
use std::mem;

pub type ConstantPool = Vec<Box<dyn ConstantInfo>>;

pub fn read_constant_pool(reader: &mut ClassReader) -> ConstantPool {
    let cp_count = reader.read_u16();
    let mut cp = Vec::new();
    for mut i in 1..cp_count {
        let constant_info = read_constant_info(reader, &cp);
        let any = &constant_info as &dyn Any;
//        if any.downcast_ref::<String>() { }
        cp.push(constant_info);
    }
    return cp;
}

pub fn get_constant_info(this:&ConstantPool,index:usize) -> &dyn ConstantInfo {
    let info = this.get(index);
    if info.is_none() {
        panic!("Invalid constant pool index!");
    }
    return info.unwrap().as_ref();
}

pub fn get_name_and_type(this:&ConstantPool,index:usize) -> (&str,&str) {
    let info = get_constant_info(this,index);
    let mut inf = &ConstantNameAndTypeInfo{ name_index: 0, desc_index: 0 };
    unsafe {
        inf = mem::transmute::<&dyn ConstantInfo, &ConstantNameAndTypeInfo>(info);
    }
    let name = get_utf8(this,inf.name_index as usize);
    let desc = get_utf8(this,inf.desc_index as usize);
    return (name,desc);
}

pub fn get_class_name(this:&ConstantPool,index:usize) -> &str {
    let info = get_constant_info(this,index);
    unsafe {
        let info: &ConstantClassInfo = mem::transmute::<&dyn ConstantInfo, &ConstantClassInfo>(info);
        return get_utf8(this,info.name_index as usize);
    }
}

pub fn get_utf8(this:&ConstantPool,index:usize) -> &str {
    let info = get_constant_info(this,index);
    unsafe {
        let info: &ConstantUtf8Info = mem::transmute::<&dyn ConstantInfo, &ConstantUtf8Info>(info);
        return info.val.as_str();
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
            _ => panic!("java.lang.ClassFormatError: constant pool tag!")
        };
        return tag;
    }
}

trait ConstantInfo {
    fn read_info(&mut self,reader: &mut ClassReader);
}

pub fn read_constant_info(reader:&mut ClassReader,cp:&ConstantPool) -> Box<dyn ConstantInfo> {
    let tag = reader.read_u8();
    let mut constant_info = new(tag,cp);
    constant_info.read_info(reader);
    return constant_info;
}

pub fn new<'a>(tag:u8, cp:&'a ConstantPool) -> Box<dyn ConstantInfo> {
    let constant_info:Box<dyn ConstantInfo> = match ConstantInfoTag::from(tag) {
        ConstantInfoTag::ConstantUtf8 => Box::new(ConstantUtf8Info{ val: String::new() }),
        ConstantInfoTag::ConstantInteger => Box::new(ConstantIntegerInfo{ val: 0 }),
        ConstantInfoTag::ConstantFloat => Box::new(ConstantFloatInfo{ val: 0.0 }),
        ConstantInfoTag::ConstantLong => Box::new(ConstantLongInfo{ val: 0 }),
        ConstantInfoTag::ConstantDouble => Box::new(ConstantDoubleInfo{ val: 0.0 }),
        ConstantInfoTag::ConstantClass => Box::new(ConstantClassInfo{ cp, name_index: 0 }),
        ConstantInfoTag::ConstantString => Box::new(ConstantStringInfo{ cp, string_index: 0 }),
        ConstantInfoTag::ConstantFieldref => Box::new(ConstantFieldRefInfo(ConstantMemberRefInfo{
            cp,
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantMethodref => Box::new(ConstantMethodRefInfo(ConstantMemberRefInfo{
            cp,
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantInterfaceMethodref => Box::new(ConstantInterfaceMethodRefInfo(ConstantMemberRefInfo{
            cp,
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantNameAndType => Box::new(ConstantNameAndTypeInfo{ name_index: 0,desc_index: 0 }),
        ConstantInfoTag::ConstantMethodHandle => Box::new(ConstantIntegerInfo{ val: 0 }),
        ConstantInfoTag::ConstantMethodType => Box::new(ConstantIntegerInfo{ val: 0 }),
        ConstantInfoTag::ConstantInvokeDynamic => Box::new(ConstantIntegerInfo{ val: 0 }),
        _ => panic!("")
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

struct ConstantStringInfo<'a> {
    cp:& 'a ConstantPool,
    string_index:u16
}

impl ConstantStringInfo<'_> {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.string_index = reader.read_u16();
    }

    pub fn string(&self) -> &str {
        return "";
    }
}

impl ConstantInfo for ConstantStringInfo<'_> {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantClassInfo<'a> {
    cp:& 'a ConstantPool,
    name_index:u16
}

impl ConstantClassInfo<'_> {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.name_index = reader.read_u16();
    }

    pub fn name(&self) -> &str {
        return "";
    }
}

impl ConstantInfo for ConstantClassInfo<'_> {
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

struct ConstantMemberRefInfo<'a> {
    cp:& 'a ConstantPool,
    class_index:u16,
    name_and_type_index:u16
}

impl ConstantMemberRefInfo<'_> {
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

impl ConstantInfo for ConstantMemberRefInfo<'_> {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

struct ConstantFieldRefInfo<'a>(ConstantMemberRefInfo<'a>);

impl ConstantInfo for ConstantFieldRefInfo<'_> {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}

struct ConstantMethodRefInfo<'a>(ConstantMemberRefInfo<'a>);

impl ConstantInfo for ConstantMethodRefInfo<'_> {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}

struct ConstantInterfaceMethodRefInfo<'a>(ConstantMemberRefInfo<'a>);

impl ConstantInfo for ConstantInterfaceMethodRefInfo<'_> {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}