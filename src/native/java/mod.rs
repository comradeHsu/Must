pub mod lang;
pub mod io;
pub fn init() {
    lang::init();
    io::init();
}