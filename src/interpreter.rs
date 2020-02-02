use crate::class_file::member_info::MemberInfo;
use crate::runtime_data_area::thread::Thread;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::new_instruction;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::frame::Frame;
use std::ops::DerefMut;
use std::time::SystemTime;
use chrono::Local;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::utils::boxed;
use crate::native::init;

pub fn interpret(method:Rc<Method>,args:&Vec<String>) {

    let thread = Rc::new(RefCell::new(Thread::new_thread()));
    let mut frame = Thread::new_frame(thread.clone(),method.clone());
    let java_args = create_args_array((*method.class()).borrow().loader(),args);
    frame.local_vars().expect("vars is none").set_ref(0,Some(java_args));
    (*thread).borrow_mut().push_frame(frame);
    circulate(thread);
}

pub fn circulate(mut thread:Rc<RefCell<Thread>>) {
    let mut reader = BytecodeReader::new();
    init();
    println!("start {:?}",Local::now());
    loop {
//        let mut borrow_thread = (*thread).borrow_mut();
        let current_frame = (*thread).borrow().current_frame();
        let pc = (*current_frame).borrow().next_pc();
        (*thread).borrow_mut().set_pc(pc);
        let method = (*current_frame).borrow().method_ptr();
        let bytecode = method.code();
        let mut codes = Vec::with_capacity(bytecode.len());
        for c in bytecode {
            codes.push(*c);
        }
//        println!("method:{}, {}, {}",method.name(),method.descriptor(),(*method.class()).borrow().name());
        reader.reset(codes, pc);
        let opcode = reader.read_u8();
        let mut inst = new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        (*current_frame).borrow_mut().set_next_pc(reader.pc());
        inst.execute((*current_frame).borrow_mut().deref_mut());
        if (*thread).borrow().is_stack_empty() {
            break;
        }
    }
    println!("end {:?}",Local::now());
}

fn create_args_array(loader:Rc<RefCell<ClassLoader>>, args:&Vec<String>) -> Rc<RefCell<Object>> {
    let string_class = ClassLoader::load_class(
        loader.clone(),
        "java/lang/String"
    );
    let args_arr_class = (*string_class).borrow().array_class();
    let mut args_arr = Class::new_array(&args_arr_class,args.len());
    let java_args = args_arr.mut_references();
    for i in 0..java_args.len() {
        java_args[i] = Some(StringPool::java_string(loader.clone(),args[i].clone()));
    }
    return boxed(args_arr);
}


#[cfg(test)]
mod test {
    use std::rc::Rc;

    #[test]
    fn test_rc() {
        let mut vec = vec![0,1,2,3];
        let mut rc = Rc::new(vec);
//        rc.as_mut().push(4);
        println!("len:{}",rc.len())
    }
}