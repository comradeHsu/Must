use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;
use std::ptr;

pub struct ExceptionsAttribute {
    exception_index_table: Vec<u16>,
}

impl ExceptionsAttribute {
    pub fn new() -> ExceptionsAttribute {
        return ExceptionsAttribute {
            exception_index_table: vec![],
        };
    }

    pub fn exception_index_table(&self) -> &Vec<u16> {
        return &self.exception_index_table;
    }

    pub fn unsafe_copy(&self) -> Vec<u16> {
        unsafe {
            let count = self.exception_index_table.len();
            let ptr = self.exception_index_table.as_ptr();
            let mut data = Vec::with_capacity(count);
            ptr::copy_nonoverlapping(ptr, data.as_mut_ptr(), count);
            data.set_len(count);
            return data;
        }
    }
}

impl AttributeInfo for ExceptionsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.exception_index_table = reader.read_u16_table();
    }
}
