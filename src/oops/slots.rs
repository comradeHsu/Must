use crate::oops::object::Object;
use crate::runtime::slot::Slot;
use crate::utils::numbers::{
    f32_to_i32, f64_to_i64, i32_to_f32, i64_back_bytes_to_i32, i64_from_bytes, i64_to_f64,
};

#[derive(Clone)]
pub struct Slots {
    slots: Vec<Slot>,
}

impl Slots {
    pub fn with_capacity(capacity: usize) -> Slots {
        let mut vec = Vec::new();
        vec.resize_with(capacity, || return Slot::new());
        return Slots { slots: vec };
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.slots[index].num = val;
    }

    pub fn get_int(&self, index: usize) -> i32 {
        return self.slots[index].num;
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        self.slots[index].num = f32_to_i32(val);
    }

    pub fn get_float(&self, index: usize) -> f32 {
        return i32_to_f32(self.slots[index].num);
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        self.slots[index].num = i64_back_bytes_to_i32(val);
        self.slots[index + 1].num = (val >> 32) as i32;
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let low = self.slots[index].num as i32;
        let high = self.slots[index + 1].num as i64;
        return (high << 32) | i64_from_bytes(low);
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        self.set_long(index, f64_to_i64(val));
    }

    pub fn get_double(&self, index: usize) -> f64 {
        return i64_to_f64(self.get_long(index));
    }

    pub fn set_ref(&mut self, index: usize, val: Option<Object>) {
        self.slots[index].reference = val;
    }

    pub fn get_ref(&self, index: usize) -> Option<Object> {
        return self.slots[index].reference.clone();
    }
}

#[cfg(test)]
mod test {
    use crate::oops::slots::Slots;

    #[test]
    fn split_long() {
        let mut slots = Slots::with_capacity(10);
        slots.set_long(0, 2507424867904i64);
        assert_eq!(2507424867904i64, slots.get_long(0), "error")
    }
}
