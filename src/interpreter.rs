use crate::class_file::member_info::MemberInfo;
use crate::runtime_data_area::thread::Thread;
use crate::instructions::base::bytecode_reader::BytecodeReader;

pub fn interpret(method_info:&mut MemberInfo) {
    let code_attr = method_info.code_attributes().expect("code_attr is none");
    let max_locals = code_attr.max_locals();
    let max_stack = code_attr.max_stack();
    let bytecode = code_attr.code();

    let mut thread = Thread::new_thread();
    let frame = thread.new_frame(maxLocals, maxStack);
    thread.push_frame(frame);
}

pub fn circulate(thread:&mut Thread,bytecode:&Vec<u8>) {
    let frame = thread.pop_frame();
    let mut reader = BytecodeReader::new();
    loop {
        let pc = frame.next_pc();
        thread.set_pc(pc);
        reader.reset(Vec::from(bytecode),pc);
        let opcode = reader.read_u8();
        let inst = instructions.NewInstruction(opcode);
        inst.FetchOperands(reader)
        frame.SetNextPC(reader.PC())
    }
}