use lark_classfile::constant_pool::ConstantMemberRefInfo;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::sym_ref::SymbolRef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct MemberRef {
    symbol_ref: SymbolRef,
    name: String,
    descriptor: String,
}

impl MemberRef {
    #[inline]
    pub fn new() -> MemberRef {
        return MemberRef {
            symbol_ref: SymbolRef::new(),
            name: "".to_string(),
            descriptor: "".to_string(),
        };
    }

    pub fn copy_member_info(&mut self, info: &ConstantMemberRefInfo) {
        self.symbol_ref
            .set_class_name(info.class_name().to_string());
        let (name, desc) = info.name_and_descriptor();
        self.name = name.to_string();
        self.descriptor = desc.to_string();
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
    pub fn resolved_class(&mut self,holder:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        return self.symbol_ref.resolved_class(holder);
    }
}
