use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

struct ExceptionsAttribute {
    exception_index_table:Vec<u16>
}

impl ExceptionsAttribute {
    pub fn exception_index_table(&self) -> &Vec<u16> {
        return &self.exception_index_table;
    }
}

impl AttributeInfo for ExceptionsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.exception_index_table = reader.read_u16_table();
    }
}