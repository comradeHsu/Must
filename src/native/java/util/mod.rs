mod atomic_long;
mod zip_file;

pub fn init() {
    atomic_long::init();
    zip_file::init();
}
