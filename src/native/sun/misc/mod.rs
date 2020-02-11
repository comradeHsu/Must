mod vm;
mod Unsafe;

pub fn init() {
    vm::init();
    Unsafe::init();
}