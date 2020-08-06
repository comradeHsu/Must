use crate::oops::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub type JObject = Option<Rc<RefCell<Object>>>;

pub type JString = Option<Rc<RefCell<Object>>>;
