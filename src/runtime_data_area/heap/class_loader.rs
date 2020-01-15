use std::rc::Rc;
use crate::class_path::class_path::{ClassPath, Entry};
use std::collections::HashMap;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::class_file::ClassFile;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::constant_pool::Constant;
use crate::runtime_data_area::heap::slots::Slots;
use std::cell::{RefCell, RefMut};

pub struct ClassLoader {
    class_path:Rc<ClassPath>,
    class_map:HashMap<String,Rc<RefCell<Class>>>
}

impl ClassLoader {
    #[inline]
    pub fn new(class_path:Rc<ClassPath>) -> ClassLoader {
        return ClassLoader{
            class_path: class_path,
            class_map: Default::default()
        };
    }

    #[inline]
    pub fn class_map(&mut self) -> &mut HashMap<String,Rc<RefCell<Class>>> {
        return &mut self.class_map;
    }

    pub fn load_class(loader:Rc<RefCell<ClassLoader>>,class_name:&str) -> Rc<RefCell<Class>> {
        let class_op = (*loader).borrow_mut().class_map().get(class_name);
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }
        let class = (*loader).borrow_mut().load_non_array_class(class_name);
        (*class).borrow_mut().set_class_loader(loader);
        return class;
    }

    pub fn load_non_array_class(&mut self,class_name:&str) -> Rc<RefCell<Class>> {
        let (bytes,entry) = self.read_class(class_name);
        let class = self.define_class(bytes);
        ClassLoader::link(&class);
        return class;
    }

    pub fn read_class(&self,class_name:&str) -> (Vec<u8>,Box<dyn Entry>) {
        let result = self.class_path.read_class(class_name);
        if result.is_err() {
            panic!("java.lang.ClassNotFoundException:{}",class_name);
        }
        return result.unwrap();
    }

    pub fn define_class(&mut self,data:Vec<u8>) -> Rc<RefCell<Class>> {
        let mut class = ClassLoader::parse_class(data);
//        class.set_class_loader(Rc::clone());
        ClassLoader::resolve_super_class(&mut class);
        ClassLoader::resolve_interfaces(&mut class);
        let class_point = Rc::new(RefCell::new(class));
        self.class_map.insert(class_point.name().to_string(),class_point.clone());
        return class_point;
    }

    pub fn parse_class(data:Vec<u8>) -> Class {
        let class_file = ClassFile::parse(data);
        return Class::new(class_file);
    }

    pub fn resolve_super_class(class:&mut Class) {
        if class.name() != "java/lang/Object" {
            let super_class = (*class.loader()).borrow_mut()
                .load_class(class.super_class_name());
            class.set_super_class(super_class);
        }
    }

    pub fn resolve_interfaces(class:&mut Class) {
        let interfaces_name = class.interfaces_name();
        let len = interfaces_name.len();
        if len > 0 {
            let mut interfaces = Vec::with_capacity(len);
            for name in interfaces_name {
                let interface = (*class.loader()).borrow_mut().load_class(name);
                interfaces.push(interface);
            }
            class.set_interfaces(interfaces);
        }
    }

    pub fn link(class:&Rc<RefCell<Class>>) {
        ClassLoader::verify(class);
        ClassLoader::prepare(class);
    }

    fn verify(class:&Rc<RefCell<Class>>) {

    }

    fn prepare(class:&Rc<RefCell<Class>>) {
        ClassLoader::calc_instance_field_slot_ids(class.clone());
        ClassLoader::calc_static_field_slot_ids(class.clone());
        ClassLoader::alloc_and_init_static_vars(class.clone());
    }

    fn calc_instance_field_slot_ids(class:Rc<RefCell<Class>>) {
        let mut slot_id = 0usize;
        let super_class = (*class).borrow().super_class();
        if super_class.is_some() {
            slot_id = (*super_class.unwrap()).borrow().instance_slot_count() as usize;
        }
        for field in (*class).borrow_mut().mut_fields() {
            if !field.parent().is_static() {
                field.set_slot(slot_id);
                slot_id += 1;
                if field.is_long_or_double() {
                    slot_id += 1;
                }
            }
        }
        (*class).borrow_mut().set_instance_slot_count(slot_id as u32);
    }

    fn calc_static_field_slot_ids(class:Rc<RefCell<Class>>) {
        let mut slot_id = 0usize;
        for field in (*class).borrow_mut().mut_fields() {
            if field.parent().is_static() {
                field.set_slot(slot_id);
                slot_id += 1;
                if field.is_long_or_double() {
                    slot_id += 1;
                }
            }
        }
        (*class).borrow_mut().set_static_slot_count(slot_id as u32);
    }

    fn alloc_and_init_static_vars(class:Rc<RefCell<Class>>) {
        class.set_static_vars(Slots::with_capacity((*class).borrow().static_slot_count() as usize));
        for field in (*class).borrow_mut().mut_fields() {
            let parent = field.parent();
            if parent.is_static() && parent.is_final(){
                ClassLoader::init_static_final_var(class.clone(), field)
            }
        }
    }

    fn init_static_final_var(class:Rc<RefCell<Class>>, field: &mut Field) {
        let vars = (*class).borrow_mut().mut_static_vars().expect("static_vars is none");
        let pool = (*class).borrow().constant_pool();
        let cp_index = field.const_value_index();
        let slot_id = field.slot_id();
        if cp_index > 0 {
            match field.parent().descriptor() {
                "Z" | "B" | "C" | "S" | "I" => {
                    let val = pool.get_constant(cp_index);
                    match val {
                        Constant::Integer(v) => vars.set_int(slot_id,*v),
                        _ => {}
                    }
                },
                "J" => {
                    let val = pool.get_constant(cp_index);
                    match val {
                        Constant::Long(v) => vars.set_long(slot_id,*v),
                        _ => {}
                    }
                },
                "F" => {
                    let val = pool.get_constant(cp_index);
                    match val {
                        Constant::Float(v) => vars.set_float(slot_id,*v),
                        _ => {}
                    }
                },
                "D" => {
                    let val = pool.get_constant(cp_index);
                    match val {
                        Constant::Double(v) => vars.set_double(slot_id,*v),
                        _ => {}
                    }
                },
                "Ljava/lang/String;" => {},
                _ => {}
            }
        }
    }

}