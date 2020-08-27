use crate::class_loader::app_class_loader::ClassLoader;
use crate::native::java::lang::throwable::StackTraceElement;
use crate::oops::class::Class;
use crate::oops::field::Field;
use crate::oops::method::Method;
use crate::oops::object::DataType::{StandardObject};
use crate::oops::object::MetaData::Null;
use crate::oops::slots::Slots;
use std::fs::File;
use crate::runtime::thread::JavaThread;
use std::sync::{Arc, RwLock, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::boxed;
use std::ops::Deref;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Data {
    pub class: Class,
    pub data: DataType,
    pub meta_data: MetaData,
}

impl Data {
    pub fn new(class: &Class) -> Data {
        let count = class.instance_slot_count();
        return Data {
            class: class.clone(),
            data: StandardObject(Some(Slots::with_capacity(count as usize))),
            meta_data: MetaData::Null,
        };
    }
}

#[derive(Clone)]
pub struct Object {
    pub data: Arc<RwLock<Data>>,
}

impl Object {
    pub fn new(class: &Class) -> Object {
        return Object {
            data: Arc::new(RwLock::new(Data::new(class))),
        };
    }

    #[inline]
    pub fn class(&self) -> Class {
        let data = self.data.read().unwrap();
        return data.class.clone();
    }

    #[inline]
    pub fn meta(&self) -> Class {
        let data = self.data.read().unwrap();
        match & data.meta_data {
            MetaData::MetaClass(class) => class.clone(),
            _ => panic!(),
        }
    }

    #[inline]
    pub fn set_meta(&self, meta: Class) {
        let mut data = self.data.write().unwrap();
        data.meta_data = MetaData::MetaClass(meta);
    }

    #[inline]
    pub fn trace<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<StackTraceElement>) -> R,
    {
        let data = self.data.read().unwrap();
        match & data.meta_data {
            MetaData::StackTrace(elements) => func(elements),
            _ => panic!("this object isn't exception"),
        }
    }

    #[inline]
    pub fn set_trace(&self, elements: Vec<StackTraceElement>) {
        let mut data = self.data.write().unwrap();
        data.meta_data = MetaData::StackTrace(elements);
    }

    #[inline]
    pub fn set_meta_data(&self, data: MetaData) {
        let mut raw = self.data.write().unwrap();
        raw.meta_data = data;
    }

    pub fn mut_fields_with<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Slots) -> R,
    {
        let mut data = self.data.write().unwrap();
        match &mut data.data {
            StandardObject(data) => {
                let slots = data.as_mut().expect("The Object haven't member variable");
                func(slots)
            }
            _ => panic!("The Object is array"),
        }
    }

    pub fn fields_with<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Slots) -> R,
    {
        let data = self.data.read().unwrap();
        match &data.data {
            StandardObject(data) => {
                let slots = data.as_ref().expect("The Object haven't member variable");
                func(slots)
            }
            _ => panic!("The Object is array"),
        }
    }

    #[inline]
    pub fn mut_data_with<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut DataType) -> R,
    {
        let mut data = self.data.write().unwrap();
        func(&mut data.data)
    }

    #[inline]
    pub fn data_with<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&DataType) -> R,
    {
        let data = self.data.read().unwrap();
        func(&data.data)
    }

    #[inline]
    pub fn is_class_object(&self) -> bool {
        let data = self.data.read().unwrap();
        if let MetaData::MetaClass(_) = data.meta_data {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_array_object(&self) -> bool {
        let data = self.data.read().unwrap();
        if let DataType::StandardObject(_) = data.data {
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn is_instance_of(&self, class: &Class) -> bool {
        return class
            .is_assignable_from(&self.class());
    }

    #[inline]
    pub fn file(&self) -> Arc<Mutex<File>> {
        let data = self.data.read().unwrap();
        match &data.meta_data {
            MetaData::File(file) => file.clone(),
            _ => panic!("The Object isn't file"),
        }
    }

    #[inline]
    pub fn set_file(&self, file: File) {
        self.set_meta_data(MetaData::File(Arc::new(Mutex::new(file))));
    }

    #[inline]
    pub fn get_class_loader(&self) -> ClassLoader {
        let data = self.data.read().unwrap();
        match &data.meta_data {
            MetaData::ClassLoader(loader) => loader.clone(),
            _ => panic!("The Object isn't class loader"),
        }
    }

    pub fn set_ref_var(&self, name: &str, descriptor: &str, reference: Object) {
        let field = self.class().get_field(name, descriptor, false);
        self.mut_fields_with(|slots| {
            slots.set_ref(field.unwrap().slot_id(), Some(reference));
        })
    }

    pub fn get_ref_var(&self, name: &str, descriptor: &str) -> Option<Object> {
        let field = self.class().get_field(name, descriptor, false);
        let field = field.unwrap();
        self.fields_with(|slots| {
            return slots.get_ref(field.slot_id());
        })
    }

    pub fn set_int_var(&self, name: &str, descriptor: &str, val: i32) {
        let field = self.class().get_field(name, descriptor, false);
        self.mut_fields_with(|slots| {
            slots.set_int(field.unwrap().slot_id(), val);
        })
    }

    pub fn get_int_var(&self, name: &str, descriptor: &str) -> i32 {
        let field = self.class().get_field(name, descriptor, false);
        self.fields_with(|slots| {
            return slots.get_int(field.unwrap().slot_id());
        })
    }

    pub fn get_long_var(&self, name: &str, descriptor: &str) -> i64 {
        let field = self.class().get_field(name, descriptor, false);
        self.fields_with(|slots| {
            return slots.get_long(field.unwrap().slot_id());
        })
    }

    pub fn get_ref_var_by_slot_id(&self, slot_id: usize) -> Option<Object> {
        self.fields_with(|slots| {
            return slots.get_ref(slot_id);
        })
    }

    #[inline]
    pub fn get_long_var_by_slot_id(&self, slot_id: usize) -> i64 {
        self.fields_with(|slots| {
            return slots.get_long(slot_id);
        })
    }

    #[inline]
    pub fn set_long_var_by_slot_id(&self, slot_id: usize, value: i64) {
        self.mut_fields_with(|slots| {
            return slots.set_long(slot_id, value);
        })
    }

    #[inline]
    pub fn meta_data(&self) -> MetaData {
        let data = self.data.read().unwrap();
        data.meta_data.clone()
    }

    #[inline]
    pub fn set_thread(&self, thread: JavaThread) {
        self.set_meta_data(MetaData::Thread(thread));
    }

    pub fn deep_clone(&self) -> Self {
        let data = self.data.read().unwrap();
        return Object {
            data: Arc::new(RwLock::new(data.clone())),
        };
    }
    /// support for java object hashCode method
    pub fn hash_code(&self) -> i32 {
        let data = self.data.read().unwrap();
        let ptr = data.deref() as *const Data;
        return ptr as usize as i32;
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let data = self.data.read().unwrap();
        let other_data = other.data.read().unwrap();
        let l = data.deref() as *const Data;
        let r = other_data.deref() as *const Data;
        if l == r {
            return true;
        }
        return false;
    }
}


#[derive(Clone)]
pub enum DataType {
    StandardObject(Option<Slots>),
    Bytes(Vec<i8>),
    Shorts(Vec<i16>),
    Ints(Vec<i32>),
    Longs(Vec<i64>),
    Chars(Vec<u16>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
    References(Vec<Option<Object>>),
}

#[derive(Clone)]
pub enum MetaData {
    Null,
    Field(Field),
    Method(Method),
    ClassLoader(ClassLoader),
    File(Arc<Mutex<File>>),
    MetaClass(Class),
    StackTrace(Vec<StackTraceElement>),
    Thread(JavaThread)
}

impl MetaData {
    #[inline]
    pub fn is_null(&self) -> bool {
        match self {
            Null => true,
            _ => false,
        }
    }

    #[inline]
    pub fn not_null(&self) -> bool {
        match self {
            Null => false,
            _ => true,
        }
    }

    #[inline]
    pub fn method(&self) -> Method {
        match self {
            MetaData::Method(method) => method.clone(),
            _ => panic!("The MetaData not method"),
        }
    }

    #[inline]
    pub fn get_class_loader(&self) -> ClassLoader {
        match self {
            MetaData::ClassLoader(loader) => loader.clone(),
            _ => panic!("The MetaData not ClassLoader"),
        }
    }

    #[inline]
    pub fn thread(&self) -> Option<JavaThread> {
        match self {
            MetaData::Thread(thread) => Some(thread.clone()),
            _ => None
        }
    }
}