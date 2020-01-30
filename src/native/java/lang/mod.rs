use crate::native::registry::Registry;
use crate::native::java::lang::class::{get_primitive_class, get_name0, desired_assertion_status0};

pub mod object;
pub mod class;

pub fn init() {
    object::init();
    class::init();
}