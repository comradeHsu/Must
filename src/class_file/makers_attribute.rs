use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

pub struct MakerAttribute{}

pub struct DeprecatedAttribute(MakerAttribute);

impl DeprecatedAttribute {
    pub fn new() -> DeprecatedAttribute {
        return DeprecatedAttribute(MakerAttribute{});
    }
}

impl AttributeInfo for DeprecatedAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        unimplemented!()
    }
}

pub struct SyntheticAttribute(MakerAttribute);

impl SyntheticAttribute {
    pub fn new() -> SyntheticAttribute {
        return SyntheticAttribute(MakerAttribute{});
    }
}

impl AttributeInfo for SyntheticAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        unimplemented!()
    }
}