use crate::runtime_data_area::slot::Slot;

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

    }

}