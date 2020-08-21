use crate::oops::class::{Class, WeakClass};
use crate::oops::class_ref::ClassRef;
use crate::oops::constant_pool::Constant::*;

use crate::oops::field_ref::FieldRef;
use crate::oops::interface_method_ref::InterfaceMethodRef;

use crate::oops::method_ref::MethodRef;
use core::mem;
use lark_classfile::constant_pool::{ConstantInfoEnum, ConstantPool as Pool};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ConstantPool {
    class: Option<WeakClass>,
    constants: Vec<Constant>,
}

impl ConstantPool {
    pub fn new_constant_pool(
        class: Option<Rc<RefCell<Class>>>,
        pool: Rc<RefCell<Pool>>,
    ) -> ConstantPool {
        let borrow_pool = (*pool).borrow();
        let size = borrow_pool.len();
        let mut constants = Vec::with_capacity(size);
        let mut index = 0usize;

        while index < size {
            let info_enum = borrow_pool.get_info(index).unwrap();
            let constant = match info_enum {
                ConstantInfoEnum::Integer(info) => Integer(info.val()),
                ConstantInfoEnum::Float(info) => Float(info.val()),
                ConstantInfoEnum::Long(info) => Long(info.val()),
                ConstantInfoEnum::Double(info) => Double(info.val()),
                ConstantInfoEnum::Str(info) => Str(info.string().to_string()),
                ConstantInfoEnum::Class(info) => ClassReference(ClassRef::new_class_ref(info)),
                ConstantInfoEnum::FieldRef(info) => FieldReference(FieldRef::new_field_ref(info)),
                ConstantInfoEnum::MethodRef(info) => {
                    MethodReference(MethodRef::new_method_ref(info))
                }
                ConstantInfoEnum::InterfaceMethodRef(info) => {
                    InterfaceMethodReference(InterfaceMethodRef::new_method_ref(info))
                }
                _ => None,
            };
            match constant {
                Long(_) | Double(_) => {
                    constants.push(constant);
                    constants.push(None);
                    index += 1;
                }
                //                None => {},  must be annotated
                _ => constants.push(constant),
            }
            index += 1;
        }
        let pool = ConstantPool { class, constants };
        return pool;
    }

    //    pub fn lazy_init_for_constants(&mut self, class: &Rc<RefCell<Class>>) {
    //        for constant in &mut self.constants {
    //            match constant {
    //                ClassReference(c) => c.set_holder(class.clone()),
    //                FieldReference(c) => c.set_holder(class.clone()),
    //                MethodReference(c) => c.set_holder(class.clone()),
    //                InterfaceMethodReference(c) => c.set_holder(class.clone()),
    //                _ => {}
    //            }
    //        }
    //    }

    pub fn get_constant(&mut self, index: usize) -> &mut Constant {
        let constant = self.constants.get_mut(index - 1);
        if constant.is_none() {
            panic!("No constants at index {}", index);
        }
        return constant.unwrap();
    }

    pub fn take_constant(&mut self, index: usize) -> Constant {
        let constant = self.constants.get_mut(index - 1);
        if constant.is_none() {
            panic!("No constants at index {}", index);
        }
        return constant.unwrap().take();
    }

    pub fn restoration_constant(&mut self, index: usize, other: Constant) {
        let constant = self.constants.get_mut(index - 1);
        if constant.is_none() {
            panic!("No constants at index {}", index);
        }
        constant.unwrap().replace(other);
    }

    pub fn get_constant_immutable(&self, index: usize) -> &Constant {
        let constant = self.constants.get(index - 1);
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
    pub fn set_class(&mut self, class: WeakClass) {
        return self.class = Some(class);
    }

    /// for debug
    #[inline]
    pub fn size(&self) -> usize {
        return self.constants.len();
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        return ConstantPool {
            class: Option::None,
            constants: vec![],
        };
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
    InterfaceMethodReference(InterfaceMethodRef),
}

impl Constant {
    pub fn take(&mut self) -> Constant {
        return mem::take(self);
    }

    pub fn replace(&mut self, mut other: Self) {
        mem::swap(self, &mut other)
    }
}

impl Default for Constant {
    fn default() -> Self {
        Constant::None
    }
}
