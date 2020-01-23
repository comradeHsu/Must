use crate::runtime_data_area::slot::Slot;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::numbers::{i64_to_f64, f64_to_i64, i32_to_f32, f32_to_i32, i64_back_bytes_to_i32, i64_from_i32_bytes};
use crate::runtime_data_area::heap::object::Object;

#[derive(Clone)]
pub struct LocalVars {
    vars:Vec<Slot>
}


impl LocalVars {
    pub fn with_capacity(max_locals: usize) -> Option<LocalVars> {
        if max_locals > 0 {
            let mut vec = Vec::new();
            vec.resize_with(max_locals,|| -> Slot {Slot::new()});
            return Some(LocalVars{ vars: vec});
        }
        return None;
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        let slot = self.vars.get_mut(index).expect("index is wrong");
        slot.set_num(val);
    }

    pub fn get_int(&self, index: usize) -> i32 {
        let slot = self.vars.get(index).expect("index is wrong");
        return slot.get_num();
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        let slot = self.vars.get_mut(index).expect("index is wrong");
        slot.set_num(f32_to_i32(val));
    }

    pub fn get_float(&self, index: usize) -> f32 {
        let slot = self.vars.get(index).expect("index is wrong");
        return i32_to_f32(slot.get_num());
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        let slot = self.vars.get_mut(index).expect("index is wrong");
        slot.set_num(i64_back_bytes_to_i32(val));
        let next_slot = self.vars.get_mut(index+1).expect("index is wrong");
        next_slot.set_num((val >> 32) as i32);
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let slot = self.vars.get(index).expect("index is wrong");
        let low = i64_from_i32_bytes(slot.get_num());
        let next_slot = self.vars.get(index+1).expect("index is wrong");
        let high = next_slot.get_num() as i64;
        return high << 32 | low;
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        let long = f64_to_i64(val);
        self.set_long(index,long);
    }

    pub fn get_double(&self, index: usize) -> f64 {
        let long = self.get_long(index);
        return i64_to_f64(long);
    }

    pub fn set_ref(&mut self, index: usize, val: Option<Rc<RefCell<Object>>>) {
        let slot = self.vars.get_mut(index).expect("index is wrong");
        slot.set_ref(val);
    }

    pub fn get_ref(&self, index: usize) -> Option<Rc<RefCell<Object>>> {
        let slot = self.vars.get(index).expect("index is wrong");
        return slot.get_ref();
    }

    pub fn set_slot(&mut self, index: usize, val: Slot) {
        self.vars[index] = val;
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::numbers::i32_to_f32;

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