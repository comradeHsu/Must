use crate::jvm::Jvm;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::object::DataType::Chars;
use crate::runtime_data_area::heap::object::Object;
use crate::utils::{boxed, java_str_to_rust_str};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct StringPool {
    pool: HashMap<String, Rc<RefCell<Object>>>,
}

static mut STRING_POOL: Option<StringPool> = None;

impl StringPool {
    fn instance() -> &'static StringPool {
        unsafe {
            if STRING_POOL.is_none() {
                STRING_POOL = Some(StringPool {
                    pool: HashMap::new(),
                });
            }
            return STRING_POOL.as_ref().unwrap();
        }
    }

    fn mut_instance() -> &'static mut StringPool {
        unsafe {
            if STRING_POOL.is_none() {
                STRING_POOL = Some(StringPool {
                    pool: HashMap::new(),
                });
            }
            return STRING_POOL.as_mut().unwrap();
        }
    }

    pub fn java_string(string: String) -> Rc<RefCell<Object>> {
        let pool_str = StringPool::instance().pool.get(&string);
        if pool_str.is_some() {
            return pool_str.unwrap().clone();
        }
        let chars: Vec<u16> = string.encode_utf16().collect();
        let bootstrap_loader = Jvm::boot_class_loader();
        let java_chars = Object::from_data(bootstrap_loader.find_or_create("[C"), Chars(chars));
        let mut java_string =
            Class::new_object(&bootstrap_loader.find_or_create("java/lang/String"));
        java_string.set_ref_var("value", "[C", boxed(java_chars));
        let target = boxed(java_string);
        StringPool::mut_instance()
            .pool
            .insert(string, target.clone());
        return target;
    }

    ///java sdk function
    /// string.intern
    pub fn intern_string(string: Rc<RefCell<Object>>) -> Rc<RefCell<Object>> {
        let rust_str = java_str_to_rust_str(string.clone());
        let pool_string = StringPool::instance().pool.get(&rust_str);
        if pool_string.is_some() {
            return pool_string.unwrap().clone();
        }
        StringPool::mut_instance()
            .pool
            .insert(rust_str, string.clone());
        return string;
    }
}
