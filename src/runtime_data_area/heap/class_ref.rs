use crate::runtime_data_area::heap::sym_ref::SymRef;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use std::rc::Rc;
use crate::class_file::constant_pool::ConstantClassInfo;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;

#[derive(Debug, Clone)]
pub struct ClassRef {
    symbol_ref: SymbolRef
}

impl ClassRef {
    pub fn new_class_ref(info:&ConstantClassInfo) -> ClassRef {
        return ClassRef{symbol_ref: SymbolRef::with_info(info)}
    }

    #[inline]
    pub fn resolved_class(&mut self) -> Rc<RefCell<Class>> {
        return self.symbol_ref.resolved_class();
    }

    #[inline]
    pub fn set_holder(&mut self, holder:Rc<RefCell<Class>>) {
        self.symbol_ref.holder = Some(holder);
    }
}