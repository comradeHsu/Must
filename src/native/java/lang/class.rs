use crate::class_loader::app_class_loader::ClassLoader;
use crate::instructions::base::class_init_logic::init_class;

use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{JavaCall, ReturnType};
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::oops::array_object::ArrayObject;
use crate::oops::class::Class;
use crate::oops::object::DataType::Bytes;
use crate::oops::object::MetaData::{Field, Method};
use crate::oops::object::Object;
use crate::oops::string_pool::StringPool;
use crate::runtime::frame::Frame;

use crate::runtime::thread::JavaThread;
use crate::utils::{java_str_to_rust_str};
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

pub fn get_primitive_class(frame: &Frame) {
    let name_obj = frame.get_this().unwrap();
    let target = java_str_to_rust_str(name_obj);

    let class = Jvm::boot_class_loader()
        .find_or_create(target.as_str())
        .unwrap();
    let java_class = class.get_java_class();
    frame.push_ref(java_class);
}

pub fn get_name0(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.meta();
    let name = class.java_name();
    let name_obj = StringPool::java_string(name);
    frame.push_ref(Some(name_obj));
}

pub fn desired_assertion_status0(frame: &Frame) {
    frame.push_int(0);
}

pub fn for_name0(frame: &Frame) {
    let (name, initialize, java_loader) = frame.local_vars_get(|vars| {
        let name = vars.get_ref(0);
        let initialize = vars.get_boolean(1);
        let java_loader = vars.get_ref(2);
        (name, initialize, java_loader)
    });

    let rust_name = java_str_to_rust_str(name.unwrap()).replace('.', "/");

    let class = ClassLoader::load_class(java_loader, rust_name.as_str());
    let java_class = class.get_java_class();
    if initialize && !class.initialized() {
        frame.set_next_pc(JavaThread::current().get_pc());
        init_class(class);
    } else {
        frame.push_ref(java_class);
    }
}

pub fn is_interface(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.meta();
    frame.push_boolean(class.is_interface());
}

pub fn is_primitive(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.meta();
    frame.push_boolean(class.is_primitive());
}

pub fn get_modifiers(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.meta();
    frame.push_int(class.access_flags() as i32);
}

pub fn get_superclass(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.meta();
    let super_class = class.super_class();
    let mut java_class: Option<Object> = None;
    if super_class.is_some() {
        java_class = super_class.unwrap().get_java_class();
    }
    frame.push_ref(java_class);
}

pub fn get_interfaces0(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.meta();
    let none = Vec::new();
    let class_arr = class.interfaces_with(|interfaces|{
        to_class_arr(interfaces.unwrap_or_else(|| &none))
    });
    frame.push_ref(Some(class_arr));
}

// []*Class => Class[]
fn to_class_arr(classes: &Vec<Class>) -> ArrayObject {
    let arr_len = classes.len();
    let bootstrap_loader = Jvm::boot_class_loader();
    let class_arr_class = bootstrap_loader.find_or_create("java/lang/Class").unwrap()
        .array_class();
    let class_arr = Class::new_array(&class_arr_class, arr_len);

    if arr_len > 0 {
        class_arr.mut_references(|class_objs| {
            for i in 0..arr_len {
                class_objs[i] = classes[i].get_java_class();
            }
        });
    }

    return class_arr;
}

// []byte => byte[]
fn to_byte_arr(rbytes: Option<Vec<u8>>) -> Option<ArrayObject> {
    if rbytes.is_some() {
        let j_bytes: Vec<i8> = rbytes.unwrap().iter().map(|x| *x as i8).collect();
        let boot_loader = Jvm::boot_class_loader();
        return Some(ArrayObject::from_data(
            &boot_loader.find_or_create("[B").unwrap(),
            Bytes(j_bytes),
        ));
    }
    return None;
}

pub fn is_array(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.meta();
    frame.push_boolean(class.is_array());
}

// public native Class<?> getComponentType();
// ()Ljava/lang/Class;
pub fn get_component_type(frame: &Frame) {
    let this = frame.get_this().unwrap();
    let class = this.meta();
    let component_class = class.component_class();
    frame.push_ref(component_class.get_java_class());
}

// public native boolean isAssignableFrom(Class<?> cls);
// (Ljava/lang/Class;)Z
pub fn is_assignable_from(frame: &Frame) {
    let (this, cls) = frame.local_vars_get(|vars| {
        let this = vars.get_this();
        let cls = vars.get_ref(1);
        (this, cls)
    });

    let this_class = this.unwrap().meta();
    let cls_class = cls.unwrap().meta();
    let ok = this_class
        .is_assignable_from(&cls_class);

    frame.push_boolean(ok);
}

const _CONSTRUCTOR_CONSTRUCTOR_DESCRIPTOR: &str =
    "(Ljava/lang/Class;[Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B)V";

// private native Constructor<T>[] getDeclaredConstructors0(boolean publicOnly);
// (Z)[Ljava/lang/reflect/Constructor;
pub fn get_declared_constructors0(frame: &Frame) {
    let (class_obj, public_only) = frame.local_vars_get(|vars| {
        let class_obj = vars.get_this().unwrap();
        let public_only = vars.get_boolean(1);
        (class_obj, public_only)
    });

    let class = class_obj.meta();
    let constructors = class.get_constructors(public_only);
    let constructor_count = constructors.len();

    let constructor_class = Jvm::boot_class_loader()
        .find_or_create("java/lang/reflect/Constructor")
        .unwrap();

    let class_arr_class = constructor_class.array_class();
    let constructor_arr = Class::new_array(&class_arr_class, constructor_count);

    let boxed_arr = Some(constructor_arr);
    frame.push_ref(boxed_arr.clone());

    if constructor_count > 0 {
        let _thread = JavaThread::current();
        let arr = boxed_arr.unwrap();

        let constructor_init_method = constructor_class.get_constructor(
            _CONSTRUCTOR_CONSTRUCTOR_DESCRIPTOR,
        );
        arr.mut_references(|constructor_objs| {
            for i in 0..constructors.len() {
                let constructor = &constructors[i];
                let constructor_obj = Class::new_object(&constructor_class);
                constructor_obj.set_meta_data(Method(constructor.clone()));
                let object = Some(constructor_obj);
                constructor_objs[i] = object.clone();

                let parameter_types = constructor.parameter_types().unwrap();
                let exception_types = constructor.exception_types().unwrap_or_else(|| Vec::new());
                let data: Vec<u8> = vec![0, 20];
                let pas: Vec<u8> = vec![0, 20];
                // init constructor_obj
                let parameters = Parameters::with_parameters(vec![
                    Parameter::Object(object),                                     // this
                    Parameter::Object(Some(class_obj.clone())),                    // declaringClass
                    Parameter::Object(Some(to_class_arr(&parameter_types))),       // parameterTypes
                    Parameter::Object(Some(to_class_arr(&exception_types))), // checkedExceptions
                    Parameter::Int(constructor.access_flags() as i32),       // modifiers
                    Parameter::Int(0),                                       // todo slot
                    Parameter::Object(get_signature_str(constructor.signature())), // signature
                    Parameter::Object(Some(to_byte_arr(Some(data)).unwrap())), // annotations
                    Parameter::Object(Some(to_byte_arr(Some(pas)).unwrap())), // parameterAnnotations
                ]);
                JavaCall::invoke(
                    constructor_init_method.clone().unwrap(),
                    Some(parameters),
                    ReturnType::Void,
                );
            }
        });
    }
}

const _FIELD_CONSTRUCTOR_DESCRIPTOR: &str =
    "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IILjava/lang/String;[B)V";

// private native Field[] getDeclaredFields0(boolean publicOnly);
// (Z)[Ljava/lang/reflect/Field;
pub fn get_declared_fields0(frame: &Frame) {
    let (class_obj, public_only) = frame.local_vars_get(|vars| {
        let class_obj = vars.get_this().unwrap();
        let public_only = vars.get_boolean(1);
        (class_obj, public_only)
    });

    let class = class_obj.meta();
    let fields = class.get_fields(public_only);
    let field_count = fields.len();

    let field_class = Jvm::boot_class_loader()
        .find_or_create("java/lang/reflect/Field")
        .unwrap();
    let field_arr_class = field_class.array_class();
    let field_arr = Class::new_array(&field_arr_class, field_count);

    let boxed_arr = Some(field_arr);
    frame.push_ref(boxed_arr.clone());

    if field_count > 0 {
        let arr = boxed_arr.unwrap();
        let field_init_method =
            field_class.get_constructor(_FIELD_CONSTRUCTOR_DESCRIPTOR);
        arr.mut_references(|field_objs| {
            for i in 0..fields.len() {
                let field = fields[i].clone();
                let field_obj = Class::new_object(&field_class);
                field_obj.set_meta_data(Field(field.clone()));
                let object = Some(field_obj);
                field_objs[i] = object.clone();

                let data: Vec<u8> = vec![0, 20];
                // init field_obj
                let parameters = Parameters::with_parameters(vec![
                    Parameter::Object(object.clone()),
                    ///this
                    Parameter::Object(Some(class_obj.clone())), // declaringClass
                    Parameter::Object(Some(StringPool::java_string(
                        field.name().to_string(),
                    ))), // name
                    Parameter::Object(field.r#type().get_java_class()), // type
                    Parameter::Int(field.access_flags() as i32), // modifiers
                    Parameter::Int(field.slot_id() as i32),      // slot
                    Parameter::Object(get_signature_str(field.signature())), // signature
                    Parameter::Object(Some(to_byte_arr(Some(data)).unwrap())), // annotations
                ]);
                JavaCall::invoke(
                    field_init_method.clone().unwrap(),
                    Some(parameters),
                    ReturnType::Void,
                );
            }
        });
    }
}

fn get_signature_str(signature: &str) -> Option<Object> {
    if signature != "" {
        return Some(StringPool::java_string(signature.to_string()));
    }
    return None;
}

const _METHOD_CONSTRUCTOR_DESCRIPTOR:&str =
    "(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B[B)V";

// private native Method[] getDeclaredMethods0(boolean publicOnly);
// (Z)[Ljava/lang/reflect/Method;
pub fn get_declared_methods0(frame: &Frame) {
    let (class_obj, public_only) = frame.local_vars_get(|vars| {
        let class_obj = vars.get_this().unwrap();
        let public_only = vars.get_boolean(1);
        (class_obj, public_only)
    });

    let class = class_obj.meta();
    let methods = class.get_methods(public_only);
    let method_count = methods.len();

    let method_class = Jvm::boot_class_loader()
        .find_or_create("java/lang/reflect/Method")
        .unwrap();
    let method_arr_class = method_class.array_class();
    let method_arr = Class::new_array(&method_arr_class, method_count);

    let boxed_arr = Some(method_arr);
    frame.push_ref(boxed_arr.clone());

    // create method objs
    if method_count > 0 {
        let arr = boxed_arr.unwrap();
        let _method_constructor =
            method_class.get_constructor(_METHOD_CONSTRUCTOR_DESCRIPTOR);
        arr.mut_references(|method_objs| {
            for i in 0..method_count {
                let method = methods[i].clone();
                let method_obj = Class::new_object(&method_class);
                method_obj.set_meta_data(Method(method.clone()));
                let object = Some(method_obj);
                method_objs[i] = object.clone();

                let parameter_types = method.parameter_types().unwrap();
                let exception_types = method.exception_types().unwrap_or_else(|| Vec::new());
                let data: Vec<u8> = vec![0, 20];

                /// init methodObj
                let parameters = Parameters::with_parameters(vec![
                    Parameter::Object(object),                  // this
                    Parameter::Object(Some(class_obj.clone())), // declaringClass
                    Parameter::Object(Some(StringPool::java_string(method.name().to_string()))), // name
                    Parameter::Object(Some(to_class_arr(&parameter_types))), // parameterTypes
                    Parameter::Object(method.return_type().get_java_class()), // returnType
                    Parameter::Object(Some(to_class_arr(&exception_types))), // checkedExceptions
                    Parameter::Int(method.access_flags() as i32),            // modifiers
                    Parameter::Int(0),                                       // todo slot
                    Parameter::Object(get_signature_str(method.signature())), // signature
                    Parameter::Object(Some(to_byte_arr(Some(data)).unwrap())), // annotations
                    Parameter::Object(None),                                 // parameterAnnotations
                    Parameter::Object(None),                                 // annotationDefault
                ]);
                JavaCall::invoke(method, Some(parameters), ReturnType::Void);
            }
        });
    }
}
