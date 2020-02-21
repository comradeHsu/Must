use std::rc::Rc;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::constant_pool::ConstantClassInfo;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use std::ops::Deref;
use std::cell::RefCell;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct SymbolRef {
    holder:Option<Rc<RefCell<Class>>>,
    class_name:String,
    class:Option<Rc<RefCell<Class>>>
}

impl SymbolRef {
    #[inline]
    pub fn new() -> SymbolRef {
        return SymbolRef {
            holder: None,
            class_name: "".to_string(),
            class: None
        };
    }

    #[inline]
    pub fn with_holder(holder:Rc<RefCell<Class>>) -> SymbolRef {
        return SymbolRef {
            holder: Some(holder),
            class_name: "".to_string(),
            class: None
        };
    }

    pub fn new_sym_ref(holder:Rc<RefCell<Class>>,info:&ConstantClassInfo) -> SymbolRef {
        return SymbolRef {
            holder: Some(holder),
            class_name: info.name().to_string(),
            class: None
        };
    }

    #[inline]
    pub fn set_class_name(&mut self,name:String) {
        self.class_name = name;
    }

    #[inline]
    pub fn set_constant_pool(&mut self,pool:Rc<RefCell<ConstantPool>>) {
        self.constant_pool = pool;
    }

    #[inline]
    pub fn constant_pool(&self) -> Rc<RefCell<ConstantPool>> {
        return self.constant_pool.clone();
    }

    pub fn resolved_class(&mut self,class:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        if self.class.is_none() {
            self.resolved_class_ref(class);
        }
        let class = self.class.as_ref().unwrap();
        return class.clone();
    }

    pub fn resolved_class_ref(&mut self,class:Rc<RefCell<Class>>) {
        let class = (*self.constant_pool).borrow().class();
        let class_loader = (*class).borrow().loader();
        let ref_class = ClassLoader::load_class(class_loader,self.class_name.as_str());
        if !(*ref_class).borrow().is_accessible_to((*class).borrow().deref()) {
            panic!("java.lang.IllegalAccessError");
        }
        self.class = Some(ref_class);
    }
}