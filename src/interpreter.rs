use crate::class_file::member_info::MemberInfo;
use crate::runtime_data_area::thread::Thread;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::new_instruction;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::cell::RefCell;

pub fn interpret(method_info:&MemberInfo) {
    let code_attr = method_info.code_attributes().expect("code_attr is none");
    let max_locals = code_attr.max_locals() as usize;
    let max_stack = code_attr.max_stack() as usize;
    let bytecode = code_attr.code();

    let thread = Rc::new(RefCell::new(Thread::new_thread()));
    let frame = Thread::new_frame(thread.clone(),max_locals, max_stack);
    (*thread).borrow_mut().push_frame(frame);
    circulate(thread,bytecode);
}

pub fn circulate(mut thread:Rc<RefCell<Thread>>,bytecode:&Vec<u8>) {
    let mut frame = (*thread).borrow_mut().pop_frame();
    let mut reader = BytecodeReader::new();
    loop {
        let pc = frame.next_pc();
        (*thread).borrow_mut().set_pc(pc);
        let mut codes = Vec::with_capacity(bytecode.len());
        for c in bytecode {
            codes.push(*c);
        }
        reader.reset(codes,pc);
        let opcode = reader.read_u8();
        let mut inst = new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        frame.set_next_pc(reader.pc());
        inst.execute(&mut frame);
    }
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