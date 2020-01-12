use crate::class_file::member_info::MemberInfo;
use crate::runtime_data_area::thread::Thread;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::new_instruction;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::cell::RefCell;

pub fn interpret(method_info:&MemberInfo) {
    println!("{}",method_info.name());
    let code_attr = method_info.code_attributes().expect("code_attr is none");
    code_attr.display();
    let max_locals = code_attr.max_locals() as usize;
    let max_stack = code_attr.max_stack() as usize;
    let bytecode = code_attr.code();

    let mut thread = Rc::new(Thread::new_thread());
//    let mut point = Rc::new(thread);
    let frame = Thread::new_frame(thread.clone(),max_locals, max_stack);
    Rc::get_mut(&mut thread).unwrap().push_frame(frame);
    circulate(Rc::get_mut(&mut thread).unwrap(),bytecode);
}

pub fn circulate(thread:&mut Thread,bytecode:&Vec<u8>) {
    let mut frame = thread.pop_frame();
    let mut reader = BytecodeReader::new();
    loop {
        let pc = frame.next_pc();
        thread.set_pc(pc);
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