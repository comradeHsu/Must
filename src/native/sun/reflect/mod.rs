mod reflection;
mod native_constructor_accessor_impl;

pub fn init() {
    reflection::init();
    native_constructor_accessor_impl::init();
}