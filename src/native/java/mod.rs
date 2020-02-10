pub mod lang;
pub mod io;
pub mod security;
pub mod util;

pub fn init() {
    lang::init();
    io::init();
    security::init();
    util::init();
}