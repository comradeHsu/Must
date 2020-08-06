use crate::attribute_info::AttributeInfo;
use crate::class_reader::ClassReader;
use crate::constant_pool::ConstantPool;
use std::rc::Rc;

pub struct SignatureAttribute {
    cp: Rc<ConstantPool>,
    signature_index: u16,
}

impl SignatureAttribute {
    pub fn signature(&self) -> &str {
        return self.cp.get_utf8(self.signature_index as usize);
    }
}

impl AttributeInfo for SignatureAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.signature_index = reader.read_u16()
    }
}
