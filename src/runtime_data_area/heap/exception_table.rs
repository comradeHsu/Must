use crate::class_file::code_attribute::ExceptionTableEntry;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::class_ref::ClassRef;
use crate::runtime_data_area::heap::constant_pool::Constant::ClassReference;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct ExceptionTable {
    table: Vec<ExceptionHandler>,
}

impl ExceptionTable {
    #[inline]
    pub fn none() -> ExceptionTable {
        return ExceptionTable { table: vec![] };
    }

    pub fn new(
        entries: &Vec<ExceptionTableEntry>,
        pool: &ConstantPool,
    ) -> ExceptionTable {
        let mut table = Vec::with_capacity(entries.len());
        for entry in entries {
            table.push(ExceptionHandler {
                start_pc: entry.start_pc() as i32,
                end_pc: entry.end_pc() as i32,
                handler_pc: entry.handler_pc() as i32,
                catch_type: Self::get_catch_type(entry.catch_type() as usize, pool),
            });
        }
        return ExceptionTable { table };
    }

    fn get_catch_type(index: usize, pool: &ConstantPool) -> Option<ClassRef> {
        if index == 0 {
            return None;
        }
        let constant = pool.get_constant_immutable(index);
        let class_ref = match constant {
            ClassReference(r) => r.clone(),
            _ => panic!("this constant isn't ClassReference"),
        };
        return Some(class_ref);
    }

    pub fn find_exception_handler(
        &self,
        class: Rc<RefCell<Class>>,
        pc: i32,
    ) -> Option<&ExceptionHandler> {
        for handler in &self.table {
            if pc >= handler.start_pc && pc < handler.end_pc {
                if handler.catch_type.is_none() {
                    return Some(handler);
                }
                let mut class_ref = handler.catch_type.clone().unwrap();
                let catch_class = class_ref.resolved_class();
                if catch_class == class
                    || (*catch_class)
                        .borrow()
                        .is_super_class_of((*class).borrow().deref())
                {
                    return Some(handler);
                }
            }
        }
        return None;
    }
}

#[derive(Debug)]
pub struct ExceptionHandler {
    start_pc: i32,
    end_pc: i32,
    handler_pc: i32,
    catch_type: Option<ClassRef>,
}

impl ExceptionHandler {
    #[inline]
    pub fn handler_pc(&self) -> i32 {
        return self.handler_pc;
    }
}
