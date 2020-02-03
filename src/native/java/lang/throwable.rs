use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::thread::Thread;
use crate::runtime_data_area::heap::class::Class;

pub fn init() {
    Registry::register("java/lang/Throwable", "fillInStackTrace",
                       "(I)Ljava/lang/Throwable;", fill_in_stack_trace);
}

pub fn fill_in_stack_trace(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    frame.operand_stack().expect("stack is none").push_ref(this.clone());
    let ptr = this.unwrap();
    let stes = StackTraceElement::create_stack_trace_elements(ptr.clone(),frame.thread());
    (*ptr).borrow_mut().set_trace(stes);
}

#[derive(Clone,Debug)]
pub struct StackTraceElement {
    file_name:String,
    class_name:String,
    method_name:String,
    line_number:i32
}

impl StackTraceElement {

    fn create_stack_trace_elements(object:Rc<RefCell<Object>>, thread:Rc<RefCell<Thread>>) -> Vec<StackTraceElement> {
        let skip = StackTraceElement::distance_to_object((*object).borrow().class()) as usize + 2;
        let thread_borrow = (*thread).borrow();
        let all_frames = thread_borrow.get_frames();
        let mut stes = Vec::with_capacity(all_frames.len() - skip);
        for i in 0..(all_frames.len() - skip) {
            stes.push(Self::create_stack_trace_element(all_frames[i].clone()));
        }
        return stes;
    }

    fn distance_to_object(class:Rc<RefCell<Class>>) -> i32 {
        let mut distance = 0;
        let mut c = (*class).borrow().super_class();
        while c.is_some() {
            distance += 1;
            c = (*c.unwrap()).borrow().super_class();
        }
        return distance;
    }

    fn create_stack_trace_element(frame:Rc<RefCell<Frame>>) -> StackTraceElement {
        let frame_borrow = (*frame).borrow();
        let method = frame_borrow.method();
        let class = method.class();
        return StackTraceElement{
            file_name: (*class).borrow().source_file(),
            class_name: (*class).borrow().java_name(),
            method_name: method.name().to_string(),
            line_number: method.get_line_number(frame_borrow.next_pc() - 1)
        };
    }
}

impl ToString for StackTraceElement {
    fn to_string(&self) -> String {
        let mut except_str = String::new();
        except_str.push_str(self.class_name.as_str());
        except_str.push('.');
        except_str.push_str(self.method_name.as_str());
        except_str.push('(');
        except_str.push_str(self.file_name.as_str());
        except_str.push(':');
        except_str.push_str(self.line_number.to_string().as_str());
        except_str.push(')');
        return except_str;
    }
}