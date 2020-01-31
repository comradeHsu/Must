use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::native::registry::Registry;

pub struct InvokeNative(NoOperandsInstruction);

impl InvokeNative {
    #[inline]
    pub const fn new() -> InvokeNative {
        return InvokeNative(NoOperandsInstruction::new());
    }
}

impl Instruction for InvokeNative {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let method = frame.method();
        let class = method.class();
        let borrow_class = (*class).borrow();
        let class_name = borrow_class.name();
        let method_name = method.name();
        let method_desc = method.descriptor();
        let native_method = Registry::find_native_method(class_name,method_name,method_desc);
        if native_method.is_none() {
            let method_info = class_name.to_string() + "." + method_name + method_desc;
            panic!("java.lang.UnsatisfiedLinkError: {}",method_info);
        }
        native_method.unwrap()(frame);
    }
}