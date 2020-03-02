mod atomic_long;
mod jar_file;
mod zip_file;

pub fn init() {
    atomic_long::init();
    zip_file::init();
    jar_file::init();
}
