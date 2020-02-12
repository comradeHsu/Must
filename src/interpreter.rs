use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::new_instruction;
use crate::native::init;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::thread::JavaThread;
use crate::utils::boxed;
use chrono::Local;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

pub fn interpret(thread: Rc<RefCell<JavaThread>>) {
    circulate(thread);
}

#[inline]
pub fn circulate(mut thread: Rc<RefCell<JavaThread>>) {
    let mut reader = BytecodeReader::new();
    init();
    println!("start {:?}", Local::now());
    loop {
        //        let mut borrow_thread = (*thread).borrow_mut();
        let current_frame = (*thread).borrow().current_frame();
        let pc = (*current_frame).borrow().next_pc();
        (*thread).borrow_mut().set_pc(pc);
        let method = (*current_frame).borrow().method_ptr();
        let bytecode = method.code();
        //        println!("method:{}, {}, {}",method.name(),method.descriptor(),(*method.class()).borrow().name());
        reader.reset(bytecode, pc);
        let opcode = reader.read_u8();
        let mut inst = new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        (*current_frame).borrow_mut().set_next_pc(reader.pc());
        inst.execute((*current_frame).borrow_mut().deref_mut());
        if (*thread).borrow().is_stack_empty() {
            break;
        }
    }
    println!("end {:?}", Local::now());
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    #[test]
    fn test_rc() {
        let mut vec = vec![0, 1, 2, 3];
        let mut rc = Rc::new(vec);
        //        rc.as_mut().push(4);
        println!("len:{}", rc.len())
    }
}
