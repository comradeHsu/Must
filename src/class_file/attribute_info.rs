use crate::class_file::class_reader::ClassReader;
use crate::class_file::constant_pool::{ConstantPool, get_utf8};
use crate::class_file::code_attribute::CodeAttribute;
use std::rc::Rc;
use crate::class_file::constant_value_attribute::ConstantValueAttribute;
use crate::class_file::makers_attribute::{DeprecatedAttribute, SyntheticAttribute};
use crate::class_file::exceptions_attribute::ExceptionsAttribute;
use crate::class_file::line_number_table_attribute::LineNumberTableAttribute;
use crate::class_file::local_variable_table_attribute::LocalVariableTableAttribute;
use crate::class_file::source_file_attribute::SourceFileAttribute;
use crate::class_file::unparsed_attribute::UnparsedAttribute;
use crate::class_file::member_info::display_16;
use crate::class_file::stack_map_table_attribute::StackMapAttribute;

pub trait AttributeInfo {

    fn read_info(&mut self, reader:&mut ClassReader);

}

pub fn read_attributes(reader:&mut ClassReader,cp:Rc<ConstantPool>) -> Vec<Box<dyn AttributeInfo>> {
    let attr_count = reader.read_u16();
    let mut attributes = Vec::new();
    println!("attr_count:{}",attr_count);
    for _ in 0..attr_count {
        attributes.push(read_attribute(reader,cp.clone()));
    }
    return attributes;
}

pub fn read_attribute(reader:&mut ClassReader,cp:Rc<ConstantPool>) -> Box<dyn AttributeInfo> {
    let attr_name_index = reader.read_u16();
    println!("attr_name_index:{}",attr_name_index);
    if attr_name_index == 10496 {
        println!("data:{:?}",reader.data.clone());
    }
    let attr_name = get_utf8(cp.clone(),attr_name_index as usize);
    println!("attr_name:{}",attr_name);
//    println!("reader_data:{}",display_16(reader.data.clone()));
    let attr_len = reader.read_u32();
    println!("attr_len:{}",attr_len);
    let mut info = new(attr_name,attr_len,cp);
    info.read_info(reader);
    return info;
}

pub fn new(attr_name:&str,attr_len:u32,cp:Rc<ConstantPool>) -> Box<dyn AttributeInfo> {
    let info:Box<dyn AttributeInfo> = match attr_name {
        "Code" => Box::new(CodeAttribute::with_cp(cp)),
        "ConstantValue" => Box::new(ConstantValueAttribute::new()),
        "Deprecated" => Box::new(DeprecatedAttribute::new()),
        "Exceptions" => Box::new(ExceptionsAttribute::new()),
        "LineNumberTable" => Box::new(LineNumberTableAttribute::new()),
        "LocalVariableTable" => Box::new(LocalVariableTableAttribute::new()),
        "SourceFile" => Box::new(SourceFileAttribute::with_cp(cp)),
        "Synthetic" => Box::new(SyntheticAttribute::new()),
        "StackMapTable" => Box::new(StackMapAttribute::new()),
        _ => Box::new(UnparsedAttribute::new(attr_len))
    };
    return info;
}