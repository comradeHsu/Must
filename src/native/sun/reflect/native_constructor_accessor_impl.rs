use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::method_invoke_logic::hack_invoke_method;
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::operand_stack::OperandStack;
use crate::utils::boxed;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init() {
    Registry::register(
        "sun/reflect/NativeConstructorAccessorImpl",
        "newInstance0",
        "(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
        new_instance0,
    );
}

// private static native Object newInstance0(Constructor<?> c, Object[] os)
// throws InstantiationException, IllegalArgumentException, InvocationTargetException;
// (Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;
pub fn new_instance0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let constructor_obj = vars.get_ref(0).unwrap();
    let arg_arr_obj = vars
        .get_ref(1)
        .unwrap_or_else(|| boxed(Object::new(boxed(Class::none()))));

    let constructor = get_constructor(constructor_obj);
    let class = constructor.class();

    if !(*class).borrow().initialized() {
        frame.revert_next_pc();
        init_class(frame.thread(), class);
        return;
    }
    let obj = Some(boxed(Class::new_object(&class)));
    let stack = frame.operand_stack().expect("stack is none");
    stack.push_ref(obj.clone());

    // call <init>
    let ops = convert_args(obj.unwrap(), arg_arr_obj, constructor.clone());
    let shim_frame = Frame::new_shim_frame(
        frame.thread(),
        ops.unwrap_or_else(|| OperandStack::new(0).unwrap()),
    );
    let thread = frame.thread();
    (*thread).borrow_mut().push_frame(shim_frame);

    hack_invoke_method(thread, constructor);
}

fn get_method(method_obj: Rc<RefCell<Object>>) -> Rc<Method> {
    return _get_method(method_obj, false);
}

fn get_constructor(constructor_obj: Rc<RefCell<Object>>) -> Rc<Method> {
    return _get_method(constructor_obj, true);
}

fn _get_method(method_obj: Rc<RefCell<Object>>, is_constructor: bool) -> Rc<Method> {
    let extra = (*method_obj).borrow().meta_data.clone();
    if extra.not_null() {
        return extra.method();
    }

    if is_constructor {
        let root = (*method_obj)
            .borrow()
            .get_ref_var("root", "Ljava/lang/reflect/Constructor;")
            .expect("the object hasn't root attribute");
        return (*root).borrow().meta_data.method();
    } else {
        let root = (*method_obj)
            .borrow()
            .get_ref_var("root", "Ljava/lang/reflect/Method;")
            .expect("the object hasn't root attribute");
        return (*root).borrow().meta_data.method();
    }
}

// Object[] -> []interface{}
fn convert_args(
    this: Rc<RefCell<Object>>,
    _arg_arr: Rc<RefCell<Object>>,
    method: Rc<Method>,
) -> Option<OperandStack> {
    if method.arg_slot_count() == 0 {
        return None;
    }

    //	argObjs := arg_arr.Refs()
    //	argTypes := method.ParsedDescriptor().ParameterTypes()

    let mut ops = OperandStack::new(method.arg_slot_count()).unwrap();
    if !method.is_static() {
        ops.push_ref(Some(this));
    }
    if method.arg_slot_count() == 1 && !method.is_static() {
        return Some(ops);
    }

    //	for i, argType := range argTypes {
    //		argObj := argObjs[i]
    //
    //		if len(argType) == 1 {
    //			// base type
    //			// todo
    //			unboxed := box.Unbox(argObj, argType)
    //			args[i+j] = unboxed
    //			if argType.isLongOrDouble() {
    //				j++
    //			}
    //		} else {
    //			args[i+j] = argObj
    //		}
    //	}

    return Some(ops);
}
