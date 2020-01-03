use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

struct LocalVariableTypeTableAttribute  {
    local_variable_type_table:Vec<LocalVariableTypeTableEntry>
}

struct LocalVariableTypeTableEntry {
    start_pc:u16,
    length:u16,
    name_index:u16,
    signature_index:u16,
    index:u16
}

impl AttributeInfo for LocalVariableTypeTableAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        let local_variable_type_table_len = reader.read_u16();
        let mut local_variable_type_table = Vec::new();
        for _ in 0..local_variable_type_table_len {
            local_variable_type_table.push(LocalVariableTypeTableEntry{
                start_pc: reader.read_u16(),
                length: reader.read_u16(),
                name_index: reader.read_u16(),
                signature_index: reader.read_u16(),
                index: reader.read_u16()
            })
        }
        self.local_variable_type_table = local_variable_type_table;
    }
}