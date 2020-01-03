use crate::class_file::constant_pool::{ConstantPool, get_utf8};
use crate::class_file::class_reader::ClassReader;
use crate::class_file::attribute_info::{AttributeInfo, read_attributes};

pub struct MemberInfo<'a> {
    cp:& 'a ConstantPool,
    access_flags:u16,
    name_index:u16,
    descriptor_index:u16,
    attributes:Vec<Box<dyn AttributeInfo>>
}

impl MemberInfo<'_> {
    pub fn read_member<'a>(reader:&mut ClassReader, cp: &'a ConstantPool) -> MemberInfo<'a> {
        return MemberInfo{
            cp,
            access_flags: reader.read_u16(),
            name_index: reader.read_u16(),
            descriptor_index: reader.read_u16(),
            attributes: read_attributes(reader,cp)
        };
    }

    pub fn read_members<'a>(reader:&mut ClassReader, cp: &'a ConstantPool) -> Vec<MemberInfo<'a>> {
        let member_count = reader.read_u16();
        let mut members:Vec<MemberInfo> = Vec::new();
        for i in 0..member_count {
            let member_info = MemberInfo::read_member(reader,cp);
            members.push(MemberInfo::read_member(reader,cp));
        }
        return members;
    }

    pub fn name(&self) -> &str {
        return get_utf8(self.cp,self.name_index as usize);
    }

    pub fn descriptor(&self) -> &str {
        return get_utf8(self.cp,self.descriptor_index as usize);
    }
}