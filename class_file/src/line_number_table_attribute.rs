use crate::attribute_info::AttributeInfo;
use crate::class_reader::ClassReader;
use std::ptr;

#[derive(Debug)]
pub struct LineNumberTableAttribute {
    line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Debug)]
struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl LineNumberTableAttribute {
    pub fn new() -> LineNumberTableAttribute {
        return LineNumberTableAttribute {
            line_number_table: vec![],
        };
    }

    pub fn get_line_number(&self, pc: u16) -> i32 {
        let len = self.line_number_table.len();
        for i in 0..len {
            let entry = self.line_number_table.get(len - 1 - i).unwrap();
            if pc >= entry.start_pc {
                return entry.line_number as i32;
            }
        }
        return -1;
    }

    pub fn unsafe_copy(&self) -> LineNumberTableAttribute {
        unsafe {
            let count = self.line_number_table.len();
            let ptr = self.line_number_table.as_ptr();
            let mut data = Vec::with_capacity(count);
            ptr::copy_nonoverlapping(ptr, data.as_mut_ptr(), count);
            data.set_len(count);
            return LineNumberTableAttribute {
                line_number_table: data,
            };
        }
    }
}

impl AttributeInfo for LineNumberTableAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let line_number_table_len = reader.read_u16();
        let mut line_number_table = Vec::new();
        for _ in 0..line_number_table_len {
            line_number_table.push(LineNumberTableEntry {
                start_pc: reader.read_u16(),
                line_number: reader.read_u16(),
            })
        }
        self.line_number_table = line_number_table;
    }
}

#[cfg(test)]
mod line {
    use crate::line_number_table_attribute::{LineNumberTableAttribute, LineNumberTableEntry};

    #[test]
    fn test_unsafe_copy() {
        let mut line_number_table = Vec::new();
        for i in 0..10 {
            line_number_table.push(LineNumberTableEntry {
                start_pc: i,
                line_number: i,
            })
        }
        let src = LineNumberTableAttribute { line_number_table };
        let copy = &src.unsafe_copy();
        let v1 = copy.line_number_table[8].start_pc;
        let v2 = src.line_number_table[8].start_pc;
        assert_eq!(v1, v2);
    }
}
