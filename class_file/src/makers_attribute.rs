use crate::attribute_info::AttributeInfo;
use crate::class_reader::ClassReader;

pub struct MakerAttribute {}

pub struct DeprecatedAttribute(MakerAttribute);

impl DeprecatedAttribute {
    pub fn new() -> DeprecatedAttribute {
        return DeprecatedAttribute(MakerAttribute {});
    }
}

impl AttributeInfo for DeprecatedAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {}
}

pub struct SyntheticAttribute(MakerAttribute);

impl SyntheticAttribute {
    pub fn new() -> SyntheticAttribute {
        return SyntheticAttribute(MakerAttribute {});
    }
}

impl AttributeInfo for SyntheticAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {
        unimplemented!()
    }
}
