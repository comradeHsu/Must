use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::class_file::constant_pool::{ConstantPool as Pool, ConstantInfoEnum};
use crate::runtime_data_area::heap::constant_pool::Constant::*;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class_ref::ClassRef;
use core::mem;
use crate::runtime_data_area::heap::field_ref::FieldRef;
use crate::runtime_data_area::heap::method_ref::MethodRef;
use crate::runtime_data_area::heap::interface_method_ref::InterfaceMethodRef;

#[derive(Debug)]
pub struct ConstantPool {
    class:Option<Rc<RefCell<Class>>>,
    constants:Vec<Constant>
}

impl ConstantPool {

    pub fn none() -> ConstantPool {
        return ConstantPool{
            class: Option::None,
            constants: vec![]
        };
    }

    pub fn new_constant_pool(class:Option<Rc<RefCell<Class>>>,pool:Rc<RefCell<Pool>>) -> Rc<RefCell<ConstantPool>> {
        let borrow_pool = (*pool).borrow();
        let size = borrow_pool.len();
        let mut constants = Vec::with_capacity(size);
        let mut index = 0usize;
        let mut cp = Rc::new(RefCell::new(ConstantPool::none()));
        println!("step-1");
        while index < size {
            let info_enum = borrow_pool.get_info(index).unwrap();
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
                ConstantInfoEnum::InterfaceMethodRef(info) => {
                    InterfaceMethodReference(InterfaceMethodRef::new_method_ref(cp.clone(),info))
                },
                _ => None
            };
            match constant {
                Long(_) | Double(_) => {
                    constants.push(constant);
                    constants.push(None);
                    index += 1;
                },
                None => {},
                _ => constants.push(constant)
            }
            index += 1;
        }
        let mut pool = Rc::new(RefCell::new(
            ConstantPool{ class, constants }
        ));
        let clone = pool.clone();
        (*pool).borrow_mut().lazy_init_for_constants(&clone);
        return pool;
    }

    fn lazy_init_for_constants(&mut self,pool:&Rc<RefCell<ConstantPool>>) {
        for constant in &mut self.constants {
            match constant {
                ClassReference(c) => c.set_constant_pool(pool.clone()),
                FieldReference(c) => c.set_constant_pool(pool.clone()),
                MethodReference(c) => c.set_constant_pool(pool.clone()),
                InterfaceMethodReference(c) => c.set_constant_pool(pool.clone()),
                _ => {}
            }
        }
    }

    pub fn get_constant(&mut self, index:usize) -> &mut Constant {
        let constant = self.constants.get_mut(index-1);
        if constant.is_none() {
            panic!("No constants at index {}", index);
        }
        return constant.unwrap();
    }

    pub fn get_constant_immutable(&self, index:usize) -> &Constant {
        let constant = self.constants.get(index-1);
        if constant.is_none() {
            panic!("No constants at index {}", index);
        }
        return constant.unwrap();
    }


    pub fn class(&self) -> Rc<RefCell<Class>> {
        let class = self.class.as_ref().unwrap();
        return class.clone();
    }

    #[inline]
    pub fn set_class(&mut self,class:Rc<RefCell<Class>>) {
        return self.class = Some(class);
    }
}

#[derive(Debug)]
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
    InterfaceMethodReference(InterfaceMethodRef)
}