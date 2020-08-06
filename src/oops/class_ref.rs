use lark_classfile::constant_pool::ConstantClassInfo;
use crate::oops::class::Class;
use crate::oops::constant_pool::ConstantPool;
use crate::oops::sym_ref::SymbolRef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ClassRef {
    symbol_ref: SymbolRef,
}

impl ClassRef {
    pub fn new_class_ref(info: &ConstantClassInfo) -> ClassRef {
        return ClassRef {
            symbol_ref: SymbolRef::with_info(info),
        };
    }

    #[inline]
    pub fn resolved_class(&mut self,holder:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        return self.symbol_ref.resolved_class(holder);
    }
}
