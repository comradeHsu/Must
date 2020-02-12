use crate::instructions::base::class_init_logic::init_class;
use crate::instructions::base::method_invoke_logic::{hack_invoke_method, invoke_method};
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::array_object::ArrayObject;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::object::DataType::Bytes;
use crate::runtime_data_area::heap::object::MetaData::{Field, Method};
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::runtime_data_area::operand_stack::OperandStack;
use crate::utils::{boxed, java_str_to_rust_str};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub fn init() {
    Registry::register(
        "java/lang/Class",
        "getPrimitiveClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        get_primitive_class,
    );
    Registry::register(
        "java/lang/Class",
        "getName0",
        "()Ljava/lang/String;",
        get_name0,
    );
    Registry::register(
        "java/lang/Class",
        "desiredAssertionStatus0",
        "(Ljava/lang/Class;)Z",
        desired_assertion_status0,
    );
    Registry::register(
        "java/lang/Class",
        "forName0",
        "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;",
        for_name0,
    );
    Registry::register("java/lang/Class", "isInterface", "()Z", is_interface);
    Registry::register("java/lang/Class", "isPrimitive", "()Z", is_primitive);
    Registry::register(
        "java/lang/Class",
        "getDeclaredFields0",
        "(Z)[Ljava/lang/reflect/Field;",
        get_declared_fields0,
    );
    Registry::register(
        "java/lang/Class",
        "getDeclaredConstructors0",
        "(Z)[Ljava/lang/reflect/Constructor;",
        get_declared_constructors0,
    );
    Registry::register("java/lang/Class", "getModifiers", "()I", get_modifiers);
    Registry::register(
        "java/lang/Class",
        "getSuperclass",
        "()Ljava/lang/Class;",
        get_superclass,
    );
    Registry::register(
        "java/lang/Class",
        "getInterfaces0",
        "()[Ljava/lang/Class;",
        get_interfaces0,
    );
    Registry::register("java/lang/Class", "isArray", "()Z", is_array);
    Registry::register(
        "java/lang/Class",
        "getDeclaredMethods0",
        "(Z)[Ljava/lang/reflect/Method;",
        get_declared_methods0,
    );
    Registry::register(
        "java/lang/Class",
        "getComponentType",
        "()Ljava/lang/Class;",
        get_component_type,
    );
    Registry::register(
        "java/lang/Class",
        "isAssignableFrom",
        "(Ljava/lang/Class;)Z",
        is_assignable_from,
    );
}

pub fn get_primitive_class(frame: &mut Frame) {
    let name_obj = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let target = java_str_to_rust_str(name_obj);
    let class = frame.method().class();
    let loader = (*class).borrow().loader();
    let class = ClassLoader::load_class(loader, target.as_str());
    let java_class = (*class).borrow().get_java_class();
    frame
        .operand_stack()
        .expect("stack null")
        .push_ref(java_class);
}

pub fn get_name0(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().meta().unwrap();
    let name = (*class).borrow().java_name();
    let name_obj = StringPool::java_string((*class).borrow().loader(), name);
    frame
        .operand_stack()
        .expect("stack null")
        .push_ref(Some(name_obj));
}

pub fn desired_assertion_status0(frame: &mut Frame) {
    frame.operand_stack().expect("stack null").push_int(0);
}

pub fn for_name0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let name = vars.get_ref(0);
    let initialize = vars.get_boolean(1);

    let rust_name = java_str_to_rust_str(name.unwrap()).replace('.', "/");
    let class = frame.method().class();
    let loader = (*class).borrow().loader();
    let class = ClassLoader::load_class(loader, rust_name.as_str());
    let java_class = (*class).borrow().get_java_class();
    if initialize && !(*class).borrow().initialized() {
        let thread = frame.thread();
        frame.set_next_pc((*thread).borrow().get_pc());
        init_class(thread, class);
    } else {
        frame
            .operand_stack()
            .expect("stack null")
            .push_ref(java_class);
    }
}

pub fn is_interface(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().meta();
    frame
        .operand_stack()
        .expect("stack null")
        .push_boolean((*class.unwrap()).borrow().is_interface());
}

pub fn is_primitive(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().meta();
    frame
        .operand_stack()
        .expect("stack null")
        .push_boolean((*class.unwrap()).borrow().is_primitive());
}

pub fn get_modifiers(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().meta();
    frame
        .operand_stack()
        .expect("stack null")
        .push_int((*class.unwrap()).borrow().access_flags() as i32);
}

pub fn get_superclass(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().meta().unwrap();
    let super_class = (*class).borrow().super_class();
    let mut java_class: Option<Rc<RefCell<Object>>> = None;
    if super_class.is_some() {
        java_class = (*super_class.unwrap()).borrow().get_java_class();
    }
    frame
        .operand_stack()
        .expect("stack null")
        .push_ref(java_class);
}

pub fn get_interfaces0(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().meta().unwrap();
    let borrow = (*class).borrow();
    let interfaces = borrow.interfaces();
    let none = Vec::new();
    let class_arr = to_class_arr(
        (*class).borrow().loader(),
        interfaces.unwrap_or_else(|| &none),
    );
    frame
        .operand_stack()
        .expect("stack null")
        .push_ref(Some(boxed(class_arr)));
}

// []*Class => Class[]
fn to_class_arr(
    loader: Rc<RefCell<ClassLoader>>,
    classes: &Vec<Rc<RefCell<Class>>>,
) -> ArrayObject {
    let arr_len = classes.len();

    let class_arr_class = (*ClassLoader::load_class(loader, "java/lang/Class"))
        .borrow()
        .array_class();
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
fn to_byte_arr(loader: Rc<RefCell<ClassLoader>>, rbytes: Option<Vec<u8>>) -> Option<ArrayObject> {
    if rbytes.is_some() {
        let j_bytes: Vec<i8> = rbytes.unwrap().iter().map(|x| *x as i8).collect();
        return Some(ArrayObject::from_data(
            ClassLoader::load_class(loader, "[B"),
            Bytes(j_bytes),
        ));
    }
    return None;
}

pub fn is_array(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().meta();
    frame
        .operand_stack()
        .expect("stack null")
        .push_boolean((*class.unwrap()).borrow().is_array());
}

// public native Class<?> getComponentType();
// ()Ljava/lang/Class;
pub fn get_component_type(frame: &mut Frame) {
    let this = frame
        .local_vars()
        .expect("vars is none")
        .get_this()
        .unwrap();
    let class = (*this).borrow().meta().unwrap();
    let component_class = (*class).borrow().component_class();
    frame
        .operand_stack()
        .expect("stack null")
        .push_ref((*component_class).borrow().get_java_class());
}

// public native boolean isAssignableFrom(Class<?> cls);
// (Ljava/lang/Class;)Z
pub fn is_assignable_from(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let this = vars.get_this();
    let cls = vars.get_ref(1);

    let this_class = (*this.unwrap()).borrow().meta().unwrap();
    let cls_class = (*cls.unwrap()).borrow().meta().unwrap();
    let ok = (*this_class)
        .borrow()
        .is_assignable_from((*cls_class).borrow().deref());

    frame.operand_stack().expect("stack null").push_boolean(ok);
}

const _CONSTRUCTOR_CONSTRUCTOR_DESCRIPTOR: &str =
    "(Ljava/lang/Class;[Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B)V";

// private native Constructor<T>[] getDeclaredConstructors0(boolean publicOnly);
// (Z)[Ljava/lang/reflect/Constructor;
pub fn get_declared_constructors0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let class_obj = vars.get_this().unwrap();
    let public_only = vars.get_boolean(1);

    let class = (*class_obj).borrow().meta().unwrap();
    let constructors = (*class).borrow().get_constructors(public_only);
    let constructor_count = constructors.len();

    let class = frame.method().class();
    let class_loader = (*class).borrow().loader();
    let constructor_class =
        ClassLoader::load_class(class_loader.clone(), "java/lang/reflect/Constructor");

    let class_arr_class = (*constructor_class).borrow().array_class();
    let constructor_arr = Class::new_array(&class_arr_class, constructor_count);

    let boxed_arr = Some(boxed(constructor_arr));
    frame
        .operand_stack()
        .expect("stack null")
        .push_ref(boxed_arr.clone());

    if constructor_count > 0 {
        let thread = frame.thread();
        let arr = boxed_arr.unwrap();
        let mut temp = (*arr).borrow_mut();
        let constructor_objs = temp.mut_references();
        let constructor_init_method = Class::get_constructor(
            constructor_class.clone(),
            _CONSTRUCTOR_CONSTRUCTOR_DESCRIPTOR,
        );
        for i in 0..constructors.len() {
            let constructor = constructors[i].clone();
            let mut constructor_obj = Class::new_object(&constructor_class);
            constructor_obj.set_meta_data(Method(constructor.clone()));
            let object = Some(boxed(constructor_obj));
            constructor_objs[i] = object.clone();

            let mut ops = OperandStack::new(9).unwrap();
            ops.push_ref(object); // this
            ops.push_ref(Some(class_obj.clone())); // declaringClass
            let parameter_types = constructor.parameter_types().unwrap();
            ops.push_ref(Some(boxed(to_class_arr(
                class_loader.clone(),
                &parameter_types,
            )))); // parameterTypes
            let exception_types = constructor.exception_types().unwrap_or_else(|| Vec::new());
            ops.push_ref(Some(boxed(to_class_arr(
                class_loader.clone(),
                &exception_types,
            )))); // checkedExceptions
            ops.push_int(constructor.access_flags() as i32); // modifiers
            ops.push_int(0); // todo slot
            ops.push_ref(get_signature_str(
                class_loader.clone(),
                constructor.signature(),
            )); // signature
            let mut data: Vec<u8> = vec![0, 20];
            ops.push_ref(Some(boxed(
                to_byte_arr(class_loader.clone(), Some((data))).unwrap(),
            )));
            let mut data: Vec<u8> = vec![0, 20]; // annotations
            ops.push_ref(Some(boxed(
                to_byte_arr(class_loader.clone(), Some(data)).unwrap(),
            ))); // parameterAnnotations

            let shim_frame = Frame::new_shim_frame(thread.clone(), ops);
            (*thread).borrow_mut().push_frame(shim_frame);

            // init constructor_obj
            hack_invoke_method(thread.clone(), constructor_init_method.clone().unwrap());
        }
    }
}

const _FIELD_CONSTRUCTOR_DESCRIPTOR: &str =
    "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IILjava/lang/String;[B)V";

// private native Field[] getDeclaredFields0(boolean publicOnly);
// (Z)[Ljava/lang/reflect/Field;
pub fn get_declared_fields0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let class_obj = vars.get_this().unwrap();
    let public_only = vars.get_boolean(1);

    let class = (*class_obj).borrow().meta().unwrap();
    let fields = (*class).borrow().get_fields(public_only);
    let field_count = fields.len();

    let class = frame.method().class();
    let class_loader = (*class).borrow().loader();
    let field_class = ClassLoader::load_class(class_loader.clone(), "java/lang/reflect/Field");
    let field_arr_class = (*field_class).borrow().array_class();
    let field_arr = Class::new_array(&field_arr_class, field_count);

    let boxed_arr = Some(boxed(field_arr));
    frame
        .operand_stack()
        .expect("stack null")
        .push_ref(boxed_arr.clone());

    if field_count > 0 {
        let thread = frame.thread();
        let arr = boxed_arr.unwrap();
        let mut temp = (*arr).borrow_mut();
        let field_objs = temp.mut_references();
        let field_init_method =
            Class::get_constructor(field_class.clone(), _FIELD_CONSTRUCTOR_DESCRIPTOR);
        for i in 0..fields.len() {
            let field = fields[i].clone();
            let mut field_obj = Class::new_object(&field_class);
            field_obj.set_meta_data(Field(field.clone()));
            let object = Some(boxed(field_obj));
            field_objs[i] = object.clone();

            let mut ops = OperandStack::new(8).unwrap();
            ops.push_ref(object); // this
            ops.push_ref(Some(class_obj.clone())); // declaringClass
            ops.push_ref(Some(StringPool::java_string(
                class_loader.clone(),
                (*field).borrow().name().to_string(),
            ))); // name
            ops.push_ref((*(*field).borrow().r#type()).borrow().get_java_class()); // type
            ops.push_int((*field).borrow().access_flags() as i32); // modifiers
            ops.push_int((*field).borrow().slot_id() as i32); // slot
            ops.push_ref(get_signature_str(
                class_loader.clone(),
                (*field).borrow().signature(),
            )); // signature
            let mut data: Vec<u8> = vec![0, 20];

            ops.push_ref(Some(boxed(
                to_byte_arr(class_loader.clone(), Some(data)).unwrap(),
            ))); // annotations

            let shim_frame = Frame::new_shim_frame(thread.clone(), ops);
            (*thread).borrow_mut().push_frame(shim_frame);

            // init field_obj
            hack_invoke_method(thread.clone(), field_init_method.clone().unwrap());
        }
    }
}

fn get_signature_str(
    loader: Rc<RefCell<ClassLoader>>,
    signature: &str,
) -> Option<Rc<RefCell<Object>>> {
    if signature != "" {
        return Some(StringPool::java_string(loader, signature.to_string()));
    }
    return None;
}

const _METHOD_CONSTRUCTOR_DESCRIPTOR:&str =
    "(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B[B)V";

/// private native Method[] getDeclaredMethods0(boolean publicOnly);
/// (Z)[Ljava/lang/reflect/Method;
pub fn get_declared_methods0(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let class_obj = vars.get_this().unwrap();
    let public_only = vars.get_boolean(1);

    let class = (*class_obj).borrow().meta().unwrap();
    let methods = (*class).borrow().get_methods(public_only);
    let method_count = methods.len();

    let class = frame.method().class();
    let class_loader = (*class).borrow().loader();
    let method_class = ClassLoader::load_class(class_loader.clone(), "java/lang/reflect/Method");
    let method_arr_class = (*method_class).borrow().array_class();
    let method_arr = Class::new_array(&method_arr_class, method_count);

    let boxed_arr = Some(boxed(method_arr));
    frame
        .operand_stack()
        .expect("stack null")
        .push_ref(boxed_arr.clone());

    // create method objs
    if method_count > 0 {
        let thread = frame.thread();
        let arr = boxed_arr.unwrap();
        let mut temp = (*arr).borrow_mut();
        let method_objs = temp.mut_references();
        let method_constructor =
            Class::get_constructor(method_class.clone(), _METHOD_CONSTRUCTOR_DESCRIPTOR);
        for i in 0..method_count {
            let method = methods[i].clone();
            let mut method_obj = Class::new_object(&method_class);
            method_obj.set_meta_data(Method(method.clone()));
            let object = Some(boxed(method_obj));
            method_objs[i] = object.clone();

            let mut ops = OperandStack::new(8).unwrap();
            ops.push_ref(object); // this
            ops.push_ref(Some(class_obj.clone())); // declaringClass
            ops.push_ref(Some(StringPool::java_string(
                class_loader.clone(),
                method.name().to_string(),
            ))); // name
            let parameter_types = method.parameter_types().unwrap();
            ops.push_ref(Some(boxed(to_class_arr(
                class_loader.clone(),
                &parameter_types,
            )))); // parameterTypes
            ops.push_ref((*method.return_type()).borrow().get_java_class()); // returnType
            let exception_types = method.exception_types().unwrap_or_else(|| Vec::new());
            ops.push_ref(Some(boxed(to_class_arr(
                class_loader.clone(),
                &exception_types,
            )))); // checkedExceptions
            ops.push_int(method.access_flags() as i32); // modifiers
            ops.push_int(0); // todo: slot
            ops.push_ref(get_signature_str(class_loader.clone(), method.signature())); // signature
            let mut data: Vec<u8> = vec![0, 20];
            ops.push_ref(Some(boxed(
                to_byte_arr(class_loader.clone(), Some(data)).unwrap(),
            ))); // annotations
                 //            ops.push_ref(toByteArr(classLoader, method.ParameterAnnotationData())) // parameterAnnotations
            ops.push_ref(None);
            let mut data: Vec<u8> = vec![0, 20];
            ops.push_ref(None);
            //            ops.push_ref(toByteArr(classLoader, method.AnnotationDefaultData()))   // annotationDefault

            let shim_frame = Frame::new_shim_frame(thread.clone(), ops);
            (*thread).borrow_mut().push_frame(shim_frame);

            // init methodObj
            hack_invoke_method(thread.clone(), method);
        }
    }
}
