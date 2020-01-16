use crate::instructions::base::instruction::{ConstantPoolInstruction, Instruction};
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::constant_pool::Constant::MethodReference;

pub struct InvokeVirtual(ConstantPoolInstruction);

impl InvokeVirtual {
    #[inline]
    pub const fn new() -> InvokeVirtual {
        return InvokeVirtual(ConstantPoolInstruction::new());
    }
}

impl Instruction for InvokeVirtual {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = (*frame.method().class()).borrow().constant_pool();
        let constant = cp.get_constant(self.0.index());
        let method_ref = match constant {
            MethodReference(c) => c,
            _ => {}
        };
        if method_ref.name() == "println" {
            let stack = frame.operand_stack().expect("stack is none");
            match method_ref.descriptor() {
                "(Z)V" => println!("{}",stack.pop_int() != 0),
                "(C)V" => println!("{}",stack.pop_int() as char),
                "(I)V" | "(B)V" | "(S)V" => println!("{}",stack.pop_int()),
                "(F)V" => println!("{}",stack.pop_float()),
                "(J)V" => println!("{}",stack.pop_long()),
                "(D)V" => println!("{}",stack.pop_double()),
                _ => panic!("println: " + method_ref.descriptor())
            }
            stack.pop_ref();
        }
    }
}