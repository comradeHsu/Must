use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::constant_pool::{ConstantPool as Pool, ConstantInfoEnum};
use crate::runtime_data_area::heap::constant_pool::Constant::*;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class_ref::ClassRef;
use core::mem;
use crate::runtime_data_area::heap::field_ref::FieldRef;
use crate::runtime_data_area::heap::method_ref::MethodRef;

pub struct ConstantPool {
    class:Rc<RefCell<Class>>,
    constants:Vec<Constant>
}

impl ConstantPool {

    fn none() -> ConstantPool {
        return ConstantPool{
            class: Rc::new(RefCell::new(Class::none())),
            constants: vec![]
        };
    }

    pub fn new_constant_pool(class:Rc<RefCell<Class>>,pool:Pool) -> Rc<ConstantPool> {
        let size = pool.len();
        let mut constants = Vec::with_capacity(size);
        let mut index = 0usize;
        let mut cp = Rc::new(ConstantPool::none());
        while index < size {
            let info_enum = pool.get_info(index).unwrap();
            let constant = match info_enum {
                ConstantInfoEnum::Integer(info) => Integer(info.val()),
                ConstantInfoEnum::Float(info) => Float(info.val()),
                ConstantInfoEnum::Long(info) => Long(info.val()),
                ConstantInfoEnum::Double(info) => Double(info.val()),
                ConstantInfoEnum::Str(info) => Str(info.string().to_string()),
                ConstantInfoEnum::Class(info) => {
                    ClassReference(ClassRef::new_class_ref(cp.clone(),info))
                },
                ConstantInfoEnum::FieldRef(info) => {
                    FieldReference(FieldRef::new_field_ref(cp.clone(),info))
                },
                ConstantInfoEnum::MethodRef(info) => {
                    MethodReference(MethodRef::new_method_ref(cp.clone(),info))
                },
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
        let mut pool = Rc::new(ConstantPool{ class, constants });
        mem::swap(&mut pool,&mut cp);
        return cp;
    }

    pub fn get_constant(&self, index:usize) -> &Constant {
        let constant = self.constants.get(index);
        if constant.is_none() {
            panic!("No constants at index {}", index);
        }
        return constant.unwrap();
    }

    pub fn class(&self) -> &Rc<RefCell<Class>> {
        return &self.class;
    }
}

pub enum Constant {
    None,
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Str(String),
    ClassReference(ClassRef),
    FieldReference(FieldRef),
    MethodReference(MethodRef),
    InterfaceMethodRef()
}