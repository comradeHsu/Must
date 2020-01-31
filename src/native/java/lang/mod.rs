
pub mod object;
pub mod class;
pub mod system;

pub fn init() {
    object::init();
    class::init();
    system::init();
}