pub mod registry;
pub mod java;
mod sun;

pub fn init() {
    java::init();
    sun::init();
}