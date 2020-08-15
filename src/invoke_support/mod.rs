use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::new_instruction;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::return_value::ReturnValue;
use crate::jvm::JVM;
use crate::runtime::frame::Frame;
use crate::oops::method::Method;
use crate::oops::string_pool::StringPool;
use crate::runtime::thread::JavaThread;
use crate::utils::boxed;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::process::exit;
use std::rc::Rc;
use std::thread::sleep_ms;
use crate::class_loader::app_class_loader::ClassLoader;
use crate::oops::class::Class;
use crate::instructions::references::athrow::AThrow;
use crate::instructions::base::instruction::Instruction;

pub mod parameter;
pub mod return_value;

pub struct JavaCall {
    current_pc: i32,
    thread: JavaThread,
    method: Rc<Method>,
    params: Option<Parameters>,
    return_type: ReturnType
}

impl JavaCall {

    fn new(method: Rc<Method>,
           params: Option<Parameters>,
           return_type: ReturnType,
    ) -> JavaCall {
        let thread = JavaThread::current();
        return JavaCall{
            current_pc: thread.get_pc(),
            thread ,
            method,
            params,
            return_type
        };
    }

    pub fn invoke(
        method: Rc<Method>,
        params: Option<Parameters>,
        return_type: ReturnType,
    ) -> ReturnValue {
        let call = Self::new(method,params,return_type);
        let thread = call.create_execute_env();
        let return_value = call.executable();
        return return_value;
    }

    fn create_execute_env(&self) {
        let mut dummy_frame = Frame::new_barrier_frame();
        let mut frame =  Frame::new_intrinsic_frame(self.method.clone());
        self.prepare_parameter(&mut frame);
        self.thread.push_frame(dummy_frame);
        self.thread.push_frame(frame);
    }

    fn prepare_parameter(&self,frame: &mut Frame) {
        if self.params.is_some() {
            frame.local_vars_set(|vars| {
                let params = self.params.as_ref().unwrap();
                let mut index = 0;
                for i in 0..params.size() {
                    let parameter = params.get_parameter(i);
                    match parameter {
                        Parameter::Boolean(value) => vars.set_boolean(index, *value),
                        Parameter::Byte(value) => vars.set_int(index, *value as i32),
                        Parameter::Short(value) => vars.set_int(index, *value as i32),
                        Parameter::Int(value) => vars.set_int(index, *value),
                        Parameter::Long(value) => {
                            vars.set_long(index, *value);
                            index += 1;
                        }
                        Parameter::Float(value) => vars.set_float(index, *value),
                        Parameter::Double(value) => {
                            vars.set_double(index, *value);
                            index += 1;
                        }
                        Parameter::Char(value) => vars.set_int(index, *value as u8 as i32),
                        Parameter::Object(value) => vars.set_ref(index, value.clone()),
                    }
                    index += 1;
                }
            });
        }
    }

    fn executable(&self) -> ReturnValue {
        let mut reader = BytecodeReader::new();
        loop {
            let current_frame = self.thread.current_frame();
            if current_frame.is_barrier_frame() {
                break;
            }
            let pc = current_frame.next_pc();
            self.thread.set_pc(pc);
            let method = current_frame.method_ptr();
            let bytecode = method.code();
            reader.reset(bytecode, pc);
            let opcode = reader.read_u8();
            //println!("\tmethod:{}, {}, {},inst:{}",method.name(),method.descriptor(),(*method.class()).borrow().name(),opcode);
            let mut inst = new_instruction(opcode);
            inst.fetch_operands(&mut reader);
            current_frame.set_next_pc(reader.pc());
            inst.execute(&current_frame);
            if self.thread.is_stack_empty() {
                exit(101);
            }
            //sleep_ms(500);
        }
        let value_frame = self.thread.pop_frame();
        let value = match self.return_type {
            ReturnType::Void => ReturnValue::Void,
            ReturnType::Boolean => ReturnValue::Boolean(value_frame.pop_boolean()),
            ReturnType::Byte => ReturnValue::Byte(value_frame.pop_int() as i8),
            ReturnType::Short => ReturnValue::Short(value_frame.pop_int() as i16),
            ReturnType::Int => ReturnValue::Int(value_frame.pop_int()),
            ReturnType::Long => ReturnValue::Long(value_frame.pop_long()),
            ReturnType::Float => ReturnValue::Float(value_frame.pop_float()),
            ReturnType::Double => ReturnValue::Double(value_frame.pop_double()),
            ReturnType::Char => ReturnValue::Char(value_frame.pop_int() as u8 as char),
            ReturnType::Object => ReturnValue::Object(value_frame.pop_ref()),
        };
        return value;
    }

}

impl Drop for JavaCall {
    fn drop(&mut self) {
        self.thread.set_pc(self.current_pc)
    }
}

pub enum ReturnType {
    Void,
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
    Object,
}

pub fn throw_exception(frame: &Frame, class_name: &str, msg: Option<&str>) {
    let class = frame.method().class();
    let class_loader = (*class).borrow().get_class_loader();
    let exception_class = ClassLoader::load_class(class_loader,class_name);
    let mut object = Class::new_object(&exception_class);
    let constructor_desc = "(Ljava/lang/String;)V";
    let detail_message = match msg.is_some() {
        true => Some(StringPool::java_string(msg.unwrap().to_string())),
        false => None
    };
    let constructor = Class::get_constructor(exception_class.clone(), constructor_desc);
    let object_ptr = Some(boxed(object));
    let parameters = vec![
        Parameter::Object(object_ptr.clone()),
        Parameter::Object(detail_message)
    ];
    JavaCall::invoke(constructor.unwrap(),Some(Parameters::with_parameters(parameters)),ReturnType::Void);
    frame.push_ref(object_ptr);
    let mut athrow = AThrow::new();
    athrow.execute(frame);
}

fn to_hex_str(seq:&Vec<u8>) -> String {
    let mut string = String::new();
    string.push_str("[");
    for s in seq {
        string.push_str(format!("{:X}", *s).as_str());
        string.push_str(", ")
    }
    string.push_str("]");
    string
}