use crate::class_file::constant_pool::{ConstantPool, get_utf8};
use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

struct SignatureAttribute<'a> {
    cp:&'a ConstantPool,
    signature_index:u16
}

impl SignatureAttribute<'_> {
    pub fn signature(&self) -> &str {
        return get_utf8(self.cp,self.signature_index as usize);
    }
}

impl AttributeInfo for SignatureAttribute<'_> {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.signature_index = reader.read_u16()
    }
}