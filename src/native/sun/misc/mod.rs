mod Unsafe;
mod vm;

pub fn init() {
    vm::init();
    Unsafe::init();
}
