use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

struct LocalVariableTableAttribute {
    local_variable_table: Vec<LocalVariableTableEntry>
}

struct LocalVariableTableEntry  {
    start_pc:u16,
    length:u16,
    name_index:u16,
    descriptor_index:u16,
    index:u16,
}

impl LocalVariableTableAttribute {

}

impl AttributeInfo for LocalVariableTableAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let local_variable_table_len = reader.read_u16();
        let mut local_variable_table = Vec::new();
        for _ in 0..local_variable_table_len {
            local_variable_table.push(LocalVariableTableEntry{
                start_pc: reader.read_u16(),
                length: reader.read_u16(),
                name_index: reader.read_u16(),
                descriptor_index: reader.read_u16(),
                index: reader.read_u16()
            })
        }
        self.local_variable_table = local_variable_table;
    }
}