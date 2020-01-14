use crate::runtime_data_area::heap::sym_ref::SymRef;
use crate::class_file::constant_pool::ConstantMemberRefInfo;

pub struct MemberRef {
    sym_ref:SymRef,
    name:String,
    descriptor:String
}

impl MemberRef {

    pub fn copy_member_info(&mut self,info:&ConstantMemberRefInfo) {
        self.sym_ref.set_class_name(info.class_name().to_string());
        let (name,desc) = info.name_and_descriptor();
        self.name = name.to_string();
        self.descriptor = desc.to_string();
    }
}