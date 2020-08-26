use crate::oops::class::Class;
use crate::oops::sym_ref::SymbolRef;
use lark_classfile::constant_pool::ConstantClassInfo;
use std::sync::Arc;

#[derive(Clone)]
pub struct ClassRef {
    symbol_ref: Arc<SymbolRef>,
}

impl ClassRef {
    pub fn new_class_ref(info: &ConstantClassInfo) -> ClassRef {
        return ClassRef {
            symbol_ref: Arc::new(SymbolRef::with_info(info)),
        };
    }

    #[inline]
    pub fn resolved_class(&self, holder: &Class) -> Class {
        return self.symbol_ref.resolved_class(holder);
    }
}
