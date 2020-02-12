pub mod class;
mod class_loader;
mod double;
mod float;
pub mod object;
mod runtime;
mod string;
pub mod system;
mod thread;
pub mod throwable;

pub fn init() {
    object::init();
    class::init();
    system::init();
    float::init();
    double::init();
    string::init();
    throwable::init();
    thread::init();
    class_loader::init();
    runtime::init();
}
