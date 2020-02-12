use crate::class_file::constant_pool::ConstantMemberRefInfo;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::sym_ref::SymRef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct MemberRef {
    sym_ref: SymRef,
    name: String,
    descriptor: String,
}

impl MemberRef {
    #[inline]
    pub fn with_pool(pool: Rc<RefCell<ConstantPool>>) -> MemberRef {
        return MemberRef {
            sym_ref: SymRef::with_pool(pool),
            name: "".to_string(),
            descriptor: "".to_string(),
        };
    }

    pub fn copy_member_info(&mut self, info: &ConstantMemberRefInfo) {
        self.sym_ref.set_class_name(info.class_name().to_string());
        let (name, desc) = info.name_and_descriptor();
        self.name = name.to_string();
        self.descriptor = desc.to_string();
    }

    #[inline]
    pub fn set_constant_pool(&mut self, pool: Rc<RefCell<ConstantPool>>) {
        self.sym_ref.set_constant_pool(pool);
    }

    #[inline]
    pub fn constant_pool(&self) -> Rc<RefCell<ConstantPool>> {
        return self.sym_ref.constant_pool();
    }

    #[inline]
    pub fn name(&self) -> &str {
        return self.name.as_str();
    }

    #[inline]
    pub fn descriptor(&self) -> &str {
        return self.descriptor.as_str();
    }

    #[inline]
    pub fn resolved_class(&mut self, pool_class: Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        return self.sym_ref.resolved_class(pool_class);
    }
}
