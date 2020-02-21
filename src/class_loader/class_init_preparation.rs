use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::constant_pool::Constant;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::heap::string_pool::StringPool;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ClassPreparation();

impl ClassPreparation {
    pub fn prepare(class: &Rc<RefCell<Class>>) {
        Self::calc_instance_field_slot_ids(class.clone());
        Self::calc_static_field_slot_ids(class.clone());
        Self::alloc_and_init_static_vars(class.clone());
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
            Self::init_static_final_var(class.clone(), field)
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
                    let java_string = StringPool::java_string(mete_str.to_string());
                    vars.set_ref(slot_id, Some(java_string));
                }
                _ => {}
            }
        }
    }
}
