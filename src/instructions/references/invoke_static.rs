use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::MethodReference;

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
        let pool_class = (*cp).borrow().class();
        let mut borrow_cp = (*cp).borrow_mut();
        let constant = borrow_cp.get_constant(self.0.index());
        let method_ref = match constant {
            MethodReference(c) => c,
            _ => panic!("Unknown constant type"),
        };
        let resolved_method = method_ref.resolved_method(pool_class).unwrap();
        if !resolved_method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let class = resolved_method.class();
        if !(*class).borrow().initialized() {
            frame.revert_next_pc();
            init_class(frame.thread(), class.clone());
            return;
        }
        invoke_method(frame, resolved_method);
    }
}
