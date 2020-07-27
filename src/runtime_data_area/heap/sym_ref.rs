use crate::class_file::constant_pool::ConstantClassInfo;
use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::Jvm;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::object::Object;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SymbolRef {
    pub(in crate::runtime_data_area::heap) holder: Option<Rc<RefCell<Class>>>,
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
}

impl SymbolRef {
    #[inline]
    pub fn new() -> SymbolRef {
        return SymbolRef {
            holder: None,
            class_name: "".to_string(),
            class: None,
        };
    }

    pub fn with_info(info: &ConstantClassInfo) -> SymbolRef {
        return SymbolRef {
            holder: None,
            class_name: info.name().to_string(),
            class: None,
        };
    }

    #[inline]
    pub fn set_class_name(&mut self, name: String) {
        self.class_name = name;
    }

    pub fn resolved_class(&mut self) -> Rc<RefCell<Class>> {
        if self.class.is_none() {
            self.resolved_class_ref();
        }
        let class = self.class.as_ref().unwrap();
        return class.clone();
    }

    pub fn resolved_class_ref(&mut self) {
        let ref_class = self.resolve_load();
        if !(*ref_class)
            .borrow()
            .is_accessible_to((*self.holder()).borrow().deref())
        {
            panic!("java.lang.IllegalAccessError");
        }
        self.class = Some(ref_class);
    }

    fn holder(&self) -> Rc<RefCell<Class>> {
        return self.holder.clone().unwrap();
    }

    fn resolve_load(&self) -> Rc<RefCell<Class>> {
//        let class_object = (*self.holder())
//            .borrow()
//            .get_java_class();
        let class_object:Option<Rc<RefCell<Object>>> = None;
        let class_loader = match class_object.is_none() {
            true => None,
            false => (*class_object.unwrap())
                    .borrow()
                    .get_ref_var("classLoader", "Ljava/lang/ClassLoader;")
        };
        return ClassLoader::load_class(class_loader, self.class_name.as_str());
    }
}
