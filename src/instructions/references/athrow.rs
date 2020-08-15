use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::runtime::frame::Frame;
use crate::oops::object::Object;
use crate::runtime::thread::JavaThread;
use crate::utils::java_str_to_rust_str;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct AThrow(NoOperandsInstruction);

impl AThrow {
    #[inline]
    pub fn new() -> AThrow {
        return AThrow(NoOperandsInstruction::new());
    }

    fn find_and_goto_exception_handler(frame: &Frame, object: Rc<RefCell<Object>>) -> bool {
        ///
        fn get_handler_pc(frame: &Frame, object: Rc<RefCell<Object>>) -> i32 {
            let pc = frame.next_pc() - 1;
            return frame
                .method()
                .find_exception_handler((*object).borrow().class(), pc);
        }

        let thread = JavaThread::current();

        //display_frame(frame);

        let pc = frame
            .method()
            .find_exception_handler((*object).borrow().class(), frame.next_pc() - 1);
        if pc > 0 {
            frame.operand_stack(|stack| {
                stack.clear();
                stack.push_ref(Some(object.clone()));
            });
            frame.set_next_pc(pc);
            return true;
        }
        thread.pop_frame();
        loop {
            if thread.is_stack_empty() {
                break;
            }
            let frame = thread.current_frame();
            /**
             **
            {
                let fra = (*frame).borrow();
                display_frame(fra.deref());
            }
            **/
            let handler_pc = get_handler_pc(&frame, object.clone());
            if handler_pc > 0 {
                frame.operand_stack(|stack| {
                    stack.clear();
                    stack.push_ref(Some(object.clone()));
                });
                frame.set_next_pc(handler_pc);
                return true;
            }
            thread.pop_frame();
        }
        return false;
    }

    fn handle_uncaught_exception(object: Rc<RefCell<Object>>) {
        let thread = JavaThread::current();
        thread.clear_stack();
        let _java_msg = (*object)
            .borrow()
            .get_ref_var("detailMessage", "Ljava/lang/String;");
        //        let rust_msg = java_str_to_rust_str(java_msg.unwrap());
        let bor_obj = (*object).borrow();
        let stes = bor_obj.trace().expect("The exception object hasn't trace");
        let ex_class = bor_obj.class();

        let detail_message = bor_obj
            .get_ref_var("detailMessage", "Ljava/lang/String;")
            .map_or("".to_string(), |v| java_str_to_rust_str(v));

        println!("\t{},{}", (*ex_class).borrow().java_name(), detail_message);
        for ste in stes {
            println!("\tat {}", ste.to_string());
        }
    }
}

impl Instruction for AThrow {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.0.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &Frame) {
        let ex = frame.pop_ref();
        if ex.is_none() {
            panic!("java.lang.NullPointerException");
        }
        let object = ex.unwrap();

        //        let meta = (*object).borrow().meta();
        //        println!("ex class : {}",(*meta.unwrap()).borrow().java_name());
        {
            let method = frame.method_ptr();
            let class = (*object).borrow().class();
        }
        if !Self::find_and_goto_exception_handler(frame, object.clone()) {
            Self::handle_uncaught_exception(object);
        }
    }
}

fn display_frame(frame: &Frame) {
    let method = frame.method_ptr();
    if method.name() == "loadClass"
        && method.descriptor() == "(Ljava/lang/String;Z)Ljava/lang/Class;"
    {
        let this = frame.get_this().unwrap();
        let class = (*this).borrow().class();
        let name = (*class).borrow().java_name();
        println!("java class:{}", name);
    }
}
