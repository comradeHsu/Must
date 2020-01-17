use crate::runtime_data_area::slot::Slot;
use crate::utils::numbers::{f32_to_i32, i32_to_f32, i64_back_bytes_to_i32, f64_to_i64, i64_to_f64};
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::object::Object;

#[derive(Debug)]
pub struct Slots {
    slots:Vec<Slot>
}

impl Slots {

    pub fn new(slots:Vec<Slot>) -> Slots {
        return Slots{slots};
    }

    pub fn with_capacity(capacity:usize) -> Slots {
        return Slots{ slots: Vec::with_capacity(capacity)};
    }

    pub fn set_int(&mut self,index:usize,val:i32) {
        self.slots[index].num = Some(val);
    }

    pub fn get_int(&self,index:usize) -> i32{
        if self.slots[index].num.is_none() {
            panic!("slot is reference");
        }
        return self.slots[index].num.unwrap();
    }

    pub fn set_float(&mut self,index:usize,val:f32) {
        self.slots[index].num = Some(f32_to_i32(val));
    }

    pub fn get_float(&self,index:usize) -> f32{
        if self.slots[index].num.is_none() {
            panic!("slot is reference");
        }
        return i32_to_f32(self.slots[index].num.unwrap());
    }

    pub fn set_long(&mut self,index:usize,val:i64) {
        self.slots[index].num = Some(i64_back_bytes_to_i32(val));
        self.slots[index+1].num = Some((val >> 32) as i32);
    }

    pub fn get_long(&self,index:usize) -> i64 {
        if self.slots[index].num.is_none() || self.slots[index+1].num.is_none(){
            panic!("slot is reference");
        }
        let low = self.slots[index].num.unwrap() as i64;
        let high = self.slots[index+1].num.unwrap() as i64;
        return (high << 32) | low;
    }

    pub fn set_double(&mut self,index:usize,val:f64) {
        self.set_long(index,f64_to_i64(val));
    }

    pub fn get_double(&self,index:usize) -> f64 {
        return i64_to_f64(self.get_long(index));
    }

    pub fn set_ref(&mut self,index:usize,val:Option<Rc<RefCell<Object>>>) {
        self.slots[index].reference = val;
    }

    pub fn get_ref(&self,index:usize) -> Option<Rc<RefCell<Object>>> {
        if self.slots[index].reference.is_none() {
            panic!("slot is number");
        }
        return self.slots[index].reference.clone();
    }
}
