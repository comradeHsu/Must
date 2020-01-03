use crate::class_file::constant_pool::{ConstantPool, get_utf8};
use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

struct SourceFileAttribute<'a> {
    cp:&'a ConstantPool,
    source_file_index:u16
}

impl SourceFileAttribute<'_> {
    pub fn file_name(&self) -> &str {
        return get_utf8(self.cp,self.source_file_index as usize);
    }
}

impl AttributeInfo for SourceFileAttribute<'_> {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.source_file_index = reader.read_u16();
    }
}