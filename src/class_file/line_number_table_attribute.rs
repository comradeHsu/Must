use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

struct LineNumberTableAttribute {
    line_number_table:Vec<LineNumberTableEntry>
}

struct LineNumberTableEntry {
    start_pc:u16,
    line_number:u16
}

impl LineNumberTableAttribute {
    pub fn get_line_number(&self,pc:u16) -> i32 {
        for i in self.line_number_table.len()..0 {
            let entry = self.line_number_table.get(i).unwrap();
            if pc >= entry.start_pc {
                return entry.line_number as i32;
            }
        }
        return -1;
    }
}

impl AttributeInfo for LineNumberTableAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let line_number_table_len = reader.read_u32();
        let mut line_number_table = Vec::new();
        for _ in 0..line_number_table_len {
            line_number_table.push(LineNumberTableEntry{
                start_pc: reader.read_u16(),
                line_number: reader.read_u16()
            })
        }
        self.line_number_table = line_number_table;
    }
}