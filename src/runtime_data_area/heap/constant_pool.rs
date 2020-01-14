use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::constant_pool::{ConstantPool as Pool, ConstantInfoEnum};
use crate::runtime_data_area::heap::constant_pool::Constant::*;

pub struct ConstantPool {
    class:Rc<Class>,
    constants:Vec<Constant>
}

impl ConstantPool {
    pub fn new_constant_pool(class:Rc<Class>,pool:Pool) -> ConstantPool {
        let size = pool.len();
        let mut constants = Vec::with_capacity(size);
        let mut index = 0usize;
        while index < size {
            let info_enum = pool.get_info(index).unwrap();
            let constant = match info_enum {
                ConstantInfoEnum::Integer(info) => Integer(info.val()),
                ConstantInfoEnum::Float(info) => Float(info.val()),
                ConstantInfoEnum::Long(info) => Long(info.val()),
                ConstantInfoEnum::Double(info) => Double(info.val()),
                ConstantInfoEnum::Str(info) => String(info.string()),
                _ => {}
            };
            match constant {
                Long(val) | Double(val) => {
                    constants.push(constant);
                    constants.push(None);
                    i += 1;
                },
                _ => constants.push(constant)
            }
        }
        return ConstantPool{ class, constants };
    }
}

pub enum Constant {
    None,
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
    Class(),
    FieldRef(),
    MethodRef(),
    InterfaceMethodRef()
}