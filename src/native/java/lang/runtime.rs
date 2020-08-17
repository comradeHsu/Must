use crate::native::registry::Registry;
use crate::runtime::frame::Frame;

pub fn init() {
    Registry::register(
        "java/lang/Runtime",
        "availableProcessors",
        "()I",
        available_processors,
    );
    Registry::register("java/lang/Runtime", "freeMemory", "()J", free_memory);
    Registry::register("java/lang/Runtime", "totalMemory", "()J", total_memory);
    Registry::register("java/lang/Runtime", "maxMemory", "()J", max_memory);
    Registry::register("java/lang/Runtime", "gc", "()V", gc);
}

/// public native int availableProcessors();
/// ()I
pub fn available_processors(frame: &Frame) {
    let num_cpu = num_cpus::get();
    frame.push_int(num_cpu as i32);
}

/// public native long freeMemory();
/// ()J
pub fn free_memory(frame: &Frame) {
    frame.push_long(100_000);
}

/// public native long totalMemory();
/// ()J
pub fn total_memory(frame: &Frame) {
    frame.push_long(100_0000);
}

/// public native long maxMemory();
/// ()J
pub fn max_memory(frame: &Frame) {
    frame.push_long(100_0000);
}

/// public native void gc();
/// ()V
pub fn gc(_frame: &Frame) {}

#[cfg(test)]
mod test {

    #[test]
    fn test_num_cpu() {
        println!("num cpu:{}", num_cpus::get());
    }
}
