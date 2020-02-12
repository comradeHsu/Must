use crate::class_file::attribute_info::Attribute::*;
use crate::class_file::class_reader::ClassReader;
use crate::class_file::code_attribute::CodeAttribute;
use crate::class_file::constant_pool::ConstantPool;
use crate::class_file::constant_value_attribute::ConstantValueAttribute;
use crate::class_file::enclosing_method_attribute::EnclosingMethodAttribute;
use crate::class_file::exceptions_attribute::ExceptionsAttribute;
use crate::class_file::inner_classes_attribute::InnerClassesAttribute;
use crate::class_file::line_number_table_attribute::LineNumberTableAttribute;
use crate::class_file::local_variable_table_attribute::LocalVariableTableAttribute;
use crate::class_file::local_variable_type_table_attribute::LocalVariableTypeTableAttribute;
use crate::class_file::makers_attribute::{DeprecatedAttribute, SyntheticAttribute};
use crate::class_file::runtime_visible_annotations_attribute::AnnotationsAttribute;
use crate::class_file::signature_attribute::SignatureAttribute;
use crate::class_file::source_file_attribute::SourceFileAttribute;
use crate::class_file::stack_map_table_attribute::StackMapAttribute;
use crate::class_file::unparsed_attribute::UnparsedAttribute;
use std::cell::RefCell;
use std::rc::Rc;

pub trait AttributeInfo {
    fn read_info(&mut self, reader: &mut ClassReader);
}

pub fn read_attributes(reader: &mut ClassReader, cp: Rc<RefCell<ConstantPool>>) -> Vec<Attribute> {
    let clone = reader.data.clone();
    let attr_count = reader.read_u16();
    let mut attributes = Vec::new();
    for _i in 0..attr_count {
        attributes.push(read_attribute(reader, cp.clone()));
    }
    return attributes;
}

pub fn read_attribute(reader: &mut ClassReader, cp: Rc<RefCell<ConstantPool>>) -> Attribute {
    let attr_name_index = reader.read_u16();
    let clone = cp.clone();
    let borrow_clone = (*clone).borrow();
    let attr_name = borrow_clone.get_utf8(attr_name_index as usize);
    let attr_len = reader.read_u32();
    let mut info = new(attr_name, attr_len, cp);

    info.read_info(reader);
    return info;
}

pub fn new(attr_name: &str, attr_len: u32, cp: Rc<RefCell<ConstantPool>>) -> Attribute {
    let info: Attribute = match attr_name {
        "Code" => Code(CodeAttribute::with_cp(cp)),
        "ConstantValue" => ConstantValue(ConstantValueAttribute::new()),
        "Deprecated" => Deprecated(DeprecatedAttribute::new()),
        "Exceptions" => Exceptions(ExceptionsAttribute::new()),
        "LineNumberTable" => LineNumberTable(LineNumberTableAttribute::new()),
        "LocalVariableTable" => LocalVariableTable(LocalVariableTableAttribute::new()),
        "SourceFile" => SourceFile(SourceFileAttribute::with_cp(cp)),
        "Synthetic" => Synthetic(SyntheticAttribute::new()),
        //        "StackMapTable" => StackMap(StackMapAttribute::new(attr_len)),
        "RuntimeVisibleAnnotations" => RuntimeVisibleAnnotations(AnnotationsAttribute::with_cp(cp)),
        _ => Unparsed(UnparsedAttribute::new(attr_len)),
    };
    return info;
}

pub enum Attribute {
    Unparsed(UnparsedAttribute),
    SourceFile(SourceFileAttribute),
    StackMap(StackMapAttribute),
    Signature(SignatureAttribute),
    Deprecated(DeprecatedAttribute),
    Synthetic(SyntheticAttribute),
    LocalVariableTypeTable(LocalVariableTypeTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    LineNumberTable(LineNumberTableAttribute),
    InnerClasses(InnerClassesAttribute),
    Exceptions(ExceptionsAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    RuntimeVisibleAnnotations(AnnotationsAttribute),
}

impl Attribute {
    #[inline]
    pub fn read_info(&mut self, reader: &mut ClassReader) {
        match self {
            Code(attr) => attr.read_info(reader),
            ConstantValue(attr) => attr.read_info(reader),
            EnclosingMethod(attr) => attr.read_info(reader),
            Exceptions(attr) => attr.read_info(reader),
            InnerClasses(attr) => attr.read_info(reader),
            LineNumberTable(attr) => attr.read_info(reader),
            LocalVariableTable(attr) => attr.read_info(reader),
            LocalVariableTypeTable(attr) => attr.read_info(reader),
            Synthetic(attr) => attr.read_info(reader),
            Deprecated(attr) => attr.read_info(reader),
            Signature(attr) => attr.read_info(reader),
            StackMap(attr) => attr.read_info(reader),
            SourceFile(attr) => attr.read_info(reader),
            Unparsed(attr) => attr.read_info(reader),
            RuntimeVisibleAnnotations(attr) => attr.read_info(reader),
        }
    }
}
