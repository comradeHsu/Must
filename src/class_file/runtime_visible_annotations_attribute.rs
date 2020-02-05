use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::runtime_visible_annotations_attribute::ElementValue::*;

pub struct AnnotationsAttribute {
    cp:Rc<RefCell<ConstantPool>>,
    annotations:Vec<AnnotationAttribute>
}

#[derive(Debug,Clone)]
pub struct AnnotationAttribute {
    type_name:String,
    num_element_value_pairs:u16,
    element_value_pairs:Vec<ElementValuePair>
}

#[derive(Debug,Clone)]
struct ElementValuePair {
    element_name:String,
    element_value:ElementValue
}

#[derive(Debug,Clone)]
enum ElementValue {
    ArrayValue(Vec<ElementValue>),
    EnumConstValue(String,String),
    IntConstValue(i32),
    StringConstValue(String),
    BoolConstValue(bool),
    ByteConstValue(i8),
    ShortConstValue(i16),
    LongConstValue(i64),
    CharConstValue(char),
    FloatConstValue(f32),
    DoubleConstValue(f64),
    ClassConstValue(String)
}

impl AnnotationsAttribute {
    #[inline]
    pub fn with_cp(cp:Rc<RefCell<ConstantPool>>) -> Self {
        return AnnotationsAttribute{
            cp,
            annotations: vec![]
        };
    }

    #[inline]
    pub fn annotations(&self) -> &Vec<AnnotationAttribute> {
        for annotation in &self.annotations {
            println!("annotation:{}",annotation.type_name)
        }
        return &self.annotations;
    }
}

impl AttributeInfo for AnnotationsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let num_annotations = reader.read_u16() as usize;
        let mut annotations = Vec::with_capacity(num_annotations);
        for _ in 0..num_annotations {
            annotations.push(AnnotationAttribute::new(reader,self.cp.clone()))
        }
    }
}

impl AnnotationAttribute {
    fn new(reader: &mut ClassReader,cp:Rc<RefCell<ConstantPool>>) -> Self {
        let type_index = reader.read_u16();
        let name = (*cp).borrow().get_utf8(type_index as usize).to_string();
        let num_element_value_pairs = reader.read_u16();
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(ElementValuePair::new(reader,cp.clone()));
        }
        return AnnotationAttribute{
            type_name: name,
            num_element_value_pairs,
            element_value_pairs
        };
    }
}

impl ElementValuePair {
    fn new(reader: &mut ClassReader,cp:Rc<RefCell<ConstantPool>>) -> Self {
        let element_name_index = reader.read_u16() as usize;
        let tag = reader.read_char();
        let pool = (*cp).borrow();
        let element_name = pool.get_constant_info(element_name_index);
        let const_value_index = reader.read_u16() as usize;
        let constant = pool.get_constant_info(const_value_index);
        let value = match tag {
            '[' => ArrayValue(ElementValue::array_value(reader,cp.clone())),
            's' => StringConstValue(constant.string()),
            'c' => ClassConstValue(constant.string()),
            'e' => {
                let const_name_index = reader.read_u16() as usize;
                let constant_name = pool.get_constant_info(const_name_index);
                EnumConstValue(constant.string(),constant_name.string())
            },
            'Z' => BoolConstValue(integer_to_bool(constant.integer())),
            'B' => ByteConstValue(constant.integer() as i8),
            'S' => ShortConstValue(constant.integer() as i16),
            'I' => IntConstValue(constant.integer()),
            'J' => LongConstValue(constant.long()),
            'F' => FloatConstValue(constant.float()),
            'D' => DoubleConstValue(constant.double()),
            'C' => CharConstValue(constant.integer() as u8 as char),
            _ => panic!("The invalid annotation value tag!")
        };
        return ElementValuePair{
            element_name: element_name.string(),
            element_value: value
        };
    }
}

impl ElementValue {
    pub fn array_value(reader: &mut ClassReader,cp:Rc<RefCell<ConstantPool>>) -> Vec<Self> {
        let num_values = reader.read_u16() as usize;
        let mut values = Vec::with_capacity(num_values);
        let pool = (*cp).borrow();
        for _ in 0..num_values {
            let tag = reader.read_char();
            let const_value_index = reader.read_u16() as usize;
            let constant = pool.get_constant_info(const_value_index);
            let value = match tag {
                '[' => panic!("The invlid annotation value tag '[' !"),
                's' => StringConstValue(constant.string()),
                'c' => ClassConstValue(constant.string()),
                'e' => {
                    let const_name_index = reader.read_u16() as usize;
                    let constant_name = pool.get_constant_info(const_name_index);
                    EnumConstValue(constant.string(),constant_name.string())
                },
                'Z' => BoolConstValue(integer_to_bool(constant.integer())),
                'B' => ByteConstValue(constant.integer() as i8),
                'S' => ShortConstValue(constant.integer() as i16),
                'I' => IntConstValue(constant.integer()),
                'J' => LongConstValue(constant.long()),
                'F' => FloatConstValue(constant.float()),
                'D' => DoubleConstValue(constant.double()),
                'C' => CharConstValue(constant.integer() as u8 as char),
                _ => panic!("The invlid annotation value tag!")
            };
            values.push(value);
        }
        return values;

    }
}

#[inline]
fn integer_to_bool(int:i32) -> bool {
    return int == 1;
}