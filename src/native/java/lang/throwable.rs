use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::thread::JavaThread;
use std::cell::RefCell;
use std::rc::Rc;
use crate::jvm::Jvm;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::utils::boxed;

pub fn init() {
    Registry::register(
        "java/lang/Throwable",
        "fillInStackTrace",
        "(I)Ljava/lang/Throwable;",
        fill_in_stack_trace,
    );
    Registry::register(
        "java/lang/Throwable",
        "getStackTraceDepth",
        "()I",
        get_stack_trace_depth,
    );
    Registry::register(
        "java/lang/Throwable",
        "getStackTraceElement",
        "(I)Ljava/lang/StackTraceElement;",
        get_stack_trace_element,
    );
}

pub fn fill_in_stack_trace(frame: &mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(this.clone());
    let ptr = this.unwrap();
    let stes = StackTraceElement::create_stack_trace_elements(ptr.clone(), frame.thread());
    (*ptr).borrow_mut().set_trace(stes);
}

///  native int getStackTraceDepth();
/// getStackTraceDepth()I
pub fn get_stack_trace_depth(frame: &mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    let ptr = this.unwrap();
    let depth = (*ptr).borrow().trace().unwrap().len();
    frame
        .operand_stack()
        .expect("stack is none")
        .push_int(depth as i32);
}

///  native StackTraceElement getStackTraceElement(int index);
/// (I)Ljava/lang/StackTraceElement;
pub fn get_stack_trace_element(frame: &mut Frame) {
    let this = frame.local_vars().expect("vars is none").get_this();
    let index = frame.local_vars().expect("vars is none").get_int(1) as usize;
    let ptr = this.unwrap();
    let this_ref = (*ptr).borrow();
    let elements = this_ref.trace().unwrap();
    let java_element = create_java_stack_trace_element(elements.get(index).unwrap());
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(java_element);
}

fn create_java_stack_trace_element(element:&StackTraceElement) -> Option<Rc<RefCell<Object>>> {
    let loader = Jvm::boot_class_loader();
    let class = loader.find_or_create("java/lang/StackTraceElement").unwrap();
    let mut object = Class::new_object(&class);
    object.set_ref_var(
        "declaringClass",
        "Ljava/lang/String;",
        StringPool::java_string(element.class_name.clone())
    );
    object.set_ref_var(
        "fileName",
        "Ljava/lang/String;",
        StringPool::java_string(element.file_name.clone())
    );
    object.set_ref_var(
        "methodName",
        "Ljava/lang/String;",
        StringPool::java_string(element.method_name.clone())
    );
    object.set_int_var(
        "lineNumber",
        "I",
        element.line_number
    );
    Some(boxed(object))
}

#[derive(Clone, Debug)]
pub struct StackTraceElement {
    file_name: String,
    class_name: String,
    method_name: String,
    line_number: i32,
}

impl StackTraceElement {
    fn create_stack_trace_elements(
        object: Rc<RefCell<Object>>,
        thread: Rc<RefCell<JavaThread>>,
    ) -> Vec<StackTraceElement> {
        let skip = StackTraceElement::distance_to_object((*object).borrow().class()) as usize + 2;
        let thread_borrow = (*thread).borrow();
        let all_frames = thread_borrow.get_frames();
        let mut stes = Vec::with_capacity(all_frames.len() - skip);
        for i in 0..(all_frames.len() - skip) {
            stes.push(Self::create_stack_trace_element(all_frames[i].clone()));
        }
        return stes;
    }

    fn distance_to_object(class: Rc<RefCell<Class>>) -> i32 {
        let mut distance = 0;
        let mut c = (*class).borrow().super_class();
        while c.is_some() {
            distance += 1;
            c = (*c.unwrap()).borrow().super_class();
        }
        return distance;
    }

    fn create_stack_trace_element(frame: Rc<RefCell<Frame>>) -> StackTraceElement {
        let frame_borrow = (*frame).borrow();
        let method = frame_borrow.method();
        let class = method.class();
        return StackTraceElement {
            file_name: (*class).borrow().source_file(),
            class_name: (*class).borrow().java_name(),
            method_name: method.name().to_string(),
            line_number: method.get_line_number(frame_borrow.next_pc() - 1),
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
