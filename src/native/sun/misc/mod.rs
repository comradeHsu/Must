mod misc_unsafe;
mod perf;
mod url_class_path;
mod vm;

pub fn init() {
    vm::init();
    misc_unsafe::init();
    url_class_path::init();
    perf::init();
}
