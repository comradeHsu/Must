use std::rc::Rc;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::class::Class;
use crate::class_file::constant_pool::ConstantClassInfo;

pub struct SymRef {
    constant_pool:Rc<ConstantPool>,
    class_name:String,
    class:Rc<Class>
}

impl SymRef {
    #[inline]
    pub fn new() -> SymRef{
        return SymRef{
            constant_pool: Rc::new(ConstantPool::into()),
            class_name: "".to_string(),
            class: Rc::new(Class::none())
        };
    }

    #[inline]
    pub fn with_pool(pool:Rc<ConstantPool>) -> SymRef{
        return SymRef{
            constant_pool: pool,
            class_name: "".to_string(),
            class: Rc::new(Class::none())
        };
    }

    pub fn new_sym_ref(cp:Rc<ConstantPool>,info:&ConstantClassInfo) -> SymRef {
        return SymRef{
            constant_pool: cp,
            class_name: info.name().to_string(),
            class: Rc::new(Class::none())
        };
    }

    #[inline]
    pub fn set_class_name(&mut self,name:String) {
        self.class_name = name;
    }

    #[inline]
    pub fn set_constant_pool(&mut self,pool:Rc<ConstantPool>) {
        self.constant_pool = pool;
    }
}