use crate::native::java::lang::throwable::StackTraceElement;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::object::DataType::StandardObject;
use crate::runtime_data_area::heap::object::MetaData::Null;
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::slot::Slot;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime_data_area::heap::class_loader::ClassLoader;

#[derive(Debug, Clone)]
pub struct Object {
    pub class: Rc<RefCell<Class>>,
    pub data: DataType,
    pub meta: Option<Rc<RefCell<Class>>>,
    pub trace: Option<Vec<StackTraceElement>>,
    pub meta_data: MetaData,
}

impl Object {
    pub fn new(class: Rc<RefCell<Class>>) -> Object {
        let count = (*class).borrow().instance_slot_count();
        return Object {
            class: class.clone(),
            data: StandardObject(Some(Slots::with_capacity(count as usize))),
            meta: None,
            trace: None,
            meta_data: MetaData::Null,
        };
    }

    #[inline]
    pub fn class(&self) -> Rc<RefCell<Class>> {
        return self.class.clone();
    }

    #[inline]
    pub fn meta(&self) -> Option<Rc<RefCell<Class>>> {
        return self.meta.clone();
    }

    #[inline]
    pub fn set_meta(&mut self, meta: Rc<RefCell<Class>>) {
        self.meta = Some(meta);
    }

    #[inline]
    pub fn trace(&self) -> Option<&Vec<StackTraceElement>> {
        return self.trace.as_ref();
    }

    #[inline]
    pub fn set_trace(&mut self, eles: Vec<StackTraceElement>) {
        self.trace = Some(eles);
    }

    #[inline]
    pub fn set_meta_data(&mut self, data: MetaData) {
        self.meta_data = data;
    }

    #[inline]
    pub fn fields(&mut self) -> &mut Slots {
        let fields = &mut self.data;
        match fields {
            StandardObject(data) => data.as_mut().unwrap(),
            _ => panic!("The Object is array"),
        }
    }

    #[inline]
    pub fn fields_immutable(&self) -> &Slots {
        let fields = &self.data;
        match fields {
            StandardObject(data) => data.as_ref().unwrap(),
            _ => panic!("The Object is array"),
        }
    }

    #[inline]
    pub fn mut_data(&mut self) -> &mut DataType {
        return &mut self.data;
    }

    #[inline]
    pub fn data(&self) -> &DataType {
        return &self.data;
    }

    #[inline]
    pub fn is_class_object(&self) -> bool {
        return self.meta.is_some();
    }

    #[inline]
    pub fn is_array_object(&self) -> bool {
        match &self.data {
            StandardObject(_) => false,
            _ => true,
        }
    }

    #[inline]
    pub fn is_instance_of(&self, class: Rc<RefCell<Class>>) -> bool {
        return (*class)
            .borrow()
            .is_assignable_from(self.class.as_ref().borrow().borrow());
    }

    pub fn set_ref_var(&mut self, name: &str, descriptor: &str, reference: Rc<RefCell<Object>>) {
        let field = Class::get_field(Some(self.class.clone()), name, descriptor, false);
        let slots = self.fields();
        slots.set_ref((*field.unwrap()).borrow().slot_id(), Some(reference));
    }

    pub fn get_ref_var(&self, name: &str, descriptor: &str) -> Option<Rc<RefCell<Object>>> {
        let field = Class::get_field(Some(self.class.clone()), name, descriptor, false);
        let fields = &self.data;
        let slots = match fields {
            StandardObject(data) => data.as_ref().unwrap(),
            _ => panic!("The Object is array"),
        };
        return slots.get_ref((*field.unwrap()).borrow().slot_id());
    }

    pub fn set_int_var(&mut self, name: &str, descriptor: &str, val: i32) {
        let field = Class::get_field(Some(self.class.clone()), name, descriptor, false);
        let slots = self.fields();
        slots.set_int((*field.unwrap()).borrow().slot_id(), val);
    }

    pub fn get_int_var(&self, name: &str, descriptor: &str) -> i32 {
        let field = Class::get_field(Some(self.class.clone()), name, descriptor, false);
        let fields = &self.data;
        let slots = match fields {
            StandardObject(data) => data.as_ref().unwrap(),
            _ => panic!("The Object is array"),
        };
        return slots.get_int((*field.unwrap()).borrow().slot_id());
    }

    pub fn get_ref_var_by_slot_id(&self, slot_id: usize) -> Option<Rc<RefCell<Object>>> {
        let slots = match &self.data {
            StandardObject(data) => data.as_ref().unwrap(),
            _ => panic!("The Object is array"),
        };
        return slots.get_ref(slot_id);
    }

    #[inline]
    pub fn get_long_var_by_slot_id(&self, slot_id: usize) -> i64 {
        let slots = match &self.data {
            StandardObject(data) => data.as_ref().unwrap(),
            _ => panic!("The Object is array"),
        };
        return slots.get_long(slot_id);
    }

    #[inline]
    pub fn set_long_var_by_slot_id(&mut self, slot_id: usize, value: i64) {
        let slots = match &mut self.data {
            StandardObject(data) => data.as_mut().unwrap(),
            _ => panic!("The Object is array"),
        };
        return slots.set_long(slot_id, value);
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let l = self as *const Object;
        let r = other as *const Object;
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
    References(Vec<Option<Rc<RefCell<Object>>>>),
}

#[derive(Debug, Clone)]
pub enum MetaData {
    Null,
    Field(Rc<RefCell<Field>>),
    Method(Rc<Method>),
    ClassLoader(Rc<RefCell<ClassLoader>>)
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
}
