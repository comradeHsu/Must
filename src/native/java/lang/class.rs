use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::native::registry::Registry;
use crate::utils::{java_str_to_rust_str, boxed};
use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::heap::array_object::ArrayObject;
use crate::runtime_data_area::heap::class::Class;
use std::ops::Deref;
use crate::instructions::base::class_init_logic::init_class;
use crate::runtime_data_area::operand_stack::OperandStack;
use crate::instructions::base::method_invoke_logic::{invoke_method, hack_invoke_method};
use crate::runtime_data_area::heap::object::DataType::Bytes;
use crate::runtime_data_area::heap::object::MetaData::{Field, Method};

pub fn init() {
    Registry::register("java/lang/Class", "getPrimitiveClass",
                       "(Ljava/lang/String;)Ljava/lang/Class;", get_primitive_class);
    Registry::register("java/lang/Class", "getName0",
                       "()Ljava/lang/String;", get_name0);
    Registry::register("java/lang/Class", "desiredAssertionStatus0",
                       "(Ljava/lang/Class;)Z", desired_assertion_status0);
    Registry::register("java/lang/Class", "forName0",
                       "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", for_name0);
    Registry::register("java/lang/Class", "isInterface", "()Z", is_interface);
    Registry::register("java/lang/Class", "isPrimitive", "()Z", is_primitive);
    Registry::register("java/lang/Class", "getDeclaredFields0", "(Z)[Ljava/lang/reflect/Field;", getDeclaredFields0);
    Registry::register("java/lang/Class", "getDeclaredConstructors0", "(Z)[Ljava/lang/reflect/Constructor;", getDeclaredConstructors0);
    Registry::register("java/lang/Class", "getModifiers", "()I", get_modifiers);
    Registry::register("java/lang/Class", "getSuperclass", "()Ljava/lang/Class;", get_superclass);
    Registry::register("java/lang/Class", "getInterfaces0", "()[Ljava/lang/Class;", get_interfaces0);
    Registry::register("java/lang/Class", "isArray", "()Z", is_array);
//    Registry::register("java/lang/Class", "getDeclaredMethods0", "(Z)[Ljava/lang/reflect/Method;", getDeclaredMethods0);
    Registry::register("java/lang/Class", "getComponentType", "()Ljava/lang/Class;", get_component_type);
    Registry::register("java/lang/Class", "isAssignableFrom", "(Ljava/lang/Class;)Z", is_assignable_from);
}

pub fn get_primitive_class(frame:&mut Frame) {
    let name_obj = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let target = java_str_to_rust_str(name_obj);
    let class = frame.method().class();
    let loader = (*class).borrow().loader();
    let class = ClassLoader::load_class(loader,target.as_str());
    let java_class = (*class).borrow().get_java_class();
    frame.operand_stack().expect("stack null").push_ref(java_class);
}

pub fn get_name0(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta().unwrap();
    let name = (*class).borrow().java_name();
    let name_obj = StringPool::java_string((*class).borrow().loader(),name);
    frame.operand_stack().expect("stack null").push_ref(Some(name_obj));
}

pub fn desired_assertion_status0(frame:&mut Frame) {
    frame.operand_stack().expect("stack null").push_int(0);
}

pub fn for_name0(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let name = vars.get_ref(0);
    let initialize = vars.get_boolean(1);

    let rust_name = java_str_to_rust_str(name.unwrap()).replace('.',"/");
    let class = frame.method().class();
    let loader = (*class).borrow().loader();
    let class = ClassLoader::load_class(loader,rust_name.as_str());
    let java_class = (*class).borrow().get_java_class();
    if initialize && !(*class).borrow().initialized() {
        let thread = frame.thread();
        frame.set_next_pc((*thread).borrow().get_pc());
        init_class(thread,class);
    } else {
        frame.operand_stack().expect("stack null").push_ref(java_class);
    }
}

pub fn is_interface(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta();
    frame.operand_stack().expect("stack null").push_boolean((*class.unwrap()).borrow().is_interface());
}

pub fn is_primitive(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta();
    frame.operand_stack().expect("stack null").push_boolean((*class.unwrap()).borrow().is_primitive());
}

pub fn get_modifiers(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta();
    println!("\tclass:::{}",(*class.clone().unwrap()).borrow().name());
    frame.operand_stack().expect("stack null").push_int((*class.unwrap()).borrow().access_flags() as i32);
}

pub fn get_superclass(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta().unwrap();
    let super_class = (*class).borrow().super_class();
    let mut java_class: Option<Rc<RefCell<Object>>> = None;
    if super_class.is_some() {
        java_class = (*super_class.unwrap()).borrow().get_java_class();
    }
    frame.operand_stack().expect("stack null").push_ref(java_class);
}

pub fn get_interfaces0(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta().unwrap();
    let borrow = (*class).borrow();
    let interfaces = borrow.interfaces();
    let none = Vec::new();
    let class_arr = to_class_arr(
        (*class).borrow().loader(),
                interfaces.unwrap_or_else(|| &none)
    );
    frame.operand_stack().expect("stack null").push_ref(Some(boxed(class_arr)));
}

// []*Class => Class[]
fn to_class_arr(loader:Rc<RefCell<ClassLoader>>, classes:&Vec<Rc<RefCell<Class>>>) -> ArrayObject {
    let arr_len = classes.len();

    let class_arr_class = (*ClassLoader::load_class(loader, "java/lang/Class")).borrow().array_class();
    let mut class_arr = Class::new_array(&class_arr_class, arr_len);

    if arr_len > 0 {
        let class_objs = class_arr.mut_references();
        for i in 0..arr_len {
            class_objs[i] = (*classes[i].clone()).borrow().get_java_class();
        }
    }

    return class_arr;
}

// []byte => byte[]
fn toByteArr(loader:Rc<RefCell<ClassLoader>>, rbytes:Option<Vec<u8>>) -> Option<ArrayObject> {
    if rbytes.is_some() {
        let j_bytes:Vec<i8> = rbytes.unwrap().iter().map(|x| *x as i8).collect();
        return Some(ArrayObject::from_data(ClassLoader::load_class(loader,"[B"),Bytes(j_bytes)));
    }
    return None
}

pub fn is_array(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta();
    frame.operand_stack().expect("stack null").push_boolean((*class.unwrap()).borrow().is_array());
}

// public native Class<?> getComponentType();
// ()Ljava/lang/Class;
pub fn get_component_type(frame:&mut Frame) {
    let this = frame.local_vars().expect("vars is none")
        .get_this().unwrap();
    let class = (*this).borrow().meta().unwrap();
    let component_class = (*class).borrow().component_class();
    frame.operand_stack().expect("stack null").push_ref((*component_class).borrow().get_java_class());
}

// public native boolean isAssignableFrom(Class<?> cls);
// (Ljava/lang/Class;)Z
pub fn is_assignable_from(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let this = vars.get_this();
    let cls = vars.get_ref(1);

    let this_class = (*this.unwrap()).borrow().meta().unwrap();
    let cls_class = (*cls.unwrap()).borrow().meta().unwrap();
    let ok = (*this_class).borrow().is_assignable_from((*cls_class).borrow().deref());

    frame.operand_stack().expect("stack null").push_boolean(ok);
}

const _constructorConstructorDescriptor:&str = "(Ljava/lang/Class;[Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B)V";

// private native Constructor<T>[] getDeclaredConstructors0(boolean publicOnly);
// (Z)[Ljava/lang/reflect/Constructor;
pub fn getDeclaredConstructors0(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let classObj = vars.get_this().unwrap();
    let publicOnly = vars.get_boolean(1);

    let class = (*classObj).borrow().meta().unwrap();
    let constructors = (*class).borrow().get_constructors(publicOnly);
    let constructorCount = constructors.len();

    let class = frame.method().class();
    let classLoader = (*class).borrow().loader();
    let constructorClass = ClassLoader::load_class(classLoader.clone(),"java/lang/reflect/Constructor");

    let class_arr_class = (*constructorClass).borrow().array_class();
    let constructorArr = Class::new_array(&class_arr_class,constructorCount);

    let boxed_arr = Some(boxed(constructorArr));
    frame.operand_stack().expect("stack null").push_ref(boxed_arr.clone());

    if constructorCount > 0 {
        let thread = frame.thread();
        let arr = boxed_arr.unwrap();
        let mut temp =  (*arr).borrow_mut();
        let constructorObjs = temp.mut_references();
        let constructorInitMethod = Class::get_constructor(constructorClass.clone(),_constructorConstructorDescriptor);
        for i in 0..constructors.len() {
            let constructor = constructors[i].clone();
            let mut constructorObj = Class::new_object(&constructorClass);
            constructorObj.set_meta_data(Method(constructor.clone()));
            let object = Some(boxed(constructorObj));
            constructorObjs[i] = object.clone();

            let mut ops = OperandStack::new(9).unwrap();
            ops.push_ref(object);                                                // this
            ops.push_ref(Some(classObj.clone()));                                                     // declaringClass
            let parameter_types = constructor.parameter_types().unwrap();
            ops.push_ref(Some(boxed(to_class_arr(classLoader.clone(), &parameter_types))));         // parameterTypes
            let exception_types = constructor.exception_types().unwrap_or_else(||Vec::new());
            ops.push_ref(Some(boxed(to_class_arr(classLoader.clone(), &exception_types))));         // checkedExceptions
            ops.push_int(constructor.access_flags() as i32);                              // modifiers
            ops.push_int(0);                                                      // todo slot
            ops.push_ref(getSignatureStr(classLoader.clone(), constructor.signature()));       // signature
            let mut data:Vec<u8> = vec![0,20];
            ops.push_ref(Some(boxed(toByteArr(classLoader.clone(), Some((data))).unwrap())));
            let mut data:Vec<u8> = vec![0,20];// annotations
            ops.push_ref(Some(boxed(toByteArr(classLoader.clone(), Some(data)).unwrap()))); // parameterAnnotations

            let shimFrame = Frame::new_shim_frame(thread.clone(), ops);
            (*thread).borrow_mut().push_frame(shimFrame);

            // init constructorObj
            hack_invoke_method(thread.clone(), constructorInitMethod.clone().unwrap());
        }
    }
}

const _fieldConstructorDescriptor:&str = "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IILjava/lang/String;[B)V";

// private native Field[] getDeclaredFields0(boolean publicOnly);
// (Z)[Ljava/lang/reflect/Field;
pub fn getDeclaredFields0(frame:&mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let classObj = vars.get_this().unwrap();
    let publicOnly = vars.get_boolean(1);

    let class = (*classObj).borrow().meta().unwrap();
    let fields = (*class).borrow().get_fields(publicOnly);
    let fieldCount = fields.len();

    let class = frame.method().class();
    let classLoader = (*class).borrow().loader();
    let fieldClass = ClassLoader::load_class(classLoader.clone(),"java/lang/reflect/Field");
    let field_arr_class = (*fieldClass).borrow().array_class();
    let fieldArr = Class::new_array(&field_arr_class,fieldCount);

    let boxed_arr = Some(boxed(fieldArr));
    frame.operand_stack().expect("stack null").push_ref(boxed_arr.clone());

    if fieldCount > 0 {
        let thread = frame.thread();
        let arr = boxed_arr.unwrap();
        let mut temp =  (*arr).borrow_mut();
        let fieldObjs = temp.mut_references();
        let fieldInitMethod = Class::get_constructor(fieldClass.clone(),_fieldConstructorDescriptor);
        for i in 0..fields.len() {
            let field = fields[i].clone();
            let mut fieldObj = Class::new_object(&fieldClass);
            fieldObj.set_meta_data(Field(field.clone()));
            let object = Some(boxed(fieldObj));
            fieldObjs[i] = object.clone();

            let mut ops = OperandStack::new(8).unwrap();
            ops.push_ref(object);                                        // this
            ops.push_ref(Some(classObj.clone()));                                     // declaringClass
            ops.push_ref(Some(StringPool::java_string(
                classLoader.clone(),
                (*field).borrow().name().to_string()))
            );      // name
            ops.push_ref((*(*field).borrow().r#type()).borrow().get_java_class());                           // type
            ops.push_int((*field).borrow().access_flags() as i32);                      // modifiers
            ops.push_int((*field).borrow().slot_id() as i32);                  // slot
            ops.push_ref(getSignatureStr(classLoader.clone(), (*field).borrow().signature()));// signature
            let mut data:Vec<u8> = vec![0,20];

            ops.push_ref(Some(boxed(toByteArr(classLoader.clone(), Some(data)).unwrap())));  // annotations

            let shimFrame = Frame::new_shim_frame(thread.clone(), ops);
            (*thread).borrow_mut().push_frame(shimFrame);

            // init fieldObj
            hack_invoke_method(thread.clone(), fieldInitMethod.clone().unwrap());
        }
    }
}

fn getSignatureStr(loader:Rc<RefCell<ClassLoader>>, signature:&str) -> Option<Rc<RefCell<Object>>> {
    if signature != "" {
        return Some(StringPool::java_string(loader, signature.to_string()));
    }
    return None
}