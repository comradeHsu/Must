use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::class_reader::ClassReader;

pub struct MemberInfo<'a> {
    cp:& 'a ConstantPool,
    access_flags:u16,
    name_index:u16,
    descriptor_index:u16,
    attributes:()
}

impl MemberInfo {
    fn read_member(reader:&mut ClassReader, cp: &ConstantPool) -> MemberInfo {
        return MemberInfo{
            cp,
            access_flags: reader.read_u16(),
            name_index: reader.read_u16(),
            descriptor_index: reader.read_u16(),
            attributes: ()
        };
    }

    fn read_members(reader:&mut ClassReader, cp: &ConstantPool) -> Vec<MemberInfo> {
        let member_count = reader.read_u16();
        let mut members = Vec::new();
        for i in 0..member_count {
            members.push(MemberInfo::read_member(reader,cp));
        }
        return members;
    }

    fn name(&self) -> &str {
        return "";
    }

    fn descriptor(&self) -> &str {
        return "";
    }
}