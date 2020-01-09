use crate::runtime_data_area::slot::Slot;
use crate::utils::numbers::{f32_to_i32, i32_to_f32, f64_to_i64, i64_to_f64, i64_back_bytes_to_i32, i64_from_i32_bytes};
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::object::Object;

pub struct OperandStack {
    size:usize,
    slots:Vec<Slot>
}

impl OperandStack {
    #[inline]
    pub fn new(max_stack:usize) -> Option<OperandStack> {
        if max_stack > 0 {
            return Some(OperandStack{
                size: 0,
                slots: vec![]
            });
        }
        return None;
    }

    pub fn push_int(&mut self,val:i32) {
        let slot = Slot::with_num(val);
        self.slots.push(slot);
        self.size += 1;
    }

    pub fn pop_int(&mut self) -> i32{
        let slot = self.slots.pop().unwrap();
        self.size -= 1;
        return slot.get_num();
    }

    pub fn push_float(&mut self,val:f32) {
        let slot = Slot::with_num(f32_to_i32(val));
        self.slots.push(slot);
        self.size += 1;
    }

    pub fn pop_float(&mut self) -> f32{
        let slot = self.slots.pop().unwrap();
        self.size -= 1;
        return i32_to_f32(slot.get_num());
    }

    pub fn push_long(&mut self,val:i64) {
        let low = i64_back_bytes_to_i32(val);
        let high = (val >> 32) as i32;
        let low_slot = Slot::with_num(low);
        let high_slot = Slot::with_num(high);
        self.slots.push(low_slot);
        self.slots.push(high_slot);
        self.size += 2;
    }

    pub fn pop_long(&mut self) -> i64{
        let high = self.slots.pop().unwrap().get_num() as i64;
        let low = i64_from_i32_bytes(self.slots.pop().unwrap().get_num());
        self.size -= 2;
        return high << 32 | low;
    }

    pub fn push_double(&mut self,val:f64) {
        self.push_long(f64_to_i64(val));
    }

    pub fn pop_double(&mut self) -> f64{
        return i64_to_f64(self.pop_long());
    }

    pub fn push_ref(&mut self,val:Option<Rc<RefCell<Object>>>) {
        let slot = Slot::with_ref(val);
        self.slots.push(slot);
        self.size += 1;
    }

    pub fn pop_ref(&mut self) -> Option<Rc<RefCell<Object>>>{
        let slot = self.slots.pop().unwrap();
        self.size -= 1;
        return slot.get_ref();
    }

    pub fn push_slot(&mut self,val:Slot) {
        self.slots.push(val);
        self.size += 1;
    }

    pub fn pop_slot(&mut self) -> Slot{
        let slot = self.slots.pop().unwrap();
        self.size -= 1;
        return slot;
    }
}