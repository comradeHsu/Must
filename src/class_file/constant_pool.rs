use crate::class_file::class_reader::ClassReader;

pub type ConstantPool = Vec<dyn ConstantInfo>;

fn read_constant_pool(reader: &mut ClassReader) -> ConstantPool {
    let cp_count = reader.read_u16();
    let cp = Vec::new();
    for i in 1..cp_count {}
    return cp;
}

pub fn get_constant_info(this:&ConstantPool,index:usize) -> &dyn ConstantInfo {
    let info = this.get(index);
    if info.is_none() {
        panic!("Invalid constant pool index!");
    }
    return info.unwrap();
}

pub fn get_name_and_type(this:&ConstantPool,index:usize) -> (&str,&str) {
    let info = get_constant_info(this,index) as &ConstantNameAndTypeInfo;
    let name = get_utf8(this,info.name_index as usize);
    let desc = get_utf8(this,info.desc_index as usize);
    return (name,desc);
}

pub fn get_class_name(this:&ConstantPool,index:usize) -> &str {
    let info = get_constant_info(this,index) as &ConstantClassInfo;
    return get_utf8(this,info.name_index as usize);
}

pub fn get_utf8(this:&ConstantPool,index:usize) -> &str {
    let info = get_constant_info(this,index) as &ConstantUtf8Info;
    return info.val.as_str();
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

trait ConstantInfo {
    fn read_info(&mut self,reader: &mut ClassReader);
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
        self.val = String::from_utf8(bytes)?;
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

struct ConstantClassInfo<'a> {
    cp:& 'a ConstantPool,
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

struct ConstantMemberRefInfo<'a> {
    cp:& 'a ConstantPool,
    class_index:u16,
    name_and_type_index:u16
}

impl ConstantMemberRefInfo{
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

struct ConstantFieldRefInfo<'a>(ConstantMemberRefInfo<'a>);

struct ConstantMethodRefInfo<'a>(ConstantMemberRefInfo<'a>);

struct ConstantInterfaceMethodRefInfo<'a>(ConstantMemberRefInfo<'a>);