use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::{Instruction, NoOperandsInstruction};
use crate::oops::object::Object;
use crate::runtime::frame::Frame;
use crate::runtime::thread::JavaThread;
use crate::utils::java_str_to_rust_str;




pub struct AThrow(NoOperandsInstruction);

impl AThrow {
    #[inline]
    pub fn new() -> AThrow {
        return AThrow(NoOperandsInstruction::new());
    }

    fn find_and_goto_exception_handler(frame: &Frame, object: Object) -> bool {
        ///
        fn get_handler_pc(frame: &Frame, object: Object) -> i32 {
            let pc = frame.next_pc() - 1;
            return frame.method().find_exception_handler(object.class(), pc);
        }

        let thread = JavaThread::current();
        loop {
            if thread.is_stack_empty() {
                break;
            }
            let frame = thread.current_frame();
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

    fn handle_uncaught_exception(object: Object) {
        let thread = JavaThread::current();
        thread.clear_stack();
        let _java_msg = object.get_ref_var("detailMessage", "Ljava/lang/String;");
        //        let rust_msg = java_str_to_rust_str(java_msg.unwrap());
        let ex_class = object.class();
        let detail_message = object
            .get_ref_var("detailMessage", "Ljava/lang/String;")
            .map_or("".to_string(), |v| java_str_to_rust_str(v));
        println!("\t{},{}", (*ex_class).borrow().java_name(), detail_message);
        object.trace(|elements| {
            let len = elements.len() - 1;
            for index in 0..=len {
                println!("\tat {}", elements[len - index].to_string());
            }
        })
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
            let _method = frame.method();
            let _class = object.class();
        }
        if !Self::find_and_goto_exception_handler(frame, object.clone()) {
            Self::handle_uncaught_exception(object);
        }
    }
}

fn display_frame(frame: &Frame) {
    let method = frame.method();
    if method.name() == "loadClass"
        && method.descriptor() == "(Ljava/lang/String;Z)Ljava/lang/Class;"
    {
        let this = frame.get_this().unwrap();
        let class = this.class();
        let name = (*class).borrow().java_name();
        println!("java class:{}", name);
    }
}
