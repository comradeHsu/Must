use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;
use crate::instructions::references::ResolveClassRef;

pub struct CheckCast(ConstantPoolInstruction);

impl CheckCast {
    #[inline]
    pub fn new() -> CheckCast {
        return CheckCast(ConstantPoolInstruction::new());
    }
}

impl Instruction for CheckCast {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack().expect("stack is none");
        let reference = stack.pop_ref();
        stack.push_ref(reference.clone());
        if reference.is_none() {
            return;
        }
        let c = frame.method().class();

        let class = self.resolve_class_ref(c);
        if !(*reference.unwrap()).borrow().is_instance_of(class) {
            panic!("java.lang.ClassCastException");
        }
    }
}

impl ResolveClassRef for CheckCast {
    fn get_index(&self) -> usize {
        return self.0.index();
    }
}
