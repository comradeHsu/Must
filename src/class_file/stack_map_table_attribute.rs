use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

pub struct StackMapAttribute {
    entries:Vec<FrameType>
}

impl StackMapAttribute {
    pub fn new() -> StackMapAttribute {
        return StackMapAttribute{entries: vec![] };
    }
}

impl AttributeInfo for StackMapAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
//        self.name_index = reader.read_u16();
        let number_of_entries = reader.read_u16();
        let mut entries = Vec::new();
        for _i in 0..number_of_entries {
            entries.push(FrameType{ frame_type: reader.read_u16() });
        }
    }
}

struct FrameType {
    frame_type:u16
}