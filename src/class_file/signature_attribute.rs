use crate::class_file::constant_pool::{ConstantPool, get_utf8};
use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;
use std::rc::Rc;

struct SignatureAttribute {
    cp:Rc<ConstantPool>,
    signature_index:u16
}

impl SignatureAttribute {
    pub fn signature(&self) -> &str {
        return get_utf8(self.cp.clone(),self.signature_index as usize);
    }
}

impl AttributeInfo for SignatureAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.signature_index = reader.read_u16()
    }
}