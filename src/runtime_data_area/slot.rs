use crate::runtime_data_area::heap::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Slot {
    pub num: i32,
    pub reference: Option<Rc<RefCell<Object>>>,
}

impl Slot {
    #[inline]
    pub fn new() -> Slot {
        return Slot {
            num: 0,
            reference: None,
        };
    }

    #[inline]
    pub fn with_num(num: i32) -> Slot {
        return Slot {
            num,
            reference: None,
        };
    }

    #[inline]
    pub fn with_ref(reference: Option<Rc<RefCell<Object>>>) -> Slot {
        return Slot { num: 0, reference };
    }

    #[inline]
    pub fn set_num(&mut self, num: i32) {
        self.num = num;
    }

    #[inline]
    pub fn get_num(&self) -> i32 {
        return self.num;
    }

    #[inline]
    pub fn set_ref(&mut self, reference: Option<Rc<RefCell<Object>>>) {
        self.reference = reference;
    }

    #[inline]
    pub fn get_ref(&self) -> Option<Rc<RefCell<Object>>> {
        return self.reference.clone();
    }
}
