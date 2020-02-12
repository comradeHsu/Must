use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::utils::numbers::f32_to_i32;

pub fn init() {
    Registry::register(
        "java/lang/Float",
        "floatToRawIntBits",
        "(F)I",
        float_to_raw_int_bits,
    );
}

pub fn float_to_raw_int_bits(frame: &mut Frame) {
    let value = frame.local_vars().expect("vars is none").get_float(0);
    frame
        .operand_stack()
        .expect("stack is none")
        .push_int(f32_to_i32(value));
}
