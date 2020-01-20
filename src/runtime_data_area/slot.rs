use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::object::Object;

#[derive(Clone,Debug)]
pub struct Slot {
    pub num:Option<i32>,
    pub reference:Option<Rc<RefCell<Object>>>
}

impl Slot {

    #[inline]
    pub fn new() -> Slot {
        return Slot{ num: None, reference: None };
    }

    #[inline]
    pub fn with_num(num:i32) -> Slot {
        return Slot{ num: Some(num), reference: None };
    }

    #[inline]
    pub fn with_ref(reference:Option<Rc<RefCell<Object>>>) -> Slot {
        return Slot{ num: None, reference };
    }

    #[inline]
    pub fn set_num(&mut self, num:i32) {
        self.num = Some(num);
    }

    #[inline]
    pub fn get_num(&self) -> i32{
        return self.num.unwrap();
    }

    #[inline]
    pub fn set_ref(&mut self, reference:Option<Rc<RefCell<Object>>>) {
        self.reference = reference;
    }

    #[inline]
    pub fn get_ref(&self) -> Option<Rc<RefCell<Object>>> {
        return self.reference.clone();
    }
}