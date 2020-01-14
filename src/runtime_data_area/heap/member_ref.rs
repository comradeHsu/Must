use crate::runtime_data_area::heap::sym_ref::SymRef;
use crate::class_file::constant_pool::ConstantMemberRefInfo;
use std::rc::Rc;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;

pub struct MemberRef {
    sym_ref:SymRef,
    name:String,
    descriptor:String
}

impl MemberRef {

    #[inline]
    pub fn with_pool(pool:Rc<ConstantPool>) -> MemberRef {
        return MemberRef{
            sym_ref: SymRef::with_pool(pool),
            name: "".to_string(),
            descriptor: "".to_string()
        };
    }

    pub fn copy_member_info(&mut self,info:&ConstantMemberRefInfo) {
        self.sym_ref.set_class_name(info.class_name().to_string());
        let (name,desc) = info.name_and_descriptor();
        self.name = name.to_string();
        self.descriptor = desc.to_string();
    }

    #[inline]
    pub fn set_constant_pool(&mut self,pool:Rc<ConstantPool>) {
        self.sym_ref.set_constant_pool(pool);
    }
}