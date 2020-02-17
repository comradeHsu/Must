use crate::class_file::class_file::ClassFile;
use crate::class_path::class_path::{ClassPath, Entry};
use crate::runtime_data_area::heap::access_flags::PUBLIC;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::class_name_helper::PrimitiveTypes;
use crate::runtime_data_area::heap::constant_pool::{Constant, ConstantPool};
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::utils::boxed;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;

pub struct ClassLoader {
    class_path: Rc<ClassPath>,
    verbose_class: bool,
    class_map: HashMap<String, Rc<RefCell<Class>>>,
}

impl ClassLoader {

    #[inline]
    pub fn non_bootstrap_loader(verbose_class: bool) -> Rc<RefCell<ClassLoader>> {
        let class_loader = boxed(ClassLoader {
            class_path:Rc::new(ClassPath::new()),
            verbose_class,
            class_map: Default::default(),
        });
        return class_loader;
    }

    #[inline]
    pub fn new(class_path: Rc<ClassPath>, verbose_class: bool) -> Rc<RefCell<ClassLoader>> {
        let class_loader = boxed(ClassLoader {
            class_path,
            verbose_class,
            class_map: Default::default(),
        });
        ClassLoader::load_basic_classes(class_loader.clone());
        ClassLoader::load_primitive_classes(class_loader.clone());
        return class_loader;
    }

    fn load_basic_classes(loader: Rc<RefCell<ClassLoader>>) {
        let java_lang_class = ClassLoader::load_class(loader.clone(), "java/lang/Class");
        let borrow = (*loader).borrow();
        let maps = borrow.class_map_immutable();
        for (k, v) in maps {
            let mut borrow_class = (**v).borrow_mut();
            let j_l_class = borrow_class.java_class();
            if j_l_class.is_none() {
                let mut class_object = Class::new_object(&java_lang_class);
                class_object.set_meta(v.clone());
                let boxed = boxed(class_object);
                borrow_class.set_java_class(Some(boxed));
            }
        }
    }

    fn load_primitive_classes(loader: Rc<RefCell<ClassLoader>>) {
        let primitives = PrimitiveTypes::instance().unwrap();
        let maps = primitives.primitive_types();
        for (k, v) in maps {
            ClassLoader::load_primitive_class(loader.clone(), k);
        }
    }

    fn load_primitive_class(loader: Rc<RefCell<ClassLoader>>, class_name: &str) {
        let mut class = Class::primitive_class(loader.clone(), class_name);
        let class_class = (*loader).borrow().get_class("java/lang/Class");
        let mut class_object = Class::new_object(&class_class.unwrap());
        let boxed_class = boxed(class);
        class_object.set_meta(boxed_class.clone());
        (*boxed_class)
            .borrow_mut()
            .set_java_class(Some(boxed(class_object)));
        (*loader)
            .borrow_mut()
            .class_map
            .insert(class_name.to_string(), boxed_class);
    }

    #[inline]
    pub fn class_map(&mut self) -> &mut HashMap<String, Rc<RefCell<Class>>> {
        return &mut self.class_map;
    }

    #[inline]
    pub fn class_map_immutable(&self) -> &HashMap<String, Rc<RefCell<Class>>> {
        return &self.class_map;
    }

    #[inline]
    pub fn get_class(&self, name: &str) -> Option<Rc<RefCell<Class>>> {
        let rs = self.class_map.get(name);
        match rs {
            Some(r) => return Some(r.clone()),
            None => None,
        }
    }

    pub fn load_class(loader: Rc<RefCell<ClassLoader>>, class_name: &str) -> Rc<RefCell<Class>> {
        let clone_loader = loader.clone();
        let class_op: Option<Rc<RefCell<Class>>> = (*clone_loader).borrow().get_class(class_name);
        if class_op.is_some() {
            return class_op.unwrap().clone();
        }
        let mut class: Option<Rc<RefCell<Class>>> = None;
        if class_name.starts_with('[') {
            class = Some(ClassLoader::load_array_class(loader.clone(), class_name));
        } else {
            class = Some(ClassLoader::load_non_array_class(
                loader.clone(),
                class_name,
            ));
        }
        let value = class.unwrap();
        let class_class = (*loader).borrow().get_class("java/lang/Class");
        if class_class.is_some() {
            let mut class_object = Class::new_object(&class_class.unwrap());
            class_object.set_meta(value.clone());
            let boxed = boxed(class_object);
            (*value).borrow_mut().set_java_class(Some(boxed));
        }
        return value;
    }

    ///load array's class
    fn load_array_class(loader: Rc<RefCell<ClassLoader>>, class_name: &str) -> Rc<RefCell<Class>> {
        let class = Class::new_array_class(loader.clone(), class_name);
        let class_ptr = boxed(class);
        (*loader)
            .borrow_mut()
            .class_map
            .insert(class_name.to_string(), class_ptr.clone());
        return class_ptr;
    }

    pub fn load_non_array_class(
        loader: Rc<RefCell<ClassLoader>>,
        class_name: &str,
    ) -> Rc<RefCell<Class>> {
        let (bytes, entry) = (*loader).borrow().read_class(class_name);
        let class = ClassLoader::define_class(loader.clone(), bytes);
        ClassLoader::link(&class);
        if (*loader).borrow().verbose_class {
            println!("Loaded {}.class from {}", class_name, entry.to_string());
        }
        return class;
    }

    pub fn read_class(&self, class_name: &str) -> (Vec<u8>, Box<dyn Entry>) {
        let result = self.class_path.read_class(class_name);
        if result.is_err() {
            panic!("java.lang.ClassNotFoundException:{}", class_name);
        }
        return result.unwrap();
    }

    pub fn define_class(loader: Rc<RefCell<ClassLoader>>, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let mut class = ClassLoader::parse_class(data);
        (*class).borrow_mut().set_class_loader(loader.clone());
        ClassLoader::resolve_super_class(class.clone());
        ClassLoader::resolve_interfaces(class.clone());
        (*loader)
            .borrow_mut()
            .class_map
            .insert((*class).borrow().name().to_string(), class.clone());
        return class;
    }

    pub fn parse_class(data: Vec<u8>) -> Rc<RefCell<Class>> {
        let class_file = ClassFile::parse(data);
        //        class_file.display();
        return Class::new(class_file);
    }

    pub fn resolve_super_class(class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let super_class_name = class.super_class_name();
        //        println!("resolve_super_class:{:?},super:{:?}",class.name(),super_class_name);
        if class.name() != "java/lang/Object" && super_class_name.is_some() {
            let super_class =
                ClassLoader::load_class(class.loader(), super_class_name.unwrap().as_str());
            class.set_super_class(super_class);
        }
    }

    pub fn resolve_interfaces(class: Rc<RefCell<Class>>) {
        let mut class = (*class).borrow_mut();
        let interfaces_name = class.interfaces_name();
        let len = interfaces_name.len();
        if len > 0 {
            let mut interfaces = Vec::with_capacity(len);
            for name in interfaces_name {
                let interface = ClassLoader::load_class(class.loader(), name);
                interfaces.push(interface);
            }
            class.set_interfaces(interfaces);
        }
    }

    pub fn link(class: &Rc<RefCell<Class>>) {
        ClassLoader::verify(class);
        ClassLoader::prepare(class);
    }

    fn verify(class: &Rc<RefCell<Class>>) {}

    fn prepare(class: &Rc<RefCell<Class>>) {
        ClassLoader::calc_instance_field_slot_ids(class.clone());
        ClassLoader::calc_static_field_slot_ids(class.clone());
        ClassLoader::alloc_and_init_static_vars(class.clone());
    }

    fn calc_instance_field_slot_ids(class: Rc<RefCell<Class>>) {
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
        (*class)
            .borrow_mut()
            .set_instance_slot_count(slot_id as u32);
    }

    fn calc_static_field_slot_ids(class: Rc<RefCell<Class>>) {
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

    fn alloc_and_init_static_vars(class: Rc<RefCell<Class>>) {
        let count = (*class).borrow().static_slot_count() as usize;
        (*class)
            .borrow_mut()
            .set_static_vars(Slots::with_capacity(count));
        let mut static_final_fields = Vec::new();
        for field in (*class).borrow().fields() {
            let is_static = field.borrow_mut().parent().is_static();
            if is_static && field.borrow_mut().parent().is_final() {
                //                ClassLoader::init_static_final_var(class.clone(), field.clone())
                static_final_fields.push(field.clone());
            }
        }
        for field in static_final_fields {
            ClassLoader::init_static_final_var(class.clone(), field)
        }
    }

    fn init_static_final_var(class: Rc<RefCell<Class>>, field: Rc<RefCell<Field>>) {
        let pool = (*class).borrow().constant_pool();
        let loader = (*class).borrow().loader();
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
                        Constant::Integer(v) => vars.set_int(slot_id, *v),
                        _ => {}
                    }
                }
                "J" => {
                    let val = borrow_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Long(v) => vars.set_long(slot_id, *v),
                        _ => {}
                    }
                }
                "F" => {
                    let val = borrow_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Float(v) => vars.set_float(slot_id, *v),
                        _ => {}
                    }
                }
                "D" => {
                    let val = borrow_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Double(v) => vars.set_double(slot_id, *v),
                        _ => {}
                    }
                }
                "Ljava/lang/String;" => {
                    let val = borrow_pool.get_constant_immutable(cp_index);
                    let mete_str = match val {
                        Constant::Str(v) => v.as_str(),
                        _ => panic!("It's not string"),
                    };
                    let java_string = StringPool::java_string(loader, mete_str.to_string());
                    vars.set_ref(slot_id, Some(java_string));
                }
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
