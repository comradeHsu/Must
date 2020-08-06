use crate::attribute_info::AttributeInfo;
use crate::class_reader::ClassReader;
use crate::constant_pool::ConstantPool;
use std::rc::Rc;

pub struct EnclosingMethodAttribute {
    cp: Rc<ConstantPool>,
    class_index: u16,
    method_index: u16,
}

impl EnclosingMethodAttribute {
    pub fn class_name(&self) -> &str {
        return self.cp.get_utf8(self.class_index as usize);
    }

    pub fn method_name_and_descriptor(&self) -> (&str, &str) {
        if self.class_index > 0 {
            return self.cp.get_name_and_type(self.method_index as usize);
        }
        return ("", "");
    }
}

impl AttributeInfo for EnclosingMethodAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
        self.method_index = reader.read_u16();
    }
}
