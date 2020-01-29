use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::object::Object;
use crate::runtime_data_area::heap::object::DataType::Chars;
use crate::utils::boxed;

pub struct StringPool {
    pool:HashMap<String,Rc<RefCell<Object>>>
}

static mut STRING_POOL:Option<StringPool> = None;

impl StringPool {

    fn instance() -> &'static StringPool {
        unsafe {
            if STRING_POOL.is_none() {
                STRING_POOL = Some(StringPool{ pool: HashMap::new() });
            }
            return STRING_POOL.as_ref().unwrap();
        }
    }

    fn mut_instance() -> &'static mut StringPool {
        unsafe {
            if STRING_POOL.is_none() {
                STRING_POOL = Some(StringPool{ pool: HashMap::new() });
            }
            return STRING_POOL.as_mut().unwrap();
        }
    }

    pub fn java_string(loader:Rc<RefCell<ClassLoader>>,string:String) -> Rc<RefCell<Object>>{
        let pool_str = StringPool::instance().pool.get(&string);
        if pool_str.is_some() {
            return pool_str.unwrap().clone();
        }
        let chars:Vec<u16> = string.encode_utf16().collect();
        let java_chars = Object::from_data(ClassLoader::load_class(loader.clone(),"[C"),Chars(chars));
        let mut java_string = Class::new_object(&ClassLoader::load_class(loader,"java/lang/String"));
        java_string.set_ref_var("value","[C",boxed(java_chars));
        let target = boxed(java_string);
        StringPool::mut_instance().pool.insert(string,target.clone());
        return target;
    }
}