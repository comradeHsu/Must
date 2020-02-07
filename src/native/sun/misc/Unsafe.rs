use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;
use std::mem::size_of;

pub fn init() {
    Registry::register("sun/misc/Unsafe", "arrayBaseOffset",
                       "(Ljava/lang/Class;)I", array_base_offset);
    Registry::register("sun/misc/Unsafe", "arrayIndexScale",
                       "(Ljava/lang/Class;)I", array_index_scale);
    Registry::register("sun/misc/Unsafe", "addressSize",
                       "()I", addressSize);
    Registry::register("sun/misc/Unsafe", "objectFieldOffset",
                       "(Ljava/lang/reflect/Field;)J", objectFieldOffset);
    Registry::register("sun/misc/Unsafe","compareAndSwapObject",
                       "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
                       compareAndSwapObject);
}

pub fn array_base_offset(frame:&mut Frame) {
    frame.operand_stack().expect("stack is none").push_int(0);
}

pub fn array_index_scale(frame:&mut Frame) {
    frame.operand_stack().expect("stack is none").push_int(1);
}

pub fn addressSize(frame:&mut Frame) {
    frame.operand_stack().expect("stack is none").push_int(size_of::<usize>() as i32);
}

// public native long objectFieldOffset(Field field);
// (Ljava/lang/reflect/Field;)J
pub fn objectFieldOffset(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let jField = vars.get_ref(1).unwrap();

    let offset = (*jField).borrow().get_int_var("slot", "I");

    let stack = frame.operand_stack().expect("stack is none");
    stack.push_long(offset as i64);
}

// public final native boolean compareAndSwapObject(Object o, long offset, Object expected, Object x)
// (Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z
pub fn compareAndSwapObject(frame:&mut Frame) {
//    vars := frame.LocalVars()
//    obj := vars.GetRef(1)
//    fields := obj.Data()
//    offset := vars.GetLong(2)
//    expected := vars.GetRef(4)
//    newVal := vars.GetRef(5)
//
//    // todo
//    if anys, ok := fields.(heap.Slots); ok {
//    // object
//        swapped := _casObj(obj, anys, offset, expected, newVal)
//        frame.OperandStack().PushBoolean(swapped)
//    } else if objs, ok := fields.([]*heap.Object); ok {
//    // ref[]
//        swapped := _casArr(objs, offset, expected, newVal)
//        frame.OperandStack().PushBoolean(swapped)
//    } else {
//    // todo
//        panic("todo: compareAndSwapObject!")
//    }
    let stack = frame.operand_stack().expect("stack is none");
    stack.push_boolean(true);
}

#[cfg(test)]
mod java_unsafe {
    use crate::runtime_data_area::heap::object::Object;
    use crate::runtime_data_area::heap::object::DataType::{Ints, Bytes};
    use crate::utils::boxed;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::Deref;
    use crate::runtime_data_area::heap::class::Class;
    use crate::class_path::class_path::ClassPath;
    use crate::runtime_data_area::heap::class_loader::ClassLoader;
    use crate::cmd::Cmd;
    use std::mem::size_of;

    struct Test {
        len:usize,
        data:Vec<i32>
    }

    impl Test {
        pub fn data(&self) -> &Vec<i32> {
            return &self.data;
        }
    }

    #[test]
    fn test() {
//        let vec = Vec::with_capacity(10);
        let test = Test{ len: 0, data: vec![1,2,3].clone() };
        let b = Box::new(10);
        let ptr = boxed(test);
        let ptr = (*ptr).borrow();
        let first = ptr.data().get(0).unwrap();
        let ptr = ptr.deref() as *const Test;
        let hash = ptr as usize;
        let first_ref = first as *const i32;
        let first_ptr = first_ref as usize;
        println!("object ptr:{}, first element ptr:{},差距:{}",hash,first_ptr,first_ptr-hash);
        unsafe {
            println!("size:{}", size_of::<Test>());
            println!("size:{}", size_of::<Object>());
            println!("size:{}", size_of::<usize>());
        }
    }

    #[test]
    fn test_int_array_offset() {
        let cmd = Cmd{
            help_flag: false,
            version_flag: false,
            verbose_class: false,
            cp_option: "D:/workspace/rust-jvm".to_string(),
            x_jre_option: "".to_string(),
            class: "java.ParseIntTest".to_string(),
            args: vec![]
        };
        let cp = ClassPath::parse(&cmd.x_jre_option,&cmd.cp_option);
        let class_path = Rc::new(cp);
        let class_loader = ClassLoader::new(class_path,cmd.verbose_class);
        let class = ClassLoader::load_class(class_loader,"[I");
        let object = Class::new_array(&class,10);
        let ptr = boxed(object);
        let ptr = (*ptr).borrow();
        let bytes = ptr.ints();
        let first = bytes.get(0).unwrap();
        let first_ref = first as *const i32;
        let ptr = ptr.deref() as *const Object;
        let hash = ptr as usize;
        let first_ptr = first_ref as usize;
        println!("object ptr:{}, first element ptr:{},差距:{}",hash,first_ptr,first_ptr-hash);
    }

    #[test]
    fn test_byte_array_offset() {
        let mut object = Object::new(boxed(Class::none()));
        object.data = Bytes(vec![1,2,3]);
        let ptr = boxed(object);
        let ptr = (*ptr).borrow();
        let bytes = ptr.bytes();
        let first = bytes.get(0).unwrap();
        let first_ref = first as *const i8;
        let ptr = ptr.deref() as *const Object;
        let hash = ptr as usize;
        let first_ptr = first_ref as usize;
        println!("object ptr:{}, first element ptr:{},差距:{}",hash,first_ptr,first_ptr-hash);
    }
}