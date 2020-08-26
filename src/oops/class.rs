use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::Jvm;
use crate::oops::access_flags::{
    ABSTRACT, ANNOTATION, ENUM, FINAL, INTERFACE, PUBLIC, SUPER, SYNTHETIC,
};
use crate::oops::array_object::ArrayObject;
use crate::oops::class_name_helper::PrimitiveTypes;
use crate::oops::constant_pool::{Constant, ConstantPool};
use crate::oops::field::Field;
use crate::oops::method::Method;
use crate::oops::object::DataType::{
    Bytes, Chars, Doubles, Floats, Ints, Longs, References, Shorts,
};
use crate::oops::object::{MetaData, Object};
use crate::oops::slots::Slots;
use crate::oops::string_pool::StringPool;
use crate::utils::boxed;
use lark_classfile::attribute_info::Attribute::RuntimeVisibleAnnotations;
use lark_classfile::class_file::ClassFile;
use lark_classfile::runtime_visible_annotations_attribute::AnnotationAttribute;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Weak, RwLock};
use std::string::ToString;
use crate::utils::raw_ptr::RawPtr;

pub type Interfaces = Vec<Class>;

struct Raw {
    access_flags: u16,
    name: String,
    super_class_name: Option<String>,
    interfaces_name: Vec<String>,
    constant_pool: ConstantPool,
    fields: Vec<Field>,
    methods: Vec<Method>,
    loader: Option<ClassLoader>,
    super_class: Option<Class>,
    interfaces: Option<Interfaces>,
    instance_slot_count: u32,
    static_slot_count: u32,
    static_vars: Option<Slots>,
    initialized: bool,
    java_class: Option<Object>,
    source_file: Option<String>,
    annotations: Option<Vec<AnnotationAttribute>>,
}

impl Default for Raw {
    fn default() -> Self {
        return Raw {
            access_flags: 0,
            name: "".to_string(),
            super_class_name: None,
            interfaces_name: vec![],
            constant_pool: ConstantPool::default(),
            fields: vec![],
            methods: vec![],
            loader: None,
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None,
            initialized: false,
            java_class: None,
            source_file: None,
            annotations: None,
        };
    }
}

#[derive(Clone)]
pub struct Class {
    raw: Arc<RawPtr<Raw>>
}

impl Class {
    #[inline]
    pub fn new(class_file: ClassFile) -> Class {
        let super_name = class_file.super_class_name();
        let class = Raw {
            access_flags: class_file.access_flags(),
            name: class_file.class_name().to_string(),
            super_class_name: super_name,
            interfaces_name: class_file.interface_names(),
            constant_pool: ConstantPool::new_constant_pool(None, class_file.constant_pool()),
            fields: vec![],
            methods: vec![],
            loader: None,
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None,
            initialized: false,
            java_class: None,
            source_file: Self::get_source_file(&class_file),
            annotations: Class::copy_annotations(&class_file),
        };
        let point = Arc::new(RawPtr::new(class));
        let weak_class = WeakClass {
            raw: Arc::downgrade(&point)
        };
        {
            let raw = point.write();
            raw.constant_pool.set_class(weak_class.clone());
            raw.methods = Method::new_methods(weak_class.clone(), class_file.methods());
            raw.fields = Field::new_fields(weak_class, class_file.fields());
        }
        return Class { raw: point }
    }

    fn get_source_file(class_file: &ClassFile) -> Option<String> {
        let attr = class_file.source_file_attribute();
        if attr.is_some() {
            return Some(attr.unwrap().file_name());
        }
        return None;
    }

    ///copy annotations info for class
    fn copy_annotations(class: &ClassFile) -> Option<Vec<AnnotationAttribute>> {
        let attributes = class.attributes();
        for attribute in attributes {
            match attribute {
                RuntimeVisibleAnnotations(attr) => {
                    let clone = attr.annotations().to_vec();
                    return Some(clone);
                }
                _ => {}
            }
        }
        return None;
    }

    #[inline]
    pub fn new_array_class(loader: &ClassLoader, class_name: &str) -> Class {
        let mut interfaces = Vec::new();
        let bootstrap_loader = Jvm::boot_class_loader();
        interfaces.push(
            bootstrap_loader
                .find_or_create("java/lang/Cloneable")
                .unwrap(),
        );
        interfaces.push(
            bootstrap_loader
                .find_or_create("java/io/Serializable")
                .unwrap(),
        );
        let raw = Raw {
            access_flags: PUBLIC,
            name: class_name.to_string(),
            super_class_name: Some("java/lang/Object".to_string()),
            interfaces_name: vec![],
            constant_pool: ConstantPool::default(),
            fields: vec![],
            methods: vec![],
            loader: Some(loader.clone()),
            super_class: Some(bootstrap_loader.find_or_create("java/lang/Object").unwrap()),
            interfaces: Some(interfaces),
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None,
            initialized: true,
            java_class: None,
            source_file: None,
            annotations: None,
        };
        return Class {
            raw: Arc::new(RawPtr::new(raw))
        };
    }

    #[inline]
    pub fn primitive_class(class_name: &str) -> Class {
        let boot_loader = Jvm::boot_class_loader().basic_loader();
        let raw = Raw {
            access_flags: PUBLIC,
            name: class_name.to_string(),
            super_class_name: None,
            interfaces_name: vec![],
            constant_pool: ConstantPool::default(),
            fields: vec![],
            methods: vec![],
            loader: Some(boot_loader),
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None,
            initialized: true,
            java_class: None,
            source_file: None,
            annotations: None,
        };
        return Class {
            raw: Arc::new(RawPtr::new(raw))
        };
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        let raw = self.raw.read();
        return 0 != raw.access_flags & PUBLIC;
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        let raw = self.raw.read();
        return 0 != raw.access_flags & FINAL;
    }

    #[inline]
    pub fn is_super(&self) -> bool {
        let raw = self.raw.read();
        return 0 != raw.access_flags & SUPER;
    }

    #[inline]
    pub fn is_interface(&self) -> bool {
        let raw = self.raw.read();
        return 0 != raw.access_flags & INTERFACE;
    }

    #[inline]
    pub fn is_abstract(&self) -> bool {
        let raw = self.raw.read();
        return 0 != raw.access_flags & ABSTRACT;
    }

    #[inline]
    pub fn is_synthetic(&self) -> bool {
        let raw = self.raw.read();
        return 0 != raw.access_flags & SYNTHETIC;
    }

    #[inline]
    pub fn is_annotation(&self) -> bool {
        let raw = self.raw.read();
        return 0 != raw.access_flags & ANNOTATION;
    }

    #[inline]
    pub fn is_enum(&self) -> bool {
        let raw = self.raw.read();
        return 0 != raw.access_flags & ENUM;
    }

    pub fn is_accessible_to(&self, other: &Self) -> bool {
        return self.is_public() || self.package_name() == other.package_name();
    }

    pub fn package_name(&self) -> String {
        let raw = self.raw.read();
        let index = raw.name.rfind('/');
        let name = match index {
            Some(seq) => {
                let (package, _) = raw.name.split_at(seq);
                package
            }
            None => "",
        };
        return name.to_owned();
    }

    // self extends c
    pub fn is_sub_class_of(&self, other: &Self) -> bool {
        let raw = self.raw.read();
        let mut super_class = raw.super_class.clone();
        while super_class.is_some() {
            let rc = super_class.unwrap();
            if other == &rc {
                return true;
            }
            super_class = rc.super_class();
        }
        return false;
    }

    pub fn is_assignable_from(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }
        if !other.is_array() {
            if !other.is_interface() {
                if !self.is_interface() {
                    return other.is_sub_class_of(self);
                } else {
                    return other.is_implements(self);
                }
            } else {
                if !self.is_interface() {
                    return self.is_java_lang_object();
                } else {
                    return self.is_sub_interface_of(other);
                }
            }
        } else {
            if !self.is_array() {
                if !self.is_interface() {
                    return self.is_java_lang_object();
                } else {
                    return self.is_java_lang_cloneable() || self.is_java_io_serializable();
                }
            } else {
                let sc = other.component_class();
                let tc = self.component_class();
                return sc == tc || tc.is_assignable_from(&sc);
            }
        }
        return false;
    }

    // self implements interface
    pub fn is_implements(&self, interface: &Self) -> bool {
        let raw = self.raw.read();
        let cur_interfaces = raw.interfaces.as_ref();
        if cur_interfaces.is_some() {
            for si in cur_interfaces.unwrap() {
                if si == interface
                    || si.is_sub_interface_of(interface)
                {
                    return true;
                }
            }
        }
        let mut super_class = raw.super_class.clone();
        while super_class.is_some() {
            let rc = super_class.unwrap();
            let rc_raw = rc.raw.read();
            let interfaces = rc_raw.interfaces.as_ref();
            if interfaces.is_some() {
                for i in interfaces.unwrap() {
                    if i == interface
                        || i.is_sub_interface_of(interface)
                    {
                        return true;
                    }
                }
            }
            super_class = rc_raw.super_class.clone();
        }
        return false;
    }

    //    #[inline]
    //    fn current_implement(interfaces:Option<&Interfaces>, interface: &Self) -> bool {
    //        if interfaces.is_some() {
    //            for i in interfaces.unwrap() {
    //                let interface_class = (*i).borrow();
    //                if interface_class.deref() == interface || interface_class.is_sub_interface_of(interface) {
    //                    return true;
    //                }
    //            }
    //        }
    //        return false;
    //    }

    ///
    pub fn is_sub_interface_of(&self, other: &Self) -> bool {
        let raw = self.raw.read();
        let interfaces = raw.interfaces.as_ref();
        if interfaces.is_some() {
            for interface in interfaces.unwrap() {
                if interface == other
                    || interface.is_sub_interface_of(other)
                {
                    return true;
                }
            }
        }
        return false;
    }

    // c extends self
    pub fn is_super_class_of(&self, other: &Self) -> bool {
        return other.is_sub_class_of(self);
    }

    pub fn get_main_method(&self) -> Option<Method> {
        let method = self.find_method("main","([Ljava/lang/String;)V",
                                      MethodType::Static);
        return method;
    }

    #[inline]
    pub fn is_java_lang_object(&self) -> bool {
        let raw = self.raw.read();
        return raw.name.as_str() == "java/lang/Object";
    }

    #[inline]
    pub fn is_java_lang_cloneable(&self) -> bool {
        let raw = self.raw.read();
        return raw.name.as_str() == "java/lang/Cloneable";
    }

    #[inline]
    pub fn is_java_io_serializable(&self) -> bool {
        let raw = self.raw.read();
        return raw.name.as_str() == "java/io/Serializable";
    }

    pub fn get_field(
        &self,
        name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Option<Field> {
        let inner = self.raw.read();
        for field in &inner.fields {
            if field.parent().is_static() == is_static
                && field.name() == name
                && field.descriptor() == descriptor
            {
                return Some(field.clone());
            }
        }
        let mut class_ptr = inner.super_class.clone();
        while class_ptr.is_some() {
            let class = class_ptr.unwrap();
            let class_raw = class.raw.read();
            for field in &class_raw.fields {
                if field.parent().is_static() == is_static
                    && field.name() == name
                    && field.descriptor() == descriptor
                {
                    return Some(field.clone());
                }
            }
            class_ptr = class_raw.super_class.clone();
        }
        return None;
    }

    /// find method in this class, inherited method not to find
    pub fn find_method(
        &self,
        name: &str,
        descriptor: &str,
        category: MethodType,
    ) -> Option<Method> {
        let predicate = |method:&Method| -> bool {
            match &category {
                MethodType::Static => {
                    return method.is_static()
                        && method.name() == name
                        && method.descriptor() == descriptor
                },
                MethodType::Instance => {
                    return !method.is_static()
                        && method.name() == name
                        && method.descriptor() == descriptor
                },
                MethodType::Unlimited => {
                    return method.name() == name
                        && method.descriptor() == descriptor
                }
            }
        };
        let raw = self.raw.read();
        for method in &raw.methods {
            if predicate(method) {
                return Some(method.clone())
            }
        }
        None
    }

    pub fn get_method(
        &self,
        name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Option<Method> {
        let method_type = match is_static {
            true => MethodType::Static,
            false => MethodType::Instance
        };
        if let Some(m) = self.find_method(name,descriptor,method_type) {
            return Some(m)
        }
        let mut class_ptr = self.super_class();
        while class_ptr.is_some() {
            let class = class_ptr.unwrap();
            if let Some(m) = class.find_method(name,descriptor,method_type) {
                return Some(m)
            }
            class_ptr = class.super_class();
        }
        return None;
    }

    #[inline]
    pub fn new_object(class: &Class) -> Object {
        return Object::new(class);
    }

    #[inline]
    pub fn new_class_loader_object(class: &Class) -> Object {
        let object = Object::new(class);
        object.set_meta_data(MetaData::ClassLoader(ClassLoader::with_verbose(
            false,
        )));
        return object;
    }

    #[inline]
    pub fn set_class_loader(&self, class_loader: ClassLoader) {
        let mut raw = self.raw.write();
        raw.loader = Some(class_loader);
    }

    #[inline]
    pub fn set_super_class(&self, super_class: Class) {
        let mut raw = self.raw.write();
        raw.super_class = Some(super_class);
    }

    #[inline]
    pub fn set_interfaces(&self, interfaces: Interfaces) {
        let mut raw = self.raw.write();
        raw.interfaces = Some(interfaces);
    }

    #[inline]
    pub fn set_instance_slot_count(&self, count: u32) {
        let mut raw = self.raw.write();
        raw.instance_slot_count = count;
    }

    #[inline]
    pub fn set_static_slot_count(&self, count: u32) {
        let mut raw = self.raw.write();
        raw.static_slot_count = count;
    }

    #[inline]
    pub fn set_static_vars(&self, vars: Slots) {
        let mut raw = self.raw.write();
        raw.static_vars = Some(vars);
    }

    #[inline]
    pub fn name(&self) -> String {
        let raw = self.raw.read();
        return raw.name.clone();
    }

    #[inline]
    pub fn super_class_name(&self) -> Option<String> {
        let raw = self.raw.read();
        return raw.super_class_name.clone();
    }

    #[inline]
    pub fn interfaces_name_with<F,R>(&self, fun: F) -> R
        where F: FnOnce(&Vec<String>) -> R
    {
        let raw = self.raw.read();
        fun(&raw.interfaces_name)
    }

    #[inline]
    pub fn loader(&self) -> ClassLoader {
        let raw = self.raw.read();
        let loader = raw.loader.as_ref().unwrap();
        return loader.clone();
    }

    #[inline]
    pub fn java_class(&self) -> Option<Object> {
        let raw = self.raw.read();
        return raw.java_class.clone();
    }

    #[inline]
    pub fn get_java_class(&self) -> Option<Object> {
        let raw = self.raw.read();
        return raw.java_class.clone();
    }

    #[inline]
    pub fn set_java_class(&self, object: Option<Object>) {
        let mut raw = self.raw.write();
        raw.java_class = object;
    }

    #[inline]
    pub fn super_class(&self) -> Option<Class> {
        let raw = self.raw.read();
        if raw.super_class.is_some() {
            return raw.super_class.clone();
        }
        return None;
    }

    #[inline]
    pub fn instance_slot_count(&self) -> u32 {
        let raw = self.raw.read();
        return raw.instance_slot_count;
    }

    #[inline]
    pub fn static_slot_count(&self) -> u32 {
        let raw = self.raw.read();
        return raw.static_slot_count;
    }

    #[inline]
    pub fn fields_with<F,R>(&self, fun: F) -> R
        where F: FnOnce(& Vec<Field>) -> R
    {
        let raw = self.raw.read();
        fun(&raw.fields)
    }

    #[inline]
    pub fn interfaces_with<F,R>(&self, fun: F) -> R
    where F: FnOnce(Option<&Interfaces>) -> R
    {
        let raw = self.raw.read();
        fun(raw.interfaces.as_ref())
    }

    pub fn constant_with<F,R>(&self, index:usize, func: F) -> R
        where F: FnOnce(&Constant) -> R
    {
        let raw = self.raw.read();
        let constant = raw.constant_pool.get_constant_immutable(index);
        func(constant)
    }

    #[inline]
    pub fn initialized(&self) -> bool {
        let raw = self.raw.read();
        return raw.initialized;
    }

    #[inline]
    pub fn set_initialized(&self) {
        let mut raw = self.raw.write();
        raw.initialized = true;
    }

    #[inline]
    pub fn mut_fields_with<F,R>(&self, fun: F) -> R
    where F: FnOnce(&mut Vec<Field>) -> R
    {
        let mut raw = self.raw.write();
        fun(&mut raw.fields)
    }

    /// for static vars operation
    ///
    #[inline]
    pub fn get_static_var<F,R>(&self, fun: F) -> R
        where F: FnOnce(&Slots) -> R
    {
        let mut raw = self.raw.write();
        fun(raw.static_vars.as_ref().unwrap())
    }

    #[inline]
    pub fn get_static_long(&self, slot_id: usize) -> i64 {
        self.get_static_var(|slots| slots.get_long(slot_id))
    }

    #[inline]
    pub fn get_static_int(&self, slot_id: usize) -> i32 {
        self.get_static_var(|slots| slots.get_int(slot_id))
    }

    #[inline]
    pub fn get_static_float(&self, slot_id: usize) -> f32 {
        self.get_static_var(|slots| slots.get_float(slot_id))
    }

    #[inline]
    pub fn get_static_double(&self, slot_id: usize) -> f64 {
        self.get_static_var(|slots| slots.get_double(slot_id))
    }

    #[inline]
    pub fn get_static_ref(&self, slot_id: usize) -> Option<Object> {
        self.get_static_var(|slots| slots.get_ref(slot_id))
    }

    #[inline]
    pub fn set_static_var<F,R>(&self, fun: F) -> R
        where F: FnOnce(&mut Slots) -> R
    {
        let mut raw = self.raw.write();
        fun(raw.static_vars.as_mut().unwrap())
    }

    #[inline]
    pub fn set_static_long(&self, slot_id: usize, val: i64) {
        self.set_static_var(|slots| slots.set_long(slot_id,val))
    }

    #[inline]
    pub fn set_static_int(&self, slot_id: usize, val: i32) {
        self.set_static_var(|slots| slots.set_int(slot_id,val))
    }

    #[inline]
    pub fn set_static_float(&self, slot_id: usize, val: f32)  {
        self.set_static_var(|slots| slots.set_float(slot_id,val))
    }

    #[inline]
    pub fn set_static_double(&self, slot_id: usize, val: f64) {
        self.set_static_var(|slots| slots.set_double(slot_id,val))
    }

    #[inline]
    pub fn set_static_ref(&self, slot_id: usize, val: Option<Object>) {
        self.set_static_var(|slots| slots.set_ref(slot_id,val))
    }

    #[inline]
    pub fn get_clinit_method(&self) -> Option<Method> {
        return self.get_static_method("<clinit>", "()V");
    }

    #[inline]
    pub fn get_static_method(
        &self,
        name: &str,
        desc: &str,
    ) -> Option<Method> {
        return self.get_method(name, desc, true);
    }

    pub fn get_instance_method(
        &self,
        name: &str,
        desc: &str,
    ) -> Option<Method> {
        return self.get_method(name, desc, false);
    }

    #[inline]
    pub fn java_name(&self) -> String {
        let raw = self.raw.read();
        let string = raw.name.replace('/', ".");
        return string;
    }

    pub fn is_primitive(&self) -> bool {
        let raw = self.raw.read();
        let primitive = PrimitiveTypes::instance()
            .unwrap()
            .primitive_types()
            .get(raw.name.as_str());
        return primitive.is_some();
    }

    pub fn set_static_ref_var(
        &self,
        name: &str,
        descriptor: &str,
        reference: Option<Object>,
    ) {
        let field = self.get_field(name, descriptor, true);
        self.set_static_ref(field.unwrap().slot_id(), reference);
    }

    pub fn get_static_ref_var(
        &self,
        name: &str,
        descriptor: &str,
    ) -> Option<Object> {
        let field = self.get_field(name, descriptor, true);
        return self.get_static_ref(field.unwrap().slot_id());
    }

    #[inline]
    pub fn source_file(&self) -> String {
        let raw = self.raw.read();
        if raw.source_file.is_none() {
            return "Unknown".to_string();
        }
        return raw.source_file.clone().unwrap();
    }

    #[inline]
    pub fn access_flags(&self) -> u16 {
        let raw = self.raw.read();
        return raw.access_flags;
    }

    #[inline]
    pub fn get_constructor(&self, descriptor: &str) -> Option<Method> {
        return self.get_instance_method( "<init>", descriptor);
    }

    pub fn get_constructors(&self, public_only: bool) -> Vec<Method> {
        let raw = self.raw.read();
        let mut constructors = Vec::with_capacity(raw.methods.len());
        for method in &raw.methods {
            if method.is_constructor() {
                if !public_only || method.is_public() {
                    constructors.push(method.clone());
                }
            }
        }
        return constructors;
    }

    pub fn get_fields(&self, public_only: bool) -> Vec<Field> {
        let raw = self.raw.read();
        if public_only {
            let mut public_fields = Vec::with_capacity(raw.fields.len());
            for field in &raw.fields {
                if field.is_public() {
                    public_fields.push(field.clone());
                }
            }
            return public_fields;
        } else {
            return raw.fields.clone();
        }
    }

    pub fn get_methods(&self, public_only: bool) -> Vec<Method> {
        let raw = self.raw.read();
        let mut methods = Vec::with_capacity(raw.methods.len());
        for method in &raw.methods {
            if !method.is_clinit() && !method.is_constructor() {
                if !public_only || method.is_public() {
                    methods.push(method.clone());
                }
            }
        }
        return methods;
    }

    #[inline]
    pub fn is_class_loader(&self) -> bool {
        let raw = self.raw.read();
        let mut super_class = raw.super_class.clone();
        while super_class.is_some() {
            let rc = super_class.unwrap();
            if rc.name().as_str() == "java/lang/ClassLoader" {
                return true;
            }
            let rc_raw = rc.raw.read();
            super_class = rc_raw.super_class.clone();
        }
        return false;
    }

    ///about array's class
    /// like int[]
    #[inline]
    pub fn new_array(&self, count: usize) -> ArrayObject {
        if !self.is_array() {
            panic!("Not array class: {}", self.name());
        }
        let raw = self.raw.read();
        match raw.name.as_str() {
            "[Z" | "[B" => ArrayObject::from_data(self, Bytes(vec![0; count])),
            "[C" => ArrayObject::from_data(self, Chars(vec![0; count])),
            "[S" => ArrayObject::from_data(self, Shorts(vec![0; count])),
            "[I" => ArrayObject::from_data(self, Ints(vec![0; count])),
            "[J" => ArrayObject::from_data(self, Longs(vec![0; count])),
            "[F" => ArrayObject::from_data(self, Floats(vec![0f32; count])),
            "[D" => ArrayObject::from_data(self, Doubles(vec![0f64; count])),
            _ => ArrayObject::from_data(self, References(vec![None; count])),
        }
    }

    pub fn is_array(&self) -> bool {
        let raw = self.raw.read();
        return raw.name.starts_with('[');
    }

    pub fn array_class(&self) -> Class {
        let raw = self.raw.read();
        let array_class_name = PrimitiveTypes::instance()
            .unwrap()
            .get_array_class_name(raw.name.as_str());
        let class_loader = self.get_class_loader();
        return ClassLoader::load_class(class_loader, array_class_name.as_str());
    }

    pub fn component_class(&self) -> Class {
        let raw = self.raw.read();
        let component_class_name = PrimitiveTypes::instance()
            .unwrap()
            .get_component_class_name(raw.name.as_str());
        let class_loader = self.get_class_loader();
        return ClassLoader::load_class(class_loader, component_class_name.as_str());
    }

    pub fn get_class_loader(&self) -> Option<Object> {
        let java_class = self.get_java_class();
        if java_class.is_none() {
            return None;
        }
        return java_class
            .unwrap()
            .get_ref_var("classLoader", "Ljava/lang/ClassLoader;");
    }

    pub fn init_static_final_variable(&self, field: &Field) {
        let copy = |index:usize| -> Constant {
            let raw = self.raw.read();
            raw.constant_pool.copy_constant(index)
        };
        let cp_index = field.const_value_index();
        let slot_id = field.slot_id();
        if cp_index > 0 {
            match field.parent().descriptor() {
                "Z" | "B" | "C" | "S" | "I" => {
                    let val = copy(cp_index);
                    match val {
                        Constant::Integer(v) => {
                            self.set_static_var(|vars|vars.set_int(slot_id,v))
                        },
                        _ => {}
                    }
                }
                "J" => {
                    let val = copy(cp_index);
                    match val {
                        Constant::Long(v) => {
                            self.set_static_var(|vars|vars.set_long(slot_id,v))
                        },
                        _ => {}
                    }
                }
                "F" => {
                    let val = copy(cp_index);
                    match val {
                        Constant::Float(v) => {
                            self.set_static_var(|vars|vars.set_float(slot_id,v))
                        },
                        _ => {}
                    }
                }
                "D" => {
                    let val = copy(cp_index);
                    match val {
                        Constant::Double(v) => {
                            self.set_static_var(|vars|vars.set_double(slot_id,v))
                        },
                        _ => {}
                    }
                }
                "Ljava/lang/String;" => {
                    let val = copy(cp_index);
                    let mete_str = match val {
                        Constant::Str(v) => v,
                        _ => panic!("It's not string"),
                    };
                    let java_string = StringPool::java_string(mete_str);
                    self.set_static_var(|vars|vars.set_ref(slot_id,Some(java_string)))
                }
                _ => {}
            }
        }
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        if self.name() == other.name() {
            return true;
        }
        return false;
    }
}

#[derive(Clone, Copy)]
pub enum MethodType {
    Static,
    Instance,
    Unlimited
}

#[derive(Clone)]
pub struct WeakClass {
    raw: Weak<RawPtr<Raw>>
}

impl WeakClass {

    pub fn upgrade(&self) -> Class {
        let raw = self.raw.upgrade();
        assert!(raw.is_some(),"The Class is dropped");
        return Class{
            raw: raw.unwrap()
        }
    }

}

impl Default for WeakClass {
    fn default() -> Self {
        return WeakClass {
            raw: Weak::new()
        }
    }
}
