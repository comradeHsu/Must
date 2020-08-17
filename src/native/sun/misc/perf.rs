use crate::native::registry::Registry;
use crate::prims::perf::Perf;
use crate::runtime::frame::Frame;

pub fn init() {
    Registry::register(
        "sun/misc/Perf",
        "createLong",
        "(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
        create_long,
    );
}
/// public native ByteBuffer createLong(String var1, int var2, int var3, long var4);
/// (Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;
pub fn create_long(frame: &Frame) {
    let (perf, name, variability, units, value) = frame.local_vars_get(|vars| {
        let perf = vars.get_this();
        let name = vars.get_ref(1);
        let variability = vars.get_int(2);
        let units = vars.get_int(3);
        let value = vars.get_long(4);
        (perf, name, variability, units, value)
    });

    let byte_buffer = Perf::create_long(perf, name, variability, units, value);
    frame.push_ref(byte_buffer);
}
