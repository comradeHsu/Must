use crate::class_file::class_reader::ClassReader;
use std::any::Any;
use std::mem;
use std::rc::Rc;
use crate::class_file::makers_attribute::DeprecatedAttribute;
use crate::class_file::member_info::display_16;
use crate::class_file::constant_pool::ConstantInfoEnum::*;
use std::cell::RefCell;
use std::borrow::Borrow;

//pub type ConstantPool = Vec<ConstantInfoEnum>;

pub struct ConstantPool {
    vec:Vec<ConstantInfoEnum>
}

impl ConstantPool {

    pub fn new() -> ConstantPool {
        return ConstantPool{ vec: vec![] };
    }

    pub fn read_constant_pool(reader: &mut ClassReader) -> Rc<RefCell<ConstantPool>> {
        let cp_count = reader.read_u16();
        println!("pool count:{}",cp_count);
        let mut cp = Rc::new(RefCell::new(ConstantPool::new()));
        let mut vec: Vec<ConstantInfoEnum> = Vec::new();
        let mut i = 1;
        while i < cp_count {
            let constant_info = read_constant_info(reader, cp.clone());
            match &constant_info {
                Long(info) => {
                    i = i + 2;
                    vec.push(constant_info);
                    vec.push(None);
                    println!("Long");
                    continue;
                },
                Double(info) => {
                    i = i + 2;
                    vec.push(constant_info);
                    vec.push(None);
                    println!("Double");
                    continue;
                },
                _ => {}
            }
            vec.push(constant_info);
            i += 1;
        }
        let mut count = 0;
        for info in &vec {
            match info {
                None => println!("c:{}-none",count),
                Utf8(x) => println!("c:{}-utf-8",count),
                Integer(x)=> println!("c:{}-int",count),
                Float(x)=> println!("c:{}-float",count),
                Long(x)=> println!("c:{}-long",count),
                Double(x)=> println!("c:{}-double",count),
                Class(x)=> println!("c:{}-class",count),
                Str(x)=> println!("c:{}-str",count),
                FieldRef(x)=> println!("c:{}-field",count),
                MethodRef(x)=> println!("c:{}-method",count),
                InterfaceMethodRef(x)=> println!("c:{}-interface_method",count),
                NameAndType(x)=> println!("c:{}-name and type",count),
                MethodHandle(x)=> println!("c:{}-method handle",count),
                MethodType(x)=> println!("c:{}-method type",count),
                InvokeDynamic(x)=> println!("c:{}-invoke",count)
            }
            count+=1;
        }
        let mut c = Rc::new(RefCell::new(ConstantPool{vec}));
        ConstantPool::post_constant_pool(c.clone());
        return c;
    }

    fn post_constant_pool(rc_pool:Rc<RefCell<ConstantPool>>) {
        let clone_pool = rc_pool.clone();
        let pool = &mut (*clone_pool).borrow_mut().vec;
        for c in pool {
            match c {
                Class(info) => info.cp = rc_pool.clone(),
                Str(info) => info.cp = rc_pool.clone(),
                FieldRef(info) => info.0.cp = rc_pool.clone(),
                MethodRef(info) => info.0.cp = rc_pool.clone(),
                MethodRef(info) => info.0.cp = rc_pool.clone(),
                InterfaceMethodRef(info) => info.0.cp = rc_pool.clone(),
                _ => {}
            }
        }
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
        println!("info:{:?}",index);
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

    #[inline]
    pub fn get_info(&self,index:usize) -> Option<&ConstantInfoEnum> {
        return self.vec.get(index);
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

pub fn read_constant_info(reader:&mut ClassReader,cp:Rc<RefCell<ConstantPool>>) -> ConstantInfoEnum {
    let dat = display_16(reader.data.clone());
    let tag = reader.read_u8();
    let mut constant_info = new(tag,cp);
    constant_info.read_info(reader);
    return constant_info;
}

pub fn new(tag:u8, cp: Rc<RefCell<ConstantPool>>) -> ConstantInfoEnum {
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

pub struct ConstantIntegerInfo {
    val:i32
}

impl ConstantIntegerInfo {
    #[inline]
    pub fn val(&self) -> i32 {
        return self.val;
    }

    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.val = reader.read_u32() as i32;
    }
}

impl ConstantInfo for ConstantIntegerInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

pub struct ConstantFloatInfo {
    val:f32
}

impl ConstantFloatInfo{
    #[inline]
    pub fn val(&self) -> f32 {
        return self.val;
    }

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

pub struct ConstantLongInfo {
    val:i64
}

impl ConstantLongInfo{
    #[inline]
    pub fn val(&self) -> i64 {
        return self.val;
    }

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

pub struct ConstantDoubleInfo {
    val:f64
}

impl ConstantDoubleInfo{

    #[inline]
    pub fn val(&self) -> f64 {
        return self.val;
    }

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

pub struct ConstantUtf8Info {
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

pub struct ConstantStringInfo {
    cp:Rc<RefCell<ConstantPool>>,
    string_index:u16
}

impl ConstantStringInfo {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.string_index = reader.read_u16();
    }

    pub fn string(&self) -> String {
        let borrow = (*self.cp).borrow();
        return borrow.get_utf8(self.string_index as usize).to_owned();
    }
}

impl ConstantInfo for ConstantStringInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

pub struct ConstantClassInfo {
    cp:Rc<RefCell<ConstantPool>>,
    name_index:u16
}

impl ConstantClassInfo {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.name_index = reader.read_u16();
    }

    pub fn name(&self) -> String {
        let borrow = (*self.cp).borrow();
        return borrow.get_utf8(self.name_index as usize).to_owned();
    }
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

pub struct ConstantNameAndTypeInfo {
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

pub struct ConstantMemberRefInfo {
    cp:Rc<RefCell<ConstantPool>>,
    class_index:u16,
    name_and_type_index:u16
}

impl ConstantMemberRefInfo {
    pub fn read_info(&mut self,reader:&mut ClassReader){
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();
    }

    pub fn class_name(&self) -> String {
        let borrow = (*self.cp).borrow();
        return borrow.get_class_name(self.class_index as usize).to_owned();
    }

    pub fn name_and_descriptor(&self) -> (String,String) {
        let borrow = (*self.cp).borrow();
        let (f,s)  = borrow.get_name_and_type(self.name_and_type_index as usize);
        return (f.to_owned(),s.to_owned())
    }
}

impl ConstantInfo for ConstantMemberRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.read_info(reader);
    }
}

pub struct ConstantFieldRefInfo(ConstantMemberRefInfo);

impl ConstantFieldRefInfo {
    #[inline]
    pub fn get_member_ref(&self) -> &ConstantMemberRefInfo {
        return &self.0;
    }
}

impl ConstantInfo for ConstantFieldRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}

pub struct ConstantMethodRefInfo(ConstantMemberRefInfo);

impl ConstantMethodRefInfo {
    #[inline]
    pub fn get_member_ref(&self) -> &ConstantMemberRefInfo {
        return &self.0;
    }
}

impl ConstantInfo for ConstantMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}

pub struct ConstantInterfaceMethodRefInfo(ConstantMemberRefInfo);

impl ConstantInterfaceMethodRefInfo {
    #[inline]
    pub fn get_member_ref(&self) -> &ConstantMemberRefInfo {
        return &self.0;
    }
}

impl ConstantInfo for ConstantInterfaceMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.0.read_info(reader);
    }
}

pub struct ConstantInvokeDynamicInfo {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16
}

impl ConstantInfo for ConstantInvokeDynamicInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.bootstrap_method_attr_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();
    }
}

pub struct ConstantMethodHandleInfo {
    reference_kind:u8,
    reference_index:u16
}

impl ConstantInfo for ConstantMethodHandleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.reference_kind = reader.read_u8();
        self.reference_index = reader.read_u16();
    }
}

pub struct ConstantMethodTypeInfo {
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

    pub trait Num {
        fn num(&self) -> i64;
    }

    pub struct Integer {
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

    pub struct Long {
        val:i64
    }

    impl Num for Long {
        fn num(&self) -> i64 {
            return self.val;
        }
    }

    pub enum Number {
        Int(Integer),
        Lon(Long)
    }

    pub fn get_constant_info(this: &Vec<Number>, index:usize) -> &Number {
        let info = this.get(index);
        if info.is_none() {
            panic!("Invalid constant pool index!");
        }
        return info.unwrap();
    }

    pub fn get_utf8(this:&Rc<Vec<Number>>,index:usize) -> & str {
        let info = get_constant_info(this.as_ref(),index);
        let utf8 = match info {
            Int(utf) => utf,
            _ => panic!("info is not NameAndType")
        };
        return utf8.string.as_str();
    }

    pub fn get_constant_info_1(this: &Vec<Box<dyn Num>>, index:usize) -> &dyn Num {
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

    #[test]
    fn test() {
        let mut vec = Vec::new();
        vec.push(Int(Integer{ val: 0, string: "0".to_string() }));
        vec.push(Lon(Long{ val: 1 }));
        vec.push(Int(Integer{ val: 2, string: "2".to_string() }));
        vec.push(Lon(Long{ val: 3 }));
        vec.push(Int(Integer{ val: 4,string: "4".to_string() }));
        vec.push(Lon(Long{ val: 5 }));
        let rc = Rc::new(vec);
        let ss = get_utf8(&rc,4);
        println!("{}",ss);
    }

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