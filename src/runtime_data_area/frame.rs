use crate::runtime_data_area::local_vars::LocalVars;
use crate::runtime_data_area::operand_stack::OperandStack;
use crate::runtime_data_area::thread::Thread;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Frame {
    local_vars:Option<LocalVars>,
    operand_stack:Option<OperandStack>,
    thread:Rc<Thread>,
    next_pc:i32
}

impl Frame {
//    #[inline]
//    pub fn new() -> Frame {
//        return Frame{
//            local_vars: None,
//            operand_stack: None,
//            thread: &(),
//            next_pc: 0
//        };
//    }

    #[inline]
    pub fn with_capacity(thread:Rc<Thread>,max_locals:usize,max_stack:usize) -> Frame {
        return Frame{
            local_vars: LocalVars::with_capacity(max_locals),
            operand_stack: OperandStack::new(max_stack),
            thread: thread,
            next_pc: 0
        };
    }

    #[inline]
    pub fn operand_stack(&mut self) -> Option<&mut OperandStack>{
        return self.operand_stack.as_mut();
    }

    #[inline]
    pub fn local_vars(&mut self) -> Option<&mut LocalVars>{
        return self.local_vars.as_mut();
    }

    #[inline]
    pub fn next_pc(&self) -> i32{
        return self.next_pc;
    }

    #[inline]
    pub fn set_next_pc(&mut self,next_pc:i32) {
        self.next_pc = next_pc;
    }
}

#[cfg(test)]
mod test {
    use crate::runtime_data_area::local_vars::LocalVars;
    use crate::runtime_data_area::operand_stack::OperandStack;
    use crate::runtime_data_area::frame::Frame;
    use crate::runtime_data_area::thread::Thread;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_frame() {
        let thread = Rc::new((Thread::new_thread()));
        let mut frame = Frame::with_capacity(thread,100,100);
        test_local_vars(&mut frame.local_vars.unwrap());
        test_operand_stack(&mut frame.operand_stack.unwrap());
    }

    fn test_local_vars(vars:&mut LocalVars) {
        vars.set_int(0, 100);
        vars.set_int(1, -100);
        vars.set_long(2, 2997924580i64);
        vars.set_long(4, -2997924580i64);
        vars.set_float(6, 3.1415926f32);
        vars.set_double(7, 2.71828182845f64);
        vars.set_ref(9, None);
        println!("int:{}",vars.get_int(0));
        println!("int:{}",vars.get_int(1));
        println!("long:{}",vars.get_long(2));
        println!("long:{}",vars.get_long(4));
        println!("float:{}",vars.get_float(6));
        println!("double:{}",vars.get_double(7));
        println!("ref:{:?}",vars.get_ref(9));
    }

    fn test_operand_stack(ops:&mut OperandStack) {
        ops.push_int(100);
        ops.push_int(-100);
        ops.push_long(2997924580);
        ops.push_long(-2997924580);
        ops.push_float(3.1415926f32);
        ops.push_double(2.71828182845f64);
        ops.push_ref(None);
        println!("ref:{:?}",ops.pop_ref());
        println!("double:{}",ops.pop_double());
        println!("float:{}",ops.pop_float());
        println!("long:{}",ops.pop_long());
        println!("long:{}",ops.pop_long());
        println!("int:{}",ops.pop_int());
        println!("int:{}",ops.pop_int());
    }

}