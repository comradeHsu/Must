use crate::class_loader::app_class_loader::ClassLoader;
use crate::native::java::lang::throwable::StackTraceElement;
use crate::oops::class::Class;
use crate::oops::field::Field;
use crate::oops::method::Method;
use crate::oops::object::DataType::{StandardObject};
use crate::oops::object::MetaData::Null;
use crate::oops::slots::Slots;
use crate::utils::boxed;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs::File;
use std::ops::Deref;
use std::rc::Rc;
use crate::runtime::thread::JavaThread;

#[derive(Debug, Clone)]
pub struct Data {
    pub class: Rc<RefCell<Class>>,
    pub data: DataType,
    pub meta_data: MetaData,
}

impl Data {
    pub fn new(class: Rc<RefCell<Class>>) -> Data {
        let count = (*class).borrow().instance_slot_count();
        return Data {
            class: class.clone(),
            data: StandardObject(Some(Slots::with_capacity(count as usize))),
            meta_data: MetaData::Null,
        };
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub data: Rc<RefCell<Data>>,
}

impl Object {
    pub fn new(class: Rc<RefCell<Class>>) -> Object {
        return Object {
            data: boxed(Data::new(class)),
        };
    }

    #[inline]
    pub fn class(&self) -> Rc<RefCell<Class>> {
        return (*self.data).borrow().class.clone();
    }

    #[inline]
    pub fn meta(&self) -> Rc<RefCell<Class>> {
        match &(*self.data).borrow().meta_data {
            MetaData::MetaClass(class) => class.clone(),
            _ => panic!(),
        }
    }

    #[inline]
    pub fn set_meta(&self, meta: Rc<RefCell<Class>>) {
        (*self.data).borrow_mut().meta_data = MetaData::MetaClass(meta);
    }

    #[inline]
    pub fn trace<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<StackTraceElement>) -> R,
    {
        match &(*self.data).borrow().meta_data {
            MetaData::StackTrace(elements) => func(elements),
            _ => panic!("this object isn't exception"),
        }
    }

    #[inline]
    pub fn set_trace(&self, elements: Vec<StackTraceElement>) {
        (*self.data).borrow_mut().meta_data = MetaData::StackTrace(elements);
    }

    #[inline]
    pub fn set_meta_data(&self, data: MetaData) {
        (*self.data).borrow_mut().meta_data = data;
    }

    pub fn mut_fields_with<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Slots) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
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
        match &(*self.data).borrow().data {
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
        func(&mut (*self.data).borrow_mut().data)
    }

    #[inline]
    pub fn data_with<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&DataType) -> R,
    {
        func(&(*self.data).borrow_mut().data)
    }

    #[inline]
    pub fn is_class_object(&self) -> bool {
        if let MetaData::MetaClass(_) = (*self.data).borrow().meta_data {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_array_object(&self) -> bool {
        if let DataType::StandardObject(_) = (*self.data).borrow().data {
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn is_instance_of(&self, class: Rc<RefCell<Class>>) -> bool {
        return (*class)
            .borrow()
            .is_assignable_from(self.class().as_ref().borrow().borrow());
    }

    #[inline]
    pub fn file(&self) -> Rc<RefCell<File>> {
        match &(*self.data).borrow().meta_data {
            MetaData::File(file) => file.clone(),
            _ => panic!("The Object isn't file"),
        }
    }

    #[inline]
    pub fn set_file(&self, file: File) {
        self.set_meta_data(MetaData::File(boxed(file)));
    }

    #[inline]
    pub fn get_class_loader(&self) -> Rc<RefCell<ClassLoader>> {
        match &(*self.data).borrow().meta_data {
            MetaData::ClassLoader(loader) => loader.clone(),
            _ => panic!("The Object isn't class loader"),
        }
    }

    pub fn set_ref_var(&self, name: &str, descriptor: &str, reference: Object) {
        let field = Class::get_field(Some(self.class()), name, descriptor, false);
        self.mut_fields_with(|slots| {
            slots.set_ref((*field.unwrap()).borrow().slot_id(), Some(reference));
        })
    }

    pub fn get_ref_var(&self, name: &str, descriptor: &str) -> Option<Object> {
        let field = Class::get_field(Some(self.class()), name, descriptor, false);
        let field = field.unwrap();
        self.fields_with(|slots| {
            return slots.get_ref((*field).borrow().slot_id());
        })
    }

    pub fn set_int_var(&self, name: &str, descriptor: &str, val: i32) {
        let field = Class::get_field(Some(self.class()), name, descriptor, false);
        self.mut_fields_with(|slots| {
            slots.set_int((*field.unwrap()).borrow().slot_id(), val);
        })
    }

    pub fn get_int_var(&self, name: &str, descriptor: &str) -> i32 {
        let field = Class::get_field(Some(self.class()), name, descriptor, false);
        self.fields_with(|slots| {
            return slots.get_int((*field.unwrap()).borrow().slot_id());
        })
    }

    pub fn get_long_var(&self, name: &str, descriptor: &str) -> i64 {
        let field = Class::get_field(Some(self.class()), name, descriptor, false);
        self.fields_with(|slots| {
            return slots.get_long((*field.unwrap()).borrow().slot_id());
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
        (*self.data).borrow().meta_data.clone()
    }

    pub fn deep_clone(&self) -> Self {
        return Object {
            data: Rc::new(RefCell::new((*self.data).borrow().clone())),
        };
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let l = (*self.data).borrow().deref() as *const Data;
        let r = (*other.data).borrow().deref() as *const Data;
        if l == r {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum MetaData {
    Null,
    Field(Rc<RefCell<Field>>),
    Method(Rc<Method>),
    ClassLoader(Rc<RefCell<ClassLoader>>),
    File(Rc<RefCell<File>>),
    MetaClass(Rc<RefCell<Class>>),
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
    pub fn method(&self) -> Rc<Method> {
        match self {
            MetaData::Method(method) => method.clone(),
            _ => panic!("The MetaData not method"),
        }
    }

    #[inline]
    pub fn get_class_loader(&self) -> Rc<RefCell<ClassLoader>> {
        match self {
            MetaData::ClassLoader(loader) => loader.clone(),
            _ => panic!("The MetaData not ClassLoader"),
        }
    }
}

//pub type ArrayObject = Object;
//
//impl ArrayObject {
//    #[inline]
//    pub fn from_data(class: Rc<RefCell<Class>>, data: DataType) -> ArrayObject {
//        return Object {
//            data: Rc::new(RefCell::new(Data {
//                class,
//                data,
//                meta_data: MetaData::Null
//            }))
//        };
//    }
//
//    pub fn bytes<R,F>(&self,func:F) -> R
//        where
//            F: FnOnce(&Vec<i8>) -> R
//    {
//        match &(*self.data).borrow().data {
//            Bytes(array) => func(array),
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn mut_bytes<R,F,T>(&self,func:F) -> R
//        where
//            F: FnOnce(&mut Vec<T>) -> R
//    {
//        match &mut (*self.data).borrow().data {
//            Bytes(array) => func(array),
//            Shorts(array) => func(array),
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn shorts(&self) -> &Vec<i16> {
//        match &self.data {
//            Shorts(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn mut_shorts(&mut self) -> &mut Vec<i16> {
//        match &mut self.data {
//            Shorts(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn ints(&self) -> &Vec<i32> {
//        match &self.data {
//            Ints(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn mut_ints(&mut self) -> &mut Vec<i32> {
//        match &mut self.data {
//            Ints(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn longs(&self) -> &Vec<i64> {
//        match &self.data {
//            Longs(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn mut_longs(&mut self) -> &mut Vec<i64> {
//        match &mut self.data {
//            Longs(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn get_long_by_index(&self, index: usize) -> i64 {
//        match &self.data {
//            Longs(array) => {
//                let value = array.get(index).map_or_else(|| 0, |x| *x);
//                return value;
//            }
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn set_long_by_index(&mut self, index: usize, value: i64) {
//        match &mut self.data {
//            Longs(array) => {
//                array[index] = value;
//            }
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn chars(&self) -> &Vec<u16> {
//        match &self.data {
//            Chars(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn mut_chars(&mut self) -> &mut Vec<u16> {
//        match &mut self.data {
//            Chars(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn floats(&self) -> &Vec<f32> {
//        match &self.data {
//            Floats(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn mut_floats(&mut self) -> &mut Vec<f32> {
//        match &mut self.data {
//            Floats(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn doubles(&self) -> &Vec<f64> {
//        match &self.data {
//            Doubles(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn mut_doubles(&mut self) -> &mut Vec<f64> {
//        match &mut self.data {
//            Doubles(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn references(&self) -> &Vec<Option<Rc<RefCell<Object>>>> {
//        match &self.data {
//            References(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn mut_references(&mut self) -> &mut Vec<Option<Rc<RefCell<Object>>>> {
//        match &mut self.data {
//            References(array) => array,
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn get_references_by_index(&self, index: usize) -> Option<Rc<RefCell<Object>>> {
//        match &self.data {
//            References(array) => {
//                let reference = array.get(index).map_or_else(|| None, |x| x.clone());
//                return reference;
//            }
//            _ => panic!("The object type is error"),
//        }
//    }
//
//    pub fn array_length(&self) -> usize {
//        match &self.data {
//            Bytes(array) => array.len(),
//            Shorts(array) => array.len(),
//            Ints(array) => array.len(),
//            Longs(array) => array.len(),
//            Chars(array) => array.len(),
//            Floats(array) => array.len(),
//            Doubles(array) => array.len(),
//            References(array) => array.len(),
//            _ => panic!("The object isn't array"),
//        }
//    }
//
//    pub fn array_copy(
//        src: Rc<RefCell<Object>>,
//        dst: Rc<RefCell<Object>>,
//        src_pos: usize,
//        dst_pos: usize,
//        length: usize,
//    ) {
//        if src == dst {
//            ArrayObject::array_copy_from_same_object(src, src_pos, dst_pos, length);
//            return;
//        }
//        let mut src_borrow = (*src).borrow();
//        let mut dst_borrow = (*dst).borrow_mut();
//        match (src_borrow.data(), dst_borrow.mut_data()) {
//            (Bytes(s), Bytes(d)) => {
//                let s_slice = &s[src_pos..(src_pos + length)];
//                let d_slice = &mut d[dst_pos..(dst_pos + length)];
//                d_slice.copy_from_slice(s_slice);
//            }
//            (Shorts(s), Shorts(d)) => {
//                let s_slice = &s[src_pos..(src_pos + length)];
//                let d_slice = &mut d[dst_pos..(dst_pos + length)];
//                d_slice.copy_from_slice(s_slice);
//            }
//            (Ints(s), Ints(d)) => {
//                let s_slice = &s[src_pos..(src_pos + length)];
//                let d_slice = &mut d[dst_pos..(dst_pos + length)];
//                d_slice.copy_from_slice(s_slice);
//            }
//            (Longs(s), Longs(d)) => {
//                let s_slice = &s[src_pos..(src_pos + length)];
//                let d_slice = &mut d[dst_pos..(dst_pos + length)];
//                d_slice.copy_from_slice(s_slice);
//            }
//            (Chars(s), Chars(d)) => {
//                let s_slice = &s[src_pos..(src_pos + length)];
//                let d_slice = &mut d[dst_pos..(dst_pos + length)];
//                d_slice.copy_from_slice(s_slice);
//            }
//            (Floats(s), Floats(d)) => {
//                let s_slice = &s[src_pos..(src_pos + length)];
//                let d_slice = &mut d[dst_pos..(dst_pos + length)];
//                d_slice.copy_from_slice(s_slice);
//            }
//            (Doubles(s), Doubles(d)) => {
//                let s_slice = &s[src_pos..(src_pos + length)];
//                let d_slice = &mut d[dst_pos..(dst_pos + length)];
//                d_slice.copy_from_slice(s_slice);
//            }
//            (References(s), References(d)) => {
//                let s_slice = &s[src_pos..(src_pos + length)];
//                let d_slice = &mut d[dst_pos..(dst_pos + length)];
//                d_slice.clone_from_slice(s_slice);
//            }
//            _ => panic!("The object isn't array"),
//        }
//    }
//
//    fn array_copy_from_same_object(
//        object: Rc<RefCell<Object>>,
//        src_pos: usize,
//        dst_pos: usize,
//        length: usize,
//    ) {
//        let mut dst_borrow = (*object).borrow_mut();
//        match dst_borrow.mut_data() {
//            Bytes(s) => {
//                s.copy_within(src_pos..(src_pos + length), dst_pos);
//            }
//            Shorts(s) => {
//                s.copy_within(src_pos..(src_pos + length), dst_pos);
//            }
//            Ints(s) => {
//                s.copy_within(src_pos..(src_pos + length), dst_pos);
//            }
//            Longs(s) => {
//                s.copy_within(src_pos..(src_pos + length), dst_pos);
//            }
//            Chars(s) => {
//                s.copy_within(src_pos..(src_pos + length), dst_pos);
//            }
//            Floats(s) => {
//                s.copy_within(src_pos..(src_pos + length), dst_pos);
//            }
//            Doubles(s) => {
//                s.copy_within(src_pos..(src_pos + length), dst_pos);
//            }
//            References(s) => {
//                for i in 0..length {
//                    s[dst_pos + i] = s[src_pos + i].clone();
//                }
//            }
//            _ => panic!("The object isn't array"),
//        }
//    }
//}
