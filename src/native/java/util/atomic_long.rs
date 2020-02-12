use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;

pub fn init() {
    Registry::register(
        "java/util/concurrent/atomic/AtomicLong",
        "VMSupportsCS8",
        "()Z",
        vm_supports_cs8,
    )
}

/// java/util/concurrent/atomic/AtomicLong.VMSupportsCS8()Z
pub fn vm_supports_cs8(frame: &mut Frame) {
    frame
        .operand_stack()
        .expect("stack is none")
        .push_boolean(false);
}
