use std::rc::Rc;
use std::cell::RefCell;

pub mod numbers;

pub fn boxed<T>(data:T) -> Rc<RefCell<T>> {
    return Rc::new(RefCell::new(data));
}