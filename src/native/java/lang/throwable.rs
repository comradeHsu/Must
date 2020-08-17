use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::oops::class::Class;
use crate::oops::object::Object;
use crate::runtime::thread::JavaThread;
use std::cell::RefCell;
use std::rc::Rc;
use crate::jvm::Jvm;
use crate::oops::string_pool::StringPool;
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

pub fn fill_in_stack_trace(frame: &Frame) {
    let this = frame.get_this();
    frame.push_ref(this.clone());
    let ptr = this.unwrap();
    let stes = StackTraceElement::create_stack_trace_elements(&ptr);
    ptr.set_trace(stes);
}

///  native int getStackTraceDepth();
/// getStackTraceDepth()I
pub fn get_stack_trace_depth(frame: &Frame) {
    let this = frame.get_this();
    let ptr = this.unwrap();
    let depth = ptr.trace(|elements| elements.len());
    frame.push_int(depth as i32);
}

///  native StackTraceElement getStackTraceElement(int index);
/// (I)Ljava/lang/StackTraceElement;
pub fn get_stack_trace_element(frame: &Frame) {
    let this = frame.get_this();
    let index = frame.get_int(1) as usize;
    let ptr = this.unwrap();
    let java_element = ptr.trace(|elements| {
        create_java_stack_trace_element(elements.get(index).unwrap())
    });
    frame.push_ref(java_element);
}

fn create_java_stack_trace_element(element:&StackTraceElement) -> Option<Object> {
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
    Some(object)
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
        object: &Object
    ) -> Vec<StackTraceElement> {
        let skip = StackTraceElement::distance_to_object(object.class()) as usize + 2;
        let thread = JavaThread::current();
        thread.frames_with(|frames|{
            let mut stes = Vec::with_capacity(frames.len() - skip);
            for i in 0..(frames.len() - skip) {
                stes.push(Self::create_stack_trace_element(&frames[i]));
            }
            return stes;
        })
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

    fn create_stack_trace_element(frame: &Frame) -> StackTraceElement {
        let method = frame.method();
        let class = method.class();
        return StackTraceElement {
            file_name: (*class).borrow().source_file(),
            class_name: (*class).borrow().java_name(),
            method_name: method.name().to_string(),
            line_number: method.get_line_number(frame.next_pc() - 1),
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
