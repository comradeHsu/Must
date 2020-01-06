use crate::runtime_data_area::slot::Slot;
use crate::runtime_data_area::object::Object;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{Add, Div, Mul, Sub};

pub type LocalVars = Vec<Slot>;

pub trait TraitLocalVars {

    fn new(max_locals:usize) -> Option<LocalVars>;

    fn set_int(&mut self,index:usize, val:i32);

    fn get_int(&self,index:usize) -> i32;

    fn set_float(&mut self,index:usize, val:f32);

    fn get_float(&self,index:usize) -> f32;

    fn set_long(&mut self,index:usize, val:i64);

    fn get_long(&self,index:usize) -> i64;

    fn set_double(&mut self,index:usize, val:f64);

    fn get_double(&self,index:usize) -> f64;

    fn set_ref(&mut self,index:usize, val:Rc<RefCell<Object>>);

    fn get_ref(&self,index:usize) -> Rc<RefCell<Object>>;
}

impl TraitLocalVars for LocalVars {
    fn new(max_locals: usize) -> Option<Vec<Slot>> {
        if max_locals > 0 {
            return Some(Vec::new());
        }
        return None;
    }

    fn set_int(&mut self, index: usize, val: i32) {
        let slot = self.get_mut(index).expect("index is wrong");
        slot.set_num(val);
    }

    fn get_int(&self, index: usize) -> i32 {
        let slot = self.get(index).expect("index is wrong");
        return slot.get_num();
    }

    fn set_float(&mut self, index: usize, val: f32) {
        let slot = self.get_mut(index).expect("index is wrong");
        slot.set_num(f32_to_i32(val));
    }

    fn get_float(&self, index: usize) -> f32 {
        let slot = self.get(index).expect("index is wrong");
        return i32_to_f32(slot.get_num());
    }

    fn set_long(&mut self, index: usize, val: i64) {
        let slot = self.get_mut(index).expect("index is wrong");
        slot.set_num(val as i32);
        let next_slot = self.get_mut(index+1).expect("index is wrong");
        next_slot.set_num((val >> 32) as i32);
    }

    fn get_long(&self, index: usize) -> i64 {
        let slot = self.get(index).expect("index is wrong");
        let low = slot.get_num();
        let next_slot = self.get(index+1).expect("index is wrong");
        let high = next_slot.get_num();
        return (high << 32 | low) as i64;
    }

    fn set_double(&mut self, index: usize, val: f64) {
        let long = f64_to_i64(val);
        self.set_long(index,long);
    }

    fn get_double(&self, index: usize) -> f64 {
        let long = self.get_long(index);
        return i64_to_f64(long);
    }

    fn set_ref(&mut self, index: usize, val: Rc<RefCell<Object>>) {
        let slot = self.get_mut(index).expect("index is wrong");
        slot.set_ref(val);
    }

    fn get_ref(&self, index: usize) -> Rc<RefCell<Object>> {
        let slot = self.get(index).expect("index is wrong");
        return slot.get_ref();
    }
}

pub fn f32_to_i32(val:f32) -> i32 {
    let bytes = val.to_be_bytes();
    return i32::from_be_bytes(bytes);
}

pub fn i32_to_f32(val:i32) -> f32 {
    let bytes = val.to_be_bytes();
    return f32::from_be_bytes(bytes);
}

pub fn f64_to_i64(val:f64) -> i64 {
    let bytes = val.to_be_bytes();
    return i64::from_be_bytes(bytes);
}

pub fn i64_to_f64(val:i64) -> f64 {
    let bytes = val.to_be_bytes();
    return f64::from_be_bytes(bytes);
}


#[cfg(test)]
mod tests {
    use crate::runtime_data_area::local_vars::{i32_to_f32};

    #[test]
    fn test_f32_to_i32() {
        let val = 0.66f32;
        let bytes = val.to_be_bytes();
        let val_i32 = i32::from_be_bytes(bytes);
        let new_val = i32_to_f32(val_i32);
        println!("i32 value is {}",val_i32);
        println!("f32 value is {}",new_val);
    }
}