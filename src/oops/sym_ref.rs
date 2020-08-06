use lark_classfile::constant_pool::ConstantClassInfo;
use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::Jvm;
use crate::oops::class::Class;
use crate::oops::object::Object;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SymbolRef {
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
}

impl SymbolRef {
    #[inline]
    pub fn new() -> SymbolRef {
        return SymbolRef {
            class_name: "".to_string(),
            class: None,
        };
    }

    pub fn with_info(info: &ConstantClassInfo) -> SymbolRef {
        return SymbolRef {
            class_name: info.name().to_string(),
            class: None,
        };
    }

    #[inline]
    pub fn set_class_name(&mut self, name: String) {
        self.class_name = name;
    }

    pub fn resolved_class(&mut self,holder:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        if self.class.is_none() {
            self.resolved_class_ref(holder);
        }
        let class = self.class.as_ref().unwrap();
        return class.clone();
    }

    pub fn resolved_class_ref(&mut self, holder:Rc<RefCell<Class>>) {
        let ref_class = self.resolve_load(holder.clone());
        if !(*ref_class)
            .borrow()
            .is_accessible_to((*holder).borrow().deref())
        {
            panic!("java.lang.IllegalAccessError");
        }
        self.class = Some(ref_class);
    }

    fn resolve_load(&self,holder:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        let class_loader = (*holder).borrow().get_class_loader();
        return ClassLoader::load_class(class_loader, self.class_name.as_str());
    }
}
