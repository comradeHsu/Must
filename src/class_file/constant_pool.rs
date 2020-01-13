use crate::class_file::class_reader::ClassReader;
use std::any::Any;
use std::mem;
use std::rc::Rc;
use crate::class_file::makers_attribute::DeprecatedAttribute;
use crate::class_file::member_info::display_16;

pub type ConstantPool = Vec<Box<dyn ConstantInfo>>;

pub fn read_constant_pool(reader: &mut ClassReader) -> Rc<ConstantPool> {
    let cp_count = reader.read_u16();
    let mut cp = Rc::new(Vec::new());
    let mut vec: Vec<Box<dyn ConstantInfo>> = Vec::new();
    println!("cp_count:{}",cp_count);
    for mut i in 1..cp_count {
        println!("seq:{}",i);
        let constant_info = read_constant_info(reader, cp.clone());
        let any = &constant_info as &dyn Any;
        if any.downcast_ref::<ConstantLongInfo>().is_some() {
            println!("66666");
            i = i + 1;
        }
        if any.downcast_ref::<ConstantDoubleInfo>().is_some() {
            println!("66666");
            i = i + 1;
        }
        vec.push(constant_info);
    }
    let mut c = Rc::new(vec);
    mem::swap(&mut c,&mut cp);
    return cp;
}

pub fn get_constant_info<'a>(this:&'a Rc<ConstantPool>,index:usize) -> &'a dyn ConstantInfo {
    let info = this.get(index-1);
    if info.is_none() {
        panic!("Invalid constant pool index!");
    }
    return info.unwrap().as_ref();
}

pub fn get_name_and_type<'a>(this:Rc<ConstantPool>,index:usize) -> (&'a str,&'a str) {
    let info = get_constant_info(&this,index);
    let mut inf = &ConstantNameAndTypeInfo{ name_index: 0, desc_index: 0 };
    unsafe {
        let (data, _v_table) : (usize, usize) =  mem::transmute(info);
        let p = data as * const () as * const ConstantNameAndTypeInfo;
        inf = &(*p);
    }
    let name = get_utf8(this.clone(),inf.name_index as usize);
    let desc = get_utf8(this,inf.desc_index as usize);
    return (name,desc);
}

pub fn get_class_name<'a>(this:Rc<ConstantPool>,index:usize) -> &'a str {
    let info = get_constant_info(&this,index);
    unsafe {
        let (data, _v_table) : (usize, usize) =  mem::transmute(info);
        let p = data as * const () as * const ConstantClassInfo;
        return get_utf8(this,*&(*p).name_index as usize);
    }
}

pub fn get_utf8<'a>(this:Rc<ConstantPool>,index:usize) -> &'a str {
    let info = get_constant_info(&this,index);
    unsafe {
        let (data, _v_table) : (usize, usize) =  mem::transmute(info);
        let p = data as * const () as * const ConstantUtf8Info;
        return &(*p).val.as_str();
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

pub trait ConstantInfo {
    fn read_info(&mut self,reader: &mut ClassReader);
}

pub fn read_constant_info(reader:&mut ClassReader,cp:Rc<ConstantPool>) -> Box<dyn ConstantInfo> {
    let dat = display_16(reader.data.clone());
    let tag = reader.read_u8();
    let mut constant_info = new(tag,cp);
    constant_info.read_info(reader);
    return constant_info;
}

pub fn new(tag:u8, cp: Rc<ConstantPool>) -> Box<dyn ConstantInfo> {
    let constant_info:Box<dyn ConstantInfo> = match ConstantInfoTag::from(tag) {
        ConstantInfoTag::ConstantUtf8 => Box::new(ConstantUtf8Info{ val: String::new() }),
        ConstantInfoTag::ConstantInteger => Box::new(ConstantIntegerInfo{ val: 0 }),
        ConstantInfoTag::ConstantFloat => Box::new(ConstantFloatInfo{ val: 0.0 }),
        ConstantInfoTag::ConstantLong => Box::new(ConstantLongInfo{ val: 0 }),
        ConstantInfoTag::ConstantDouble => Box::new(ConstantDoubleInfo{ val: 0.0 }),
        ConstantInfoTag::ConstantClass => Box::new(ConstantClassInfo{ cp:cp.clone(), name_index: 0 }),
        ConstantInfoTag::ConstantString => Box::new(ConstantStringInfo{ cp:cp.clone(), string_index: 0 }),
        ConstantInfoTag::ConstantFieldref => Box::new(ConstantFieldRefInfo(ConstantMemberRefInfo{
            cp:cp.clone(),
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantMethodref => Box::new(ConstantMethodRefInfo(ConstantMemberRefInfo{
            cp:cp.clone(),
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantInterfaceMethodref => Box::new(ConstantInterfaceMethodRefInfo(ConstantMemberRefInfo{
            cp,
            class_index: 0,
            name_and_type_index: 0
        })),
        ConstantInfoTag::ConstantNameAndType => Box::new(ConstantNameAndTypeInfo{ name_index: 0,desc_index: 0 }),
        //待完善
        ConstantInfoTag::ConstantMethodHandle => Box::new(ConstantMethodHandleInfo{
            reference_kind: 0, reference_index: 0
        }),
        ConstantInfoTag::ConstantMethodType => Box::new(ConstantMethodTypeInfo{ descriptor_index: 0 }),
        ConstantInfoTag::ConstantInvokeDynamic => Box::new(ConstantInvokeDynamicInfo{
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::class_file::constant_pool::tests::Number::{Int, Lon};
    use std::mem;

    trait Num {
        fn num(&self) -> i64;
    }

    struct Integer {
        val:i32,
        string:String
    }

    impl Integer {
        pub fn to_string(&self) -> String {
            return String::from_utf8(vec![self.val as u8]).unwrap();
        }
    }

    impl Num for Integer {
        fn num(&self) -> i64 {
            return self.val as i64;
        }
    }

    struct Long {
        val:i64
    }

    impl Num for Long {
        fn num(&self) -> i64 {
            return self.val;
        }
    }

    enum Number {
        Int(Integer),
        Lon(Long)
    }

    fn get_constant_info(this: &Vec<Number>, index:usize) -> &Number {
        let info = this.get(index-1);
        if info.is_none() {
            panic!("Invalid constant pool index!");
        }
        return info.unwrap();
    }

    pub fn get_utf8<'a>(this:Rc<Vec<Number>>,index:usize) -> &'a str {
        let info = get_constant_info(this.as_ref(),index);
        let utf8 = match info {
            Int(utf) => utf,
            _ => panic!("info is not NameAndType")
        };
        return utf8.string.as_str();
    }

    fn get_constant_info_1(this: &Vec<Box<dyn Num>>, index:usize) -> &dyn Num {
        let info = this.get(index);
        if info.is_none() {
            panic!("Invalid constant pool index!");
        }
        return info.unwrap().as_ref();
    }

    pub fn get_utf8_1<'a>(this:Rc<Vec<Box<dyn Num>>>,index:usize) -> &'a str {
        let info = get_constant_info_1(this.as_ref(),index);
        unsafe {
            let (data, _v_table) : (usize, usize) =  mem::transmute(info);
            let p = data as * const () as * const Integer;
            return &(*p).string.as_str();
        }
    }

//    #[test]
//    fn test() {
//        let mut vec = Vec::new();
//        vec.push(Int(Integer{ val: 0, string: "0".to_string() }));
//        vec.push(Lon(Long{ val: 1 }));
//        vec.push(Int(Integer{ val: 2, string: "2".to_string() }));
//        vec.push(Lon(Long{ val: 3 }));
//        vec.push(Int(Integer{ val: 4,string: "4".to_string() }));
//        vec.push(Lon(Long{ val: 5 }));
//        let rc = Rc::new(vec);
//        let ss = get_utf8(rc,4);
//        println!("{}",ss);
//    }

    #[test]
    fn test_1() {
        let mut vec:Vec<Box<dyn Num>> = Vec::new();
        vec.push(Box::new (Integer{ val: 0, string: "0".to_string() }));
        vec.push(Box::new(Long{ val: 1 }));
        vec.push(Box::new(Integer{ val: 2, string: "2".to_string() }));
        vec.push(Box::new(Long{ val: 3 }));
        vec.push(Box::new(Integer{ val: 4,string: "4".to_string() }));
        vec.push(Box::new(Long{ val: 5 }));
        let rc = Rc::new(vec);
        let ss = get_utf8_1(rc,4);
        println!("{}",ss);
    }
}