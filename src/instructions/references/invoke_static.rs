use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::base::method_invoke_logic::invoke_method;
use crate::runtime::frame::Frame;
use crate::oops::constant_pool::Constant::MethodReference;
use crate::instructions::references::ResolveMethodRef;

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
        let class = frame.method().class();

        let resolved_method = self.resolved_method_ref(class);
        if !resolved_method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        let class = resolved_method.class();
        if !(*class).borrow().initialized() {
            frame.revert_next_pc();
            init_class(class.clone());
            return;
        }
        invoke_method(frame, resolved_method);
    }
}

impl ResolveMethodRef for InvokeStatic {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
