pub mod lang;
pub mod io;
pub mod security;

pub fn init() {
    lang::init();
    io::init();
    security::init();
}