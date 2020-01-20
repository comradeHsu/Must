use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::slot::Slot;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Error};

#[derive(Debug)]
pub struct Object {
    class:Rc<RefCell<Class>>,
    fields:Option<Slots>
}

impl Object {
    pub fn new(class:Rc<RefCell<Class>>) -> Object {
        let count = (*class).borrow().instance_slot_count();
        return Object{
            class: class.clone(),
            fields: Some(Slots::with_capacity(count as usize))
        };
    }

    #[inline]
    pub fn class(&self) -> Rc<RefCell<Class>> {
        return self.class.clone();
    }
    #[inline]
    pub fn fields(&mut self) -> &mut Slots {
        return self.fields.as_mut().unwrap();
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