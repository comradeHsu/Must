use std::rc::Rc;
use crate::class_path::class_path::{ClassPath, Entry};
use std::collections::HashMap;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::class_file::ClassFile;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::constant_pool::Constant;
use crate::runtime_data_area::heap::slots::Slots;
use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Formatter, Error};
use std::borrow::Borrow;

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

    #[inline]
    pub fn class_map_immutable(&self) -> &HashMap<String,Rc<RefCell<Class>>> {
        return &self.class_map;
    }

    #[inline]
    pub fn get_class(&self,name:&str) -> Option<Rc<RefCell<Class>>> {
        let rs = self.class_map.get(name);
        match rs {
            Some(r) => return Some(r.clone()),
            None => None
        }
    }

    pub fn load_class(loader:Rc<RefCell<ClassLoader>>,class_name:&str) -> Rc<RefCell<Class>> {
        let clone_loader = loader.clone();
//        let mut_loader = (*clone_loader).borrow();
        let class_op = (*clone_loader).borrow().get_class(class_name);
        println!("name:{},class_op:{}",class_name,class_op.is_some());
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }
        let class = ClassLoader::load_non_array_class(loader,class_name);
        return class;
    }

    pub fn load_non_array_class(loader:Rc<RefCell<ClassLoader>>,class_name:&str) -> Rc<RefCell<Class>> {
        let (bytes,entry) = (*loader).borrow().read_class(class_name);
        let class = ClassLoader::define_class(loader,bytes);
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

    pub fn define_class(loader:Rc<RefCell<ClassLoader>>,data:Vec<u8>) -> Rc<RefCell<Class>> {
        let mut class = ClassLoader::parse_class(data);
        (*class).borrow_mut().set_class_loader(loader.clone());
        println!("class:{:?}",(*class).borrow().name());
        ClassLoader::resolve_super_class(class.clone());
        ClassLoader::resolve_interfaces(class.clone());
        println!("class_name:{}",(*class).borrow().name());
        (*loader).borrow_mut().class_map.insert((*class).borrow().name().to_string(),class.clone());
        return class;
    }

    pub fn parse_class(data:Vec<u8>) -> Rc<RefCell<Class>> {
        let class_file = ClassFile::parse(data);
        class_file.display();
        return Class::new(class_file);
    }

    pub fn resolve_super_class(class:Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let super_class_name = class.super_class_name();
        println!("resolve_super_class:{:?},super:{:?}",class.name(),super_class_name);
        if class.name() != "java/lang/Object" && super_class_name.is_some() {
            let super_class =
                ClassLoader::load_class(class.loader(),super_class_name.unwrap().as_str());
            class.set_super_class(super_class);
        }
    }

    pub fn resolve_interfaces(class:Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let interfaces_name = class.interfaces_name();
        let len = interfaces_name.len();
        if len > 0 {
            let mut interfaces = Vec::with_capacity(len);
            for name in interfaces_name {
                let interface =
                    ClassLoader::load_class(class.loader(),name);
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
        {
            let borrow_class = (*class).borrow();
            let super_class = borrow_class.super_class();
            if super_class.is_some() {
                slot_id = (*super_class.unwrap()).borrow().instance_slot_count() as usize;
            }
        }
        for field in (*class).borrow_mut().fields() {
            let field = field.clone();
            if !(*field).borrow().parent().is_static() {
                (*field).borrow_mut().set_slot(slot_id);
                slot_id += 1;
                if (*field).borrow().is_long_or_double() {
                    slot_id += 1;
                }
            }
        }
        (*class).borrow_mut().set_instance_slot_count(slot_id as u32);
    }

    fn calc_static_field_slot_ids(class:Rc<RefCell<Class>>) {
        let mut slot_id = 0usize;
        for field in (*class).borrow_mut().fields() {
            let field = field.clone();
            if (*field).borrow().parent().is_static() {
                (*field).borrow_mut().set_slot(slot_id);
                slot_id += 1;
                if (*field).borrow().is_long_or_double() {
                    slot_id += 1;
                }
            }
        }
        (*class).borrow_mut().set_static_slot_count(slot_id as u32);
    }

    fn alloc_and_init_static_vars(class:Rc<RefCell<Class>>) {
        let count = (*class).borrow().static_slot_count() as usize;
        (*class).borrow_mut().set_static_vars(
            Slots::with_capacity(count)
        );
        let mut static_final_fields = Vec::new();
        for field in (*class).borrow().fields() {
            let is_static = field.borrow_mut().parent().is_static();
            if is_static && field.borrow_mut().parent().is_final(){
//                ClassLoader::init_static_final_var(class.clone(), field.clone())
                static_final_fields.push(field.clone());
            }
        }
        for field in static_final_fields {
            ClassLoader::init_static_final_var(class.clone(), field)
        }
    }

    fn init_static_final_var(class:Rc<RefCell<Class>>, field: Rc<RefCell<Field>>) {
        let pool = (*class).borrow().constant_pool();
        let mut borrow_class = (*class).borrow_mut();
        let vars = borrow_class.mut_static_vars().expect("static_vars is none");
        let cp_index = (*field).borrow().const_value_index();
        let slot_id = (*field).borrow().slot_id();
        let borrow_pool = (*pool).borrow();
        if cp_index > 0 {
            match (*field).borrow().parent().descriptor() {
                "Z" | "B" | "C" | "S" | "I" => {
                    let val = borrow_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Integer(v) => vars.set_int(slot_id,*v),
                        _ => {}
                    }
                },
                "J" => {
                    let val = borrow_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Long(v) => vars.set_long(slot_id,*v),
                        _ => {}
                    }
                },
                "F" => {
                    let val = borrow_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Float(v) => vars.set_float(slot_id,*v),
                        _ => {}
                    }
                },
                "D" => {
                    let val = borrow_pool.get_constant_immutable(cp_index);
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

impl Debug for ClassLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}