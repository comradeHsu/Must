use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::heap::constant_pool::Constant::MethodReference;
use crate::instructions::base::method_invoke_logic::invoke_method;

pub struct InvokeStatic(ConstantPoolInstruction);

impl InvokeStatic {
    #[inline]
    pub fn new() -> InvokeStatic {
        return InvokeStatic(ConstantPoolInstruction::new());
    }
}

impl Instruction for InvokeStatic {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = (*frame.method().class()).borrow().constant_pool();
        let mut borrow_cp = (*cp).borrow_mut();
        let constant = borrow_cp.get_constant(self.0.index());
        let method_ref = match constant {
            MethodReference(c) => c,
            _ => panic!("Unknown constant type")
        };
        let resolved_method = method_ref.resolved_method().unwrap();
        if resolved_method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        invoke_method(frame,resolved_method);
    }
}