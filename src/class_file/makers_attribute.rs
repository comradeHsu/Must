use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

struct MakerAttribute{}

struct DeprecatedAttribute(MakerAttribute);

impl AttributeInfo for DeprecatedAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        unimplemented!()
    }
}

struct SyntheticAttribute(MakerAttribute);

impl AttributeInfo for SyntheticAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        unimplemented!()
    }
}