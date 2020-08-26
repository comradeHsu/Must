use crate::class_loader::app_class_loader::ClassLoader;
use crate::oops::class::Class;
use lark_classfile::constant_pool::ConstantClassInfo;
use std::sync::RwLock;

pub struct SymbolRef {
    class_name: String,
    class: RwLock<Option<Class>>,
}

impl SymbolRef {
    #[inline]
    pub fn new() -> SymbolRef {
        return SymbolRef {
            class_name: "".to_string(),
            class: RwLock::new(None),
        };
    }

    pub fn with_info(info: &ConstantClassInfo) -> SymbolRef {
        return SymbolRef {
            class_name: info.name().to_string(),
            class: RwLock::new(None),
        };
    }

    #[inline]
    pub fn set_class_name(&mut self, name: String) {
        self.class_name = name;
    }

    pub fn resolved_class(&self, holder: &Class) -> Class {
        let class_op = {
            let class = self.class.read().unwrap();
            class.clone()
        };
        match class_op {
            Some(class) => class,
            None => self.resolved_class_ref(holder)
        }
    }

    pub fn resolved_class_ref(&self, holder: &Class ) -> Class {
        let ref_class = self.resolve_load(holder);
        if !ref_class
            .is_accessible_to(holder)
        {
            panic!("java.lang.IllegalAccessError");
        }
        let mut class = self.class.write().unwrap();
        *class = Some(ref_class.clone());
        ref_class
    }

    fn resolve_load(&self, holder: &Class) -> Class {
        let class_loader = holder.get_class_loader();
        return ClassLoader::load_class(class_loader, self.class_name.as_str());
    }
}
