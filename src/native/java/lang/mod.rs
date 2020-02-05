
pub mod object;
pub mod class;
pub mod system;
mod float;
mod double;
mod string;
pub mod throwable;
mod thread;

pub fn init() {
    object::init();
    class::init();
    system::init();
    float::init();
    double::init();
    string::init();
    throwable::init();
    thread::init();
}