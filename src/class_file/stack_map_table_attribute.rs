use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

pub struct StackMapAttribute {
    len:u32,
    entries:Vec<FrameType>
}

impl StackMapAttribute {
    pub fn new(len:u32) -> StackMapAttribute {
        return StackMapAttribute{ len: len, entries: Vec::new() };
    }
}

impl AttributeInfo for StackMapAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        let number_of_entries = reader.read_u16();
        let mut entries = Vec::new();
        let unit_count = ((self.len - 2) / (number_of_entries as u32)) as usize;
        for _i in 0..number_of_entries {
            entries.push(FrameType{ frame_type: reader.read_bytes(unit_count) });
        }
    }
}

struct FrameType {
    frame_type:Vec<u8>
}