mod vm;
mod r#unsafe;

pub fn init() {
    vm::init();
    r#unsafe::init();
}