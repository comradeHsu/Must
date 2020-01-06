use crate::class_file::constant_pool::{ConstantPool, get_utf8, get_name_and_type};
use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;
use std::rc::Rc;

struct EnclosingMethodAttribute {
    cp:Rc<ConstantPool>,
    class_index:u16,
    method_index:u16
}

impl EnclosingMethodAttribute {
    pub fn class_name(&self) -> &str {
        return get_utf8(self.cp.clone(),self.class_index as usize);
    }

    pub fn method_name_and_descriptor(&self) -> (&str,&str) {
        if self.class_index > 0 {
            return get_name_and_type(self.cp.clone(),self.method_index as usize);
        }
        return ("","")
    }
}

impl AttributeInfo for EnclosingMethodAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
        self.method_index = reader.read_u16();
    }
}