use crate::class_loader::app_class_loader::ClassLoader;
use crate::oops::class::{Class, WeakClass};
use crate::oops::class_member::ClassMember;
use crate::oops::class_name_helper::PrimitiveTypes;
use crate::oops::object::Object;
use lark_classfile::attribute_info::Attribute::RuntimeVisibleAnnotations;
use lark_classfile::member_info::MemberInfo;
use lark_classfile::runtime_visible_annotations_attribute::AnnotationAttribute;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;

pub struct MetaField {
    class_member: ClassMember,
    const_value_index: usize,
    slot_id: usize,
    annotations: Option<Vec<AnnotationAttribute>>,
}

impl MetaField {
    #[inline]
    pub fn new() -> MetaField {
        return MetaField {
            class_member: ClassMember::new(),
            const_value_index: 0,
            slot_id: 0,
            annotations: None,
        };
    }

    fn copy_const_attribute(&mut self, info: &MemberInfo) {
        let const_attr = info.constant_value_attr();
        if const_attr.is_some() {
            self.const_value_index = const_attr.unwrap().value_index() as usize;
        }
    }

    ///copy annotations info
    fn copy_annotations(&mut self, info: &MemberInfo) {
        let attributes = info.attributes();
        for attribute in attributes {
            match attribute {
                RuntimeVisibleAnnotations(attr) => {
                    let clone = attr.annotations().to_vec();
                    self.annotations = Some(clone)
                }
                _ => {}
            }
        }
    }

    #[inline]
    pub fn parent(&self) -> &ClassMember {
        return &self.class_member;
    }

    #[inline]
    pub fn const_value_index(&self) -> usize {
        return self.const_value_index;
    }

    #[inline]
    pub fn slot_id(&self) -> usize {
        return self.slot_id;
    }

    #[inline]
    pub fn name(&self) -> &str {
        return self.class_member.name();
    }

    #[inline]
    pub fn descriptor(&self) -> &str {
        return self.class_member.descriptor();
    }

    #[inline]
    fn set_slot_id(&mut self, slot_id: usize) {
        self.slot_id = slot_id;
    }

    #[inline]
    pub fn is_long_or_double(&self) -> bool {
        let descriptor = self.class_member.descriptor();
        return descriptor == "J" || descriptor == "D";
    }

    #[inline]
    pub fn is_accessible_to(&self, class: &Class) -> bool {
        return self.class_member.is_accessible_to(class);
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        return self.class_member.is_public();
    }

    // reflection
    pub fn r#type(&self) -> Class {
        let class_name = PrimitiveTypes::instance()
            .unwrap()
            .to_class_name(self.descriptor());
        let loader = self.get_class_loader();
        return ClassLoader::load_class(loader, class_name.as_str());
    }

    #[inline]
    pub fn access_flags(&self) -> u16 {
        return self.class_member.access_flags();
    }

    #[inline]
    pub fn signature(&self) -> &str {
        return self.class_member.signature();
    }

    fn get_class_loader(&self) -> Option<Object> {
        let class_object = self.class_member.class().get_java_class();
        if class_object.is_some() {
            return class_object
                .unwrap()
                .get_ref_var("classLoader", "Ljava/lang/ClassLoader;");
        }
        return None;
    }
}

#[derive(Clone)]
pub struct Field {
    meta_field: Arc<MetaField>
}

impl Field {

    pub fn new_fields(
        class: WeakClass,
        infos: &Vec<MemberInfo>,
    ) -> Vec<Field> {
        let mut fields = Vec::with_capacity(infos.len());
        for info in infos {
            let mut field = MetaField::new();
            field.class_member.set_class(class.clone());
            field.class_member.copy_member_info(info);
            field.copy_const_attribute(info);
            field.copy_annotations(info);
            fields.push(Field{ meta_field: Arc::new(field) });
        }
        return fields;
    }

    pub fn set_slot_id(&mut self, slot_id: usize) {
        let try_get = Arc::get_mut(&mut self.meta_field);
        assert!(try_get.is_some(),"this function just call once");
        try_get.unwrap().set_slot_id(slot_id)
    }

}

impl Deref for Field {
    type Target = MetaField;

    fn deref(&self) -> &Self::Target {
        self.meta_field.deref()
    }
}
