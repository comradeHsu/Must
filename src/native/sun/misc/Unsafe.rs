use crate::runtime_data_area::frame::Frame;
use crate::native::registry::Registry;
use std::mem::size_of;
use crate::runtime_data_area::heap::object::DataType::{StandardObject, Ints};
use std::alloc::Layout;
use crate::utils::numbers::get_power_of_two;

pub fn init() {
    Registry::register("sun/misc/Unsafe", "arrayBaseOffset",
                       "(Ljava/lang/Class;)I", array_base_offset);
    Registry::register("sun/misc/Unsafe", "arrayIndexScale",
                       "(Ljava/lang/Class;)I", array_index_scale);
    Registry::register("sun/misc/Unsafe", "addressSize",
                       "()I", address_size);
    Registry::register("sun/misc/Unsafe", "objectFieldOffset",
                       "(Ljava/lang/reflect/Field;)J", object_field_offset);
    Registry::register("sun/misc/Unsafe","compareAndSwapObject",
                       "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
                       compare_and_swap_object);
    Registry::register("sun/misc/Unsafe", "getIntVolatile",
                       "(Ljava/lang/Object;J)I", get_int_volatile);
    Registry::register("sun/misc/Unsafe", "compareAndSwapInt",
                       "(Ljava/lang/Object;JII)Z", compare_and_swap_int);
    Registry::register("sun/misc/Unsafe", "allocateMemory",
                       "(J)J", allocate_memory);
    Registry::register("sun/misc/Unsafe", "putLong",
                       "(JJ)V", put_long);
    Registry::register("sun/misc/Unsafe", "getByte",
                       "(J)B", get_byte);
    Registry::register("sun/misc/Unsafe", "freeMemory",
                       "(J)V", free_memory);
}

pub fn array_base_offset(frame:&mut Frame) {
    frame.operand_stack().expect("stack is none").push_int(0);
}

pub fn array_index_scale(frame:&mut Frame) {
    frame.operand_stack().expect("stack is none").push_int(1);
}

pub fn address_size(frame:&mut Frame) {
    frame.operand_stack().expect("stack is none").push_int(size_of::<usize>() as i32);
}

// public native long objectFieldOffset(Field field);
// (Ljava/lang/reflect/Field;)J
pub fn object_field_offset(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let j_field = vars.get_ref(1).unwrap();

    let offset = (*j_field).borrow().get_int_var("slot", "I");

    let stack = frame.operand_stack().expect("stack is none");
    stack.push_long(offset as i64);
}

// public final native boolean compareAndSwapObject(Object o, long offset, Object expected, Object x)
// (Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z
pub fn compare_and_swap_object(frame:&mut Frame) {
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

// public native boolean getInt(Object o, long offset);
// (Ljava/lang/Object;J)I
pub fn get_int_volatile(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let object = vars.get_ref(1).unwrap();
    let borrow = (*object).borrow();
    let data = borrow.data();
    let offset = vars.get_long(2) as usize;

    let stack = frame.operand_stack().expect("stack is none");
    match data {
        StandardObject(inner) => {
            let slots = inner.as_ref().unwrap();
            stack.push_int(slots.get_int(offset))
        },
        Ints(inner) => {
            stack.push_int(inner[offset])
        },
        _ => panic!("getInt!")
    }
}

// public final native boolean compareAndSwapInt(Object o, long offset, int expected, int x);
// (Ljava/lang/Object;JII)Z
pub fn compare_and_swap_int(frame:&mut Frame) {
//    vars := frame.LocalVars()
//    fields := vars.GetRef(1).Data()
//    offset := vars.GetLong(2)
//    expected := vars.GetInt(4)
//    newVal := vars.GetInt(5)
//
//    if slots, ok := fields.(heap.Slots); ok {
//    // object
//        oldVal := slots.GetInt(uint(offset))
//        if oldVal == expected {
//            slots.SetInt(uint(offset), newVal)
//            frame.OperandStack().PushBoolean(true)
//        } else {
//            frame.OperandStack().PushBoolean(false)
//        }
//    } else if ints, ok := fields.([]int32); ok {
//    // int[]
//        oldVal := ints[offset]
//        if oldVal == expected {
//            ints[offset] = newVal
//            frame.OperandStack().PushBoolean(true)
//        } else {
//            frame.OperandStack().PushBoolean(false)
//        }
//    } else {
//        // todo
//        panic("todo: compareAndSwapInt!")
//    }
    frame.operand_stack().expect("stack is none").push_boolean(true);
}

/// public native long allocateMemory(long bytes);
/// (J)J
pub fn allocate_memory(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    // vars.GetRef(0) // this
    let bytes = vars.get_long(1) as usize;
    let layout = Layout::from_size_align(bytes,get_power_of_two(bytes))
        .expect("The layout init fail");
    unsafe {
        let ptr = std::alloc::alloc(layout) as usize;
        let stack = frame.operand_stack().expect("stack is none");
        memory_size_map::insert(ptr,bytes);
        stack.push_long(ptr as i64)
    }
}

/// public native void putLong(long address, long x);
/// (JJ)V
pub fn put_long(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    // vars.GetRef(0) // this
    let address = vars.get_long(1);
    let value = vars.get_long(3);

    let ptr = (address as usize) as *mut u8;
    unsafe {
        *(ptr as *mut i64) = value;
    }
}

/// public native byte getByte(long address);
/// (J)B
pub fn get_byte(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    // vars.GetRef(0) // this
    let address = vars.get_long(1);
    let ptr = (address as usize) as *mut u8;
    unsafe {
        let value = *(ptr as *mut i8);
        frame.operand_stack().expect("stack is none").push_int(value as i32);
    }
}

/// public native void freeMemory(long address);
/// (J)V
pub fn free_memory(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    // vars.GetRef(0) // this
    let address = vars.get_long(1) as usize;
    let size = memory_size_map::get(address);
    let layout = Layout::from_size_align(size,get_power_of_two(size))
        .expect("The layout init fail");
    unsafe {
        std::alloc::dealloc(address as *mut u8, layout);
        memory_size_map::delete(address);
    }
}

mod memory_size_map {
    use std::collections::HashMap;

    static mut MEMORY_SIZE_MAP: Option<HashMap<usize, usize>> = None;

    fn instance() -> &'static mut HashMap<usize, usize> {
        unsafe {
            if MEMORY_SIZE_MAP.is_none() {
                MEMORY_SIZE_MAP = Some(HashMap::new());
            }
            return MEMORY_SIZE_MAP.as_mut().unwrap();
        }
    }

    pub fn insert(key:usize,size:usize) {
        instance().insert(key,size);
    }

    pub fn get(key:usize) -> usize {
        let size = instance().get(&key);
        return *size.unwrap();
    }

    pub fn delete(key:usize) {
        instance().remove(&key);
    }
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
    use std::alloc::Layout;
    use crate::utils::numbers::get_power_of_two;

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
            println!("size:{}", size_of::<i64>());
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

    #[test]
    fn test_alloc() {
        let bytes = 3 as usize;
        let layout = Layout::from_size_align(bytes,get_power_of_two(bytes))
            .expect("The layout init fail");
        unsafe {
            let ptr = std::alloc::alloc(layout.clone());
            let ptr_1 = std::alloc::alloc(layout);
            println!("address:{},sec_address:{}", ptr as usize,ptr_1 as usize);
        }
        println!("size:{}", layout.size());
    }

    #[test]
    fn test_put() {
        let bytes = 8 as usize;
        let layout = Layout::from_size_align(bytes,get_power_of_two(bytes))
            .expect("The layout init fail");
        unsafe {
            let ptr = std::alloc::alloc(layout.clone());
            *(ptr as *mut i32) = 4;
            let next = ptr as usize + 4;
            *(next as *mut i32) = 16;
            println!("value:{},next:{}", *(ptr as *mut i32),*(next as *mut i32));
            println!("ptr:{},next_ptr:{}",ptr as usize,next as usize);
        }
        println!("size:{}", layout.size());
    }

    #[test]
    fn test_get_byte() {
        let bytes = 8 as usize;
        let layout = Layout::from_size_align(bytes,get_power_of_two(bytes))
            .expect("The layout init fail");
        unsafe {
            let ptr = std::alloc::alloc(layout.clone());
            *(ptr as *mut i32) = 129;
            println!("byte:{},real value:{}",*(ptr as *mut i8),*(ptr as *mut i32));
        }
        println!("size:{}", layout.size());
    }
}