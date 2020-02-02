use std::rc::Rc;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::constant_pool::ConstantClassInfo;
use crate::runtime_data_area::heap::class_loader::ClassLoader;
use std::ops::Deref;
use std::cell::RefCell;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct SymRef {
    constant_pool:Rc<RefCell<ConstantPool>>,
    class_name:String,
    class:Option<Rc<RefCell<Class>>>
}

impl SymRef {
    #[inline]
    pub fn new() -> SymRef{
        return SymRef{
            constant_pool: Rc::new(RefCell::new(ConstantPool::none())),
            class_name: "".to_string(),
            class: None
        };
    }

    #[inline]
    pub fn with_pool(pool:Rc<RefCell<ConstantPool>>) -> SymRef{
        return SymRef{
            constant_pool: pool,
            class_name: "".to_string(),
            class: None
        };
    }

    pub fn new_sym_ref(cp:Rc<RefCell<ConstantPool>>,info:&ConstantClassInfo) -> SymRef {
        return SymRef{
            constant_pool: cp,
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

    pub fn resolved_class(&mut self,pool_class:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        if self.class.is_none() {
            self.resolved_class_ref(pool_class);
        }
        let class = self.class.as_ref().unwrap();
        return class.clone();
    }

    pub fn resolved_class_ref(&mut self,pool_class:Rc<RefCell<Class>>) {
        let class_loader = (*pool_class).borrow().loader();
        let ref_class = ClassLoader::load_class(class_loader,self.class_name.as_str());
        if !(*ref_class).borrow().is_accessible_to((*pool_class).borrow().deref()) {
            panic!("java.lang.IllegalAccessError");
        }
        self.class = Some(ref_class);
    }
}