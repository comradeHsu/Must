use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;

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
pub fn available_processors(frame: &mut Frame) {
    let num_cpu = num_cpus::get();
    frame
        .operand_stack()
        .expect("stack is none")
        .push_int(num_cpu as i32);
}

/// public native long freeMemory();
/// ()J
pub fn free_memory(frame: &mut Frame) {
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(100_000);
}

/// public native long totalMemory();
/// ()J
pub fn total_memory(frame: &mut Frame) {
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(100_0000);
}

/// public native long maxMemory();
/// ()J
pub fn max_memory(frame: &mut Frame) {
    frame
        .operand_stack()
        .expect("stack is none")
        .push_long(100_0000);
}

/// public native void gc();
/// ()V
pub fn gc(frame: &mut Frame) {}

#[cfg(test)]
mod test {

    #[test]
    fn test_num_cpu() {
        println!("num cpu:{}", num_cpus::get());
    }
}
