use crate::runtime_data_area::heap::sym_ref::SymRef;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use std::rc::Rc;
use crate::class_file::constant_pool::ConstantClassInfo;

pub struct ClassRef {
    sym_ref:SymRef
}

impl ClassRef {
    pub fn new_class_ref(cp:Rc<ConstantPool>,info:&ConstantClassInfo) -> ClassRef {
        return ClassRef{sym_ref:SymRef::new_sym_ref(cp,info)}
    }


}