use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

struct UnparsedAttribute {
    name:String,
    len:u32,
    info:Option<Vec<u8>>
}

impl UnparsedAttribute {

}

impl AttributeInfo for UnparsedAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.info = Some(reader.read_bytes(self.len as usize));
    }
}