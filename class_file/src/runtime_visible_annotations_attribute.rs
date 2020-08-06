use crate::attribute_info::AttributeInfo;
use crate::class_reader::ClassReader;
use crate::constant_pool::ConstantPool;
use crate::runtime_visible_annotations_attribute::ElementValue::*;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct AnnotationsAttribute {
    cp: Rc<RefCell<ConstantPool>>,
    annotations: Vec<AnnotationAttribute>,
}

#[derive(Debug, Clone)]
pub struct AnnotationAttribute {
    type_name: String,
    num_element_value_pairs: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone)]
struct ElementValuePair {
    element_name: String,
    element_value: ElementValue,
}

#[derive(Debug, Clone)]
enum ElementValue {
    ArrayValue(Vec<ElementValue>),
    EnumConstValue(String, String),
    IntConstValue(i32),
    StringConstValue(String),
    BoolConstValue(bool),
    ByteConstValue(i8),
    ShortConstValue(i16),
    LongConstValue(i64),
    CharConstValue(char),
    FloatConstValue(f32),
    DoubleConstValue(f64),
    ClassConstValue(String),
}

impl AnnotationsAttribute {
    #[inline]
    pub fn with_cp(cp: Rc<RefCell<ConstantPool>>) -> Self {
        return AnnotationsAttribute {
            cp,
            annotations: vec![],
        };
    }

    #[inline]
    pub fn annotations(&self) -> &Vec<AnnotationAttribute> {
        return &self.annotations;
    }
}

impl AttributeInfo for AnnotationsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let num_annotations = reader.read_u16() as usize;
        let mut annotations = Vec::with_capacity(num_annotations);
        for i in 0..num_annotations {
            annotations.push(AnnotationAttribute::new(reader, self.cp.clone()))
        }
        self.annotations = annotations;
    }
}

impl AnnotationAttribute {
    fn new(reader: &mut ClassReader, cp: Rc<RefCell<ConstantPool>>) -> Self {
        let type_index = reader.read_u16();
        let name = (*cp).borrow().get_utf8(type_index as usize).to_string();
        let num_element_value_pairs = reader.read_u16();
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for i in 0..num_element_value_pairs {
            element_value_pairs.push(ElementValuePair::new(reader, cp.clone()));
        }
        //        println!("AnnotationAttribute:{},len:{}",name,num_element_value_pairs);
        return AnnotationAttribute {
            type_name: name,
            num_element_value_pairs,
            element_value_pairs,
        };
    }

    #[inline]
    pub fn name(&self) -> &str {
        return &self.type_name.as_str();
    }
}

impl ElementValuePair {
    fn new(reader: &mut ClassReader, cp: Rc<RefCell<ConstantPool>>) -> Self {
        let element_name_index = reader.read_u16() as usize;
        let tag = reader.read_char();
        let pool = (*cp).borrow();
        let element_name = pool.get_constant_info(element_name_index);
        let value = match tag {
            '[' => ArrayValue(ElementValue::array_value(reader, cp.clone())),
            's' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                StringConstValue(constant.string())
            }
            'c' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                ClassConstValue(constant.string())
            }
            'e' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                let const_name_index = reader.read_u16() as usize;
                let constant_name = pool.get_constant_info(const_name_index);
                EnumConstValue(constant.string(), constant_name.string())
            }
            'Z' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                BoolConstValue(integer_to_bool(constant.integer()))
            }
            'B' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                ByteConstValue(constant.integer() as i8)
            }
            'S' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                ShortConstValue(constant.integer() as i16)
            }
            'I' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                IntConstValue(constant.integer())
            }
            'J' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                LongConstValue(constant.long())
            }
            'F' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                FloatConstValue(constant.float())
            }
            'D' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                DoubleConstValue(constant.double())
            }
            'C' => {
                let const_value_index = reader.read_u16() as usize;
                let constant = pool.get_constant_info(const_value_index);
                CharConstValue(constant.integer() as u8 as char)
            }
            _ => panic!("The invalid annotation value tag!"),
        };
        return ElementValuePair {
            element_name: element_name.string(),
            element_value: value,
        };
    }
}

impl ElementValue {
    pub fn array_value(reader: &mut ClassReader, cp: Rc<RefCell<ConstantPool>>) -> Vec<Self> {
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
                    EnumConstValue(constant.string(), constant_name.string())
                }
                'Z' => BoolConstValue(integer_to_bool(constant.integer())),
                'B' => ByteConstValue(constant.integer() as i8),
                'S' => ShortConstValue(constant.integer() as i16),
                'I' => IntConstValue(constant.integer()),
                'J' => LongConstValue(constant.long()),
                'F' => FloatConstValue(constant.float()),
                'D' => DoubleConstValue(constant.double()),
                'C' => CharConstValue(constant.integer() as u8 as char),
                _ => panic!("The invlid annotation value tag!"),
            };
            values.push(value);
        }
        return values;
    }
}

#[inline]
fn integer_to_bool(int: i32) -> bool {
    return int == 1;
}
