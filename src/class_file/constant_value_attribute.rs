use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

pub struct ConstantValueAttribute {
    value_index:u16
}

impl ConstantValueAttribute {
    pub fn new() -> ConstantValueAttribute {
        return ConstantValueAttribute{ value_index: 0 };
    }

    pub fn value_index(&self) -> u16 {
        return self.value_index;
    }
}

impl AttributeInfo for ConstantValueAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.value_index = reader.read_u16();
    }
}