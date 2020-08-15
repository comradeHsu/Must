use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::new_instruction;
use crate::native::init;
use crate::oops::class::Class;
use crate::oops::object::Object;
use crate::runtime::thread::JavaThread;
use crate::utils::boxed;
use chrono::Local;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
//use std::time;

pub fn interpret(thread: JavaThread) {
    circulate(thread);
}

#[inline]
pub fn circulate(mut thread: JavaThread) {
    let mut reader = BytecodeReader::new();
    init();
    println!("start {:?}", Local::now());
    loop {
        //        let mut borrow_thread = (*thread).borrow_mut();
        let current_frame = thread.current_frame();
        let pc = current_frame.next_pc();
        thread.set_pc(pc);
        let method = current_frame.method_ptr();
        let bytecode = method.code();
        reader.reset(bytecode, pc);
        let opcode = reader.read_u8();
        //println!("method:{}, {}, {},inst:{}",method.name(),method.descriptor(),(*method.class()).borrow().name(),opcode);
        let mut inst = new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        current_frame.set_next_pc(reader.pc());
        inst.execute(&current_frame);
        if thread.is_stack_empty() {
            break;
        }
        //        let ten_millis = time::Duration::from_millis(50);
        //        std::thread::sleep(ten_millis);
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
