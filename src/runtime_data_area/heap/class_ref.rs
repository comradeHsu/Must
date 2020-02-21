use crate::runtime_data_area::heap::sym_ref::SymbolRef;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use std::rc::Rc;
use crate::class_file::constant_pool::ConstantClassInfo;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;

#[derive(Debug)]
pub struct ClassRef {
    sym_ref: SymbolRef
}

impl ClassRef {
    pub fn new_class_ref(holder:Rc<RefCell<Class>>,info:&ConstantClassInfo) -> ClassRef {
        return ClassRef{sym_ref: SymbolRef::new_sym_ref(holder, info)}
    }

    #[inline]
    pub fn resolved_class(&mut self,class:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        return self.sym_ref.resolved_class(class);
    }

    #[inline]
    pub fn set_constant_pool(&mut self,pool:Rc<RefCell<ConstantPool>>) {
        self.sym_ref.set_constant_pool(pool);
    }
}