use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::utils::numbers::f32_to_i32;

pub fn init() {
    Registry::register(
        "java/lang/Float",
        "floatToRawIntBits",
        "(F)I",
        float_to_raw_int_bits,
    );
}

pub fn float_to_raw_int_bits(frame: &Frame) {
    let value = frame.get_float(0);
    frame.push_int(f32_to_i32(value));
}
