use crate::runtime_data_area::heap::class::Class;
use std::rc::Rc;
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::slot::Slot;
use std::cell::RefCell;

//#[derive(Debug, PartialEq)]
pub struct Object {
    class:Rc<RefCell<Class>>,
    fields:Option<Slots>
}

impl Object {
    pub fn new(class:&Rc<RefCell<Class>>) -> Object {
        let count = (*class).borrow().instance_slot_count();
        return Object{
            class: class.clone(),
            fields: Some(Slots::with_capacity(count as usize))
        };
    }

    // getters
    pub fn class(&self) -> Rc<RefCell<Class>> {
        return self.class.clone();
    }
    pub fn fields(&mut self) -> &mut Slots {
        return self.fields.as_mut().unwrap();
    }
}