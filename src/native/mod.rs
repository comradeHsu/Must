pub mod java;
pub mod registry;
mod sun;

pub fn init() {
    java::init();
    sun::init();
}
