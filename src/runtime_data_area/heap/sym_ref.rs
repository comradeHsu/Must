use std::rc::Rc;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::constant_pool::ConstantClassInfo;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use std::ops::Deref;
use std::cell::RefCell;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct SymbolRef {
    pub(in crate::runtime_data_area::heap) holder:Option<Rc<RefCell<Class>>>,
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

    pub fn with_info(info:&ConstantClassInfo) -> SymbolRef {
        return SymbolRef {
            holder: None,
            class_name: info.name().to_string(),
            class: None
        };
    }

    #[inline]
    pub fn set_class_name(&mut self,name:String) {
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
        let class_loader = (*self.holder()).borrow().loader();
        let ref_class = ClassLoader::load_class(class_loader,self.class_name.as_str());
        if !(*ref_class).borrow().is_accessible_to((*self.holder()).borrow().deref()) {
            panic!("java.lang.IllegalAccessError");
        }
        self.class = Some(ref_class);
    }

    fn holder(&self) -> Rc<RefCell<Class>> {
        return self.holder.clone().unwrap();
    }
}