use crate::class_file::class_reader::ClassReader;
use crate::class_file::constant_pool::{ConstantPool, get_utf8};
use crate::class_file::code_attribute::CodeAttribute;

pub trait AttributeInfo {

    fn read_info(&mut self, reader:&mut ClassReader);

    fn read_attributes(reader:&mut ClassReader,cp:&ConstantPool) -> Vec<dyn AttributeInfo> {
        let attr_count = reader.read_u16();
        let mut attributes = Vec::new();
        for _ in 0..attr_count {
            attributes.push(*AttributeInfo::read_attribute(reader,cp));
        }
        return attributes;
    }

    fn read_attribute(reader:&mut ClassReader,cp:&ConstantPool) -> Box<dyn AttributeInfo> {
        let attr_name_index = reader.read_u16();
        let attr_name = get_utf8(cp,attr_name_index as usize);
        let attr_len = reader.read_u32();
        let mut info = AttributeInfo::new(attr_name,attr_len,cp);
        info.read_info(reader);
        return info;
    }

    fn new(attr_name:&str,attr_len:u32,cp:&ConstantPool) -> Box<dyn AttributeInfo> {
        let info = match attr_name {
            "Code" => CodeAttribute{
                cp,
                max_stack: 0,
                max_locals: 0,
                code: vec![],
                exception_table: vec![],
                attributes: vec![]
            },
            _ => {}
        };
        return Box::new(info);
    }
}