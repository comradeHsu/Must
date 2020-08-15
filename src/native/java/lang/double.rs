use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::utils::numbers::{f64_to_i64, i64_to_f64};

pub fn init() {
    Registry::register(
        "java/lang/Double",
        "doubleToRawLongBits",
        "(D)J",
        double_to_raw_long_bits,
    );
    Registry::register(
        "java/lang/Double",
        "longBitsToDouble",
        "(J)D",
        long_bits_to_double,
    );
}

pub fn double_to_raw_long_bits(frame: &Frame) {
    let value = frame.get_double(0);
    frame.push_long(f64_to_i64(value));
}

pub fn long_bits_to_double(frame: &Frame) {
    let value = frame.get_long(0);
    frame.push_double(i64_to_f64(value));
}
