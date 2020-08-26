use crate::oops::class::Class;
use crate::oops::class_ref::ClassRef;
use crate::oops::constant_pool::Constant::ClassReference;
use crate::oops::constant_pool::ConstantPool;
use lark_classfile::code_attribute::ExceptionTableEntry;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct ExceptionTable {
    table: Vec<ExceptionHandler>,
    holder: Option<Class>,
}

impl ExceptionTable {
    pub fn new(entries: &Vec<ExceptionTableEntry>, class: &Class) -> ExceptionTable {
        let mut table = Vec::with_capacity(entries.len());
        for entry in entries {
            table.push(ExceptionHandler {
                start_pc: entry.start_pc() as i32,
                end_pc: entry.end_pc() as i32,
                handler_pc: entry.handler_pc() as i32,
                catch_type: Self::get_catch_type(entry.catch_type() as usize, class),
            });
        }
        return ExceptionTable {
            table,
            holder: Some(class.clone()),
        };
    }

    fn get_catch_type(index: usize, class: &Class) -> Option<ClassRef> {
        if index == 0 {
            return None;
        }
        class.constant_with(index,|constant|{
            let class_ref = match constant {
                ClassReference(r) => r.clone(),
                _ => panic!("this constant isn't ClassReference"),
            };
            return Some(class_ref);
        })
    }

    pub fn find_exception_handler(
        &self,
        class: &Class,
        pc: i32,
    ) -> Option<&ExceptionHandler> {
        for handler in &self.table {
            if pc >= handler.start_pc && pc < handler.end_pc {
                if handler.catch_type.is_none() {
                    return Some(handler);
                }
                let mut class_ref = handler.catch_type.clone().unwrap();
                let holder = self.holder.as_ref().unwrap().clone();
                let catch_class = class_ref.resolved_class(&holder);
                if &catch_class == class
                    || catch_class
                        .is_super_class_of(class)
                {
                    return Some(handler);
                }
            }
        }
        return None;
    }
}

impl Default for ExceptionTable {
    fn default() -> Self {
        return ExceptionTable {
            table: vec![],
            holder: None,
        };
    }
}

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
