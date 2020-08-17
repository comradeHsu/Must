use crate::attribute_info::Attribute::{Code, ConstantValue};
use crate::attribute_info::{read_attributes, Attribute};
use crate::class_reader::ClassReader;
use crate::code_attribute::CodeAttribute;
use crate::constant_pool::ConstantPool;
use crate::constant_value_attribute::ConstantValueAttribute;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MemberInfo {
    cp: Rc<RefCell<ConstantPool>>,
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl MemberInfo {
    pub fn read_member(reader: &mut ClassReader, cp: Rc<RefCell<ConstantPool>>) -> MemberInfo {
        let mut mem = MemberInfo {
            cp: cp.clone(),
            access_flags: reader.read_u16(),
            name_index: reader.read_u16(),
            descriptor_index: reader.read_u16(),
            attributes: vec![],
        };
        mem.attributes = read_attributes(reader, cp);
        return mem;
    }

    pub fn read_members(
        reader: &mut ClassReader,
        cp: Rc<RefCell<ConstantPool>>,
    ) -> Vec<MemberInfo> {
        let member_count = reader.read_u16();
        let mut members: Vec<MemberInfo> = Vec::new();
        for _i in 0..member_count {
            members.push(MemberInfo::read_member(reader, cp.clone()));
        }
        return members;
    }

    #[inline]
    pub fn access_flags(&self) -> u16 {
        return self.access_flags;
    }

    pub fn name(&self) -> String {
        let borrow = (*self.cp).borrow();
        return borrow.get_utf8(self.name_index as usize).to_owned();
    }

    pub fn descriptor(&self) -> String {
        let borrow = (*self.cp).borrow();
        return borrow.get_utf8(self.descriptor_index as usize).to_owned();
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        return &self.attributes;
    }

    pub fn code_attributes(&self) -> Option<&CodeAttribute> {
        for i in 0..self.attributes.len() {
            let attribute = &self.attributes[i];
            match attribute {
                Code(attr) => return Some(attr),
                _ => {}
            }
        }
        return None;
    }

    pub fn constant_value_attr(&self) -> Option<&ConstantValueAttribute> {
        for attribute in &self.attributes {
            match attribute {
                ConstantValue(attr) => return Some(attr),
                _ => {}
            }
        }
        return None;
    }
}

pub fn display_16(vec: Vec<u8>) -> String {
    let mut string = String::new();
    string.push_str("[");
    for v in vec {
        string.push_str(to_16(v).as_str());
        string.push_str("  ");
    }
    return string;
}

fn to_16(mut v: u8) -> String {
    let mut string = String::new();
    if v == 0 {
        string.push_str("00");
        return string;
    }
    let mut rem = 0;
    while v != 0 {
        rem = v % 16;
        v = v / 16;
        let s = match rem {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "A",
            11 => "B",
            12 => "C",
            13 => "D",
            14 => "E",
            15 => "F",
            _ => "",
        };
        string.insert_str(0, s);
    }
    if string.len() == 1 {
        string.insert_str(0, "0");
    }
    return string;
}
