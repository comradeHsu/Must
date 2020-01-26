use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::slot::Slot;
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};
use crate::runtime_data_area::heap::object::DataType::StandardObject;

#[derive(Debug)]
pub struct Object {
    pub class:Rc<RefCell<Class>>,
    pub data:DataType
}

impl Object {
    pub fn new(class:Rc<RefCell<Class>>) -> Object {
        let count = (*class).borrow().instance_slot_count();
        return Object{
            class: class.clone(),
            data: StandardObject(Some(Slots::with_capacity(count as usize)))
        };
    }

    #[inline]
    pub fn class(&self) -> Rc<RefCell<Class>> {
        return self.class.clone();
    }
    #[inline]
    pub fn fields(&mut self) -> &mut Slots {
        let fields = &mut self.data;
        match fields {
            StandardObject(data) => data.as_mut().unwrap(),
            _ => panic!("The Object is array")
        }
    }

    #[inline]
    pub fn is_instance_of(&self, class:Rc<RefCell<Class>>) -> bool {
        return (*class).borrow().is_assignable_from(self.class.as_ref().borrow().borrow());
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let l = self as *const Object;
        let r = other as *const Object;
        if l == r {
            return true;
        }
        return false;
    }
}

#[derive(Debug)]
pub enum DataType {
    StandardObject(Option<Slots>),
    Bytes(Vec<i8>),
    Shorts(Vec<i16>),
    Ints(Vec<i32>),
    Longs(Vec<i64>),
    Chars(Vec<u16>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
    References(Vec<Rc<RefCell<Object>>>)
}