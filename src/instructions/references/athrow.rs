use crate::instructions::base::instruction::{NoOperandsInstruction, Instruction};
use crate::runtime_data_area::frame::Frame;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::thread::Thread;
use crate::runtime_data_area::heap::object::Object;
use crate::utils::java_str_to_rust_str;

pub struct AThrow(NoOperandsInstruction);

impl AThrow {
    #[inline]
    pub fn new() -> AThrow {
        return AThrow(NoOperandsInstruction::new());
    }

    fn find_and_goto_exception_handler(frame: &mut Frame, object:Rc<RefCell<Object>>) -> bool {
        ///
        fn get_handler_pc(frame:Rc<RefCell<Frame>>,object:Rc<RefCell<Object>>) -> i32 {
            let pc = (*frame).borrow().next_pc() - 1;
            let borrow_frame = (*frame).borrow();
            return borrow_frame.method().find_exception_handler((*object).borrow().class(),pc);
        }

        let thread = frame.thread();
        let pc = frame.method().find_exception_handler((*object).borrow().class(),frame.next_pc()-1);
        if pc > 0 {
            let stack = frame.operand_stack().expect("stack is none");
            stack.clear();
            stack.push_ref(Some(object.clone()));
            frame.set_next_pc(pc);
            return true;
        }
        (*thread).borrow_mut().pop_frame();
        loop {
            let frame = (*thread).borrow().current_frame();
            let handler_pc = get_handler_pc(frame.clone(),object.clone());
            if handler_pc > 0 {
                let mut mut_borrow = (*frame).borrow_mut();
                let stack = mut_borrow.operand_stack().expect("stack is none");
                stack.clear();
                stack.push_ref(Some(object.clone()));
                mut_borrow.set_next_pc(handler_pc);
                return true;
            }
            (*thread).borrow_mut().pop_frame();
            if (*thread).borrow().is_stack_empty() {
                break;
            }
        }
        return false;
    }

    fn handle_uncaught_exception(thread:Rc<RefCell<Thread>>, object:Rc<RefCell<Object>>) {
        (*thread).borrow_mut().clear_stack();
        let java_msg = (*object).borrow().get_ref_var("detailMessage", "Ljava/lang/String;");
        let rust_msg = java_str_to_rust_str(java_msg.unwrap());
        let bor_obj = (*object).borrow();
        let stes = bor_obj.trace().expect("The exception object hasn't trace");
        for ste in stes {
            println!("\tat {}" ,ste.to_string());
        }
    }
}

impl Instruction for AThrow {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let ex = frame.operand_stack().expect("stack is none").pop_ref();
        if ex.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let thread = frame.thread();
        let object = ex.unwrap();
        if !Self::find_and_goto_exception_handler(frame, object.clone()) {
            Self::handle_uncaught_exception(thread,object);
        }
    }
}