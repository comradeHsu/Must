use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;
use crate::class_file::constant_pool::ConstantPool;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SourceFileAttribute {
    cp: Rc<RefCell<ConstantPool>>,
    source_file_index: u16,
}

impl SourceFileAttribute {
    pub fn new() -> SourceFileAttribute {
        return SourceFileAttribute {
            cp: Rc::new(RefCell::new(ConstantPool::new())),
            source_file_index: 0,
        };
    }

    pub fn with_cp(cp: Rc<RefCell<ConstantPool>>) -> SourceFileAttribute {
        return SourceFileAttribute {
            cp,
            source_file_index: 0,
        };
    }

    pub fn file_name(&self) -> String {
        let cp = self.cp.borrow();
        return cp.get_utf8(self.source_file_index as usize).to_owned();
    }
}

impl AttributeInfo for SourceFileAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.source_file_index = reader.read_u16();
    }
}
