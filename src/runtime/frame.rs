use crate::oops::method::Method;
use crate::oops::object::Object;
use crate::runtime::local_vars::LocalVars;
use crate::runtime::operand_stack::OperandStack;
use crate::runtime::slot::Slot;
use crate::runtime::thread::JavaThread;
use std::cell::RefCell;
use std::rc::Rc;

struct Core {
    local_vars: Option<LocalVars>,
    operand_stack: Option<OperandStack>,
    method: Rc<Method>,
    next_pc: i32,
    frame_type: FrameType,
}

#[derive(Clone)]
pub struct Frame {
    core: Rc<RefCell<Core>>,
}

impl Frame {
    #[inline]
    pub fn new(method: Rc<Method>) -> Frame {
        return Frame {
            core: Rc::new(RefCell::new(Core {
                local_vars: LocalVars::with_capacity(method.max_locals()),
                operand_stack: OperandStack::new(method.max_stack()),
                method,
                next_pc: 0,
                frame_type: Default::default(),
            })),
        };
    }

    pub fn new_intrinsic_frame(method: Rc<Method>) -> Frame {
        Self::with_type(method, FrameType::IntrinsicFrame)
    }

    pub fn new_barrier_frame() -> Frame {
        let frame = Self::with_type(Rc::new(Method::default()), FrameType::BarrierFrame);
        (*frame.core).borrow_mut().operand_stack = OperandStack::new(1);
        frame
    }

    #[inline]
    fn with_type(method: Rc<Method>, frame_type: FrameType) -> Frame {
        return Frame {
            core: Rc::new(RefCell::new(Core {
                local_vars: LocalVars::with_capacity(method.max_locals()),
                operand_stack: OperandStack::new(method.max_stack()),
                method,
                next_pc: 0,
                frame_type,
            })),
        };
    }

    #[inline]
    pub fn with_capacity(max_locals: usize, max_stack: usize) -> Frame {
        return Frame {
            core: Rc::new(RefCell::new(Core {
                local_vars: LocalVars::with_capacity(max_locals),
                operand_stack: OperandStack::new(max_stack),
                method: Default::default(),
                next_pc: 0,
                frame_type: Default::default(),
            })),
        };
    }

    #[inline]
    pub fn next_pc(&self) -> i32 {
        return (*self.core).borrow().next_pc;
    }

    #[inline]
    pub fn set_next_pc(&self, next_pc: i32) {
        (*self.core).borrow_mut().next_pc = next_pc;
    }

    #[inline]
    pub fn revert_next_pc(&self) {
        (*self.core).borrow_mut().next_pc = JavaThread::current().get_pc();
    }

    #[inline]
    pub fn method(&self) -> Rc<Method> {
        return (*self.core).borrow().method.clone();
    }

    #[inline]
    pub fn is_intrinsic_frame(&self) -> bool {
        if let FrameType::IntrinsicFrame = (*self.core).borrow().frame_type {
            return true;
        }
        false
    }

    #[inline]
    pub fn is_barrier_frame(&self) -> bool {
        if let FrameType::BarrierFrame = (*self.core).borrow().frame_type {
            return true;
        }
        false
    }

    /// local_vars table operation
    ///
    pub fn local_vars_get<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&LocalVars) -> R,
    {
        let holder = (*self.core).borrow();
        let vars = holder.local_vars.as_ref().expect("vars is none");
        func(vars)
    }

    pub fn local_vars_set<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut LocalVars) -> R,
    {
        let mut holder = (*self.core).borrow_mut();
        let vars = holder.local_vars.as_mut().expect("vars is none");
        func(vars)
    }

    pub fn get_boolean(&self, index: usize) -> bool {
        self.local_vars_get(|v| v.get_boolean(index))
    }

    pub fn set_boolean(&self, index: usize, val: bool) {
        self.local_vars_set(|v| v.set_boolean(index, val))
    }

    pub fn get_int(&self, index: usize) -> i32 {
        self.local_vars_get(|v| v.get_int(index))
    }

    pub fn set_int(&self, index: usize, val: i32) {
        self.local_vars_set(|v| v.set_int(index, val))
    }

    pub fn get_float(&self, index: usize) -> f32 {
        self.local_vars_get(|v| v.get_float(index))
    }

    pub fn set_float(&self, index: usize, val: f32) {
        self.local_vars_set(|v| v.set_float(index, val))
    }

    pub fn get_long(&self, index: usize) -> i64 {
        self.local_vars_get(|v| v.get_long(index))
    }

    pub fn set_long(&self, index: usize, val: i64) {
        self.local_vars_set(|v| v.set_long(index, val))
    }

    pub fn get_double(&self, index: usize) -> f64 {
        self.local_vars_get(|v| v.get_double(index))
    }

    pub fn set_double(&self, index: usize, val: f64) {
        self.local_vars_set(|v| v.set_double(index, val))
    }

    pub fn get_ref(&self, index: usize) -> Option<Object> {
        self.local_vars_get(|v| v.get_ref(index))
    }

    pub fn set_ref(&self, index: usize, val: Option<Object>) {
        self.local_vars_set(|v| v.set_ref(index, val))
    }

    pub fn set_slot(&self, index: usize, val: Slot) {
        self.local_vars_set(|v| v.set_slot(index, val))
    }

    #[inline]
    pub fn get_this(&self) -> Option<Object> {
        return self.get_ref(0);
    }

    /// OperandStack operation
    ///
    pub fn operand_stack<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut OperandStack) -> R,
    {
        let mut holder = (*self.core).borrow_mut();
        let vars = holder.operand_stack.as_mut().expect("vars is none");
        func(vars)
    }

    #[inline]
    pub fn push_int(&self, val: i32) {
        self.operand_stack(|o| o.push_int(val))
    }

    #[inline]
    pub fn pop_int(&self) -> i32 {
        self.operand_stack(|o| o.pop_int())
    }

    #[inline]
    pub fn pop_boolean(&self) -> bool {
        self.operand_stack(|o| o.pop_boolean())
    }

    #[inline]
    pub fn push_float(&self, val: f32) {
        self.operand_stack(|o| o.push_float(val))
    }

    #[inline]
    pub fn pop_float(&self) -> f32 {
        self.operand_stack(|o| o.pop_float())
    }

    #[inline]
    pub fn push_long(&self, val: i64) {
        self.operand_stack(|o| o.push_long(val))
    }

    #[inline]
    pub fn pop_long(&self) -> i64 {
        self.operand_stack(|o| o.pop_long())
    }

    #[inline]
    pub fn push_double(&self, val: f64) {
        self.operand_stack(|o| o.push_double(val))
    }

    #[inline]
    pub fn pop_double(&self) -> f64 {
        self.operand_stack(|o| o.pop_double())
    }

    #[inline]
    pub fn push_ref(&self, val: Option<Object>) {
        self.operand_stack(|o| o.push_ref(val))
    }

    #[inline]
    pub fn pop_ref(&self) -> Option<Object> {
        self.operand_stack(|o| o.pop_ref())
    }

    #[inline]
    pub fn push_boolean(&self, val: bool) {
        self.operand_stack(|o| o.push_boolean(val))
    }

    #[inline]
    pub fn push_slot(&self, val: Slot) {
        self.operand_stack(|o| o.push_slot(val))
    }

    #[inline]
    pub fn pop_slot(&self) -> Slot {
        self.operand_stack(|o| o.pop_slot())
    }

    #[inline]
    pub fn get_ref_from_top(&self, index: usize) -> Option<Object> {
        let holder = (*self.core).borrow_mut();
        let vars = holder.operand_stack.as_ref().expect("vars is none");
        vars.get_ref_from_top(index)
    }
}

#[derive(Clone, Debug)]
enum FrameType {
    InterpreterFrame,
    IntrinsicFrame,
    BarrierFrame,
}

impl Default for FrameType {
    fn default() -> Self {
        FrameType::InterpreterFrame
    }
}

#[cfg(test)]
mod test {
    use crate::runtime::frame::Frame;
    use crate::runtime::local_vars::LocalVars;
    use crate::runtime::operand_stack::OperandStack;
    use crate::runtime::thread::JavaThread;
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::oops::object::Object;

    #[test]
    fn test_frame() {
//        let _thread = Rc::new(RefCell::new(JavaThread::new_thread()));
//        let frame = Frame::with_capacity(100, 100);
//        test_local_vars(&mut (*frame.core).borrow_mut().local_vars.take().unwrap());
//        test_operand_stack(&mut (*frame.core).borrow_mut().operand_stack.take().unwrap());
    }

    fn test_local_vars(vars: &mut LocalVars) {
        vars.set_int(0, 100);
        vars.set_int(1, -100);
        vars.set_long(2, 2997924580i64);
        vars.set_long(4, -2997924580i64);
        vars.set_float(6, 3.1415926f32);
        vars.set_double(7, 2.71828182845f64);
        vars.set_ref(9, None);
        println!("int:{}", vars.get_int(0));
        println!("int:{}", vars.get_int(1));
        println!("long:{}", vars.get_long(2));
        println!("long:{}", vars.get_long(4));
        println!("float:{}", vars.get_float(6));
        println!("double:{}", vars.get_double(7));
        println!("ref:{:?}", vars.get_ref(9));
    }

    fn test_operand_stack(ops: &mut OperandStack) {
        ops.push_int(100);
        ops.push_int(-100);
        ops.push_long(2997924580);
        ops.push_long(-2997924580);
        ops.push_float(3.1415926f32);
        ops.push_double(2.71828182845f64);
        ops.push_ref(None);
        println!("ref:{:?}", ops.pop_ref());
        println!("double:{}", ops.pop_double());
        println!("float:{}", ops.pop_float());
        println!("long:{}", ops.pop_long());
        println!("long:{}", ops.pop_long());
        println!("int:{}", ops.pop_int());
        println!("int:{}", ops.pop_int());
    }
}
