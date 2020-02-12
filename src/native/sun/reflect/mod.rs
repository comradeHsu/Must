mod native_constructor_accessor_impl;
mod reflection;

pub fn init() {
    reflection::init();
    native_constructor_accessor_impl::init();
}
