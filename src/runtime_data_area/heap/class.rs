use crate::class_file::attribute_info::Attribute::RuntimeVisibleAnnotations;
use crate::class_file::class_file::ClassFile;
use crate::class_file::member_info::MemberInfo;
use crate::class_file::runtime_visible_annotations_attribute::AnnotationAttribute;
use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::Jvm;
use crate::runtime_data_area::heap::access_flags::{
    AccessFlag, ABSTRACT, ANNOTATION, ENUM, FINAL, INTERFACE, PUBLIC, SUPER, SYNTHETIC,
};
use crate::runtime_data_area::heap::array_object::ArrayObject;
use crate::runtime_data_area::heap::class_name_helper::PrimitiveTypes;
use crate::runtime_data_area::heap::constant_pool::{ConstantPool, Constant};
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::object::DataType::{
    Bytes, Chars, Doubles, Floats, Ints, Longs, References, Shorts,
};
use crate::runtime_data_area::heap::object::{MetaData, Object};
use crate::runtime_data_area::heap::slots::Slots;
use crate::runtime_data_area::slot::Slot;
use crate::utils::boxed;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::runtime_data_area::heap::string_pool::StringPool;

pub type Interfaces = Vec<Rc<RefCell<Class>>>;

#[derive(Debug)]
pub struct Class {
    access_flags: u16,
    name: String,
    super_class_name: Option<String>,
    interfaces_name: Vec<String>,
    constant_pool: ConstantPool,
    fields: Vec<Rc<RefCell<Field>>>,
    methods: Vec<Rc<Method>>,
    loader: Option<Rc<RefCell<ClassLoader>>>,
    super_class: Option<Rc<RefCell<Class>>>,
    interfaces: Option<Interfaces>,
    instance_slot_count: u32,
    static_slot_count: u32,
    static_vars: Option<Slots>,
    initialized: bool,
    java_class: Option<Rc<RefCell<Object>>>,
    source_file: Option<String>,
    annotations: Option<Vec<AnnotationAttribute>>,
}

impl Class {
    #[inline]
    pub fn none() -> Class {
        return Class {
            access_flags: 0,
            name: "".to_string(),
            super_class_name: None,
            interfaces_name: vec![],
            constant_pool: ConstantPool::none(),
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

    #[inline]
    pub fn new(class_file: ClassFile) -> Rc<RefCell<Class>> {
        let super_name = class_file.super_class_name();
        let mut class = Class {
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
        //        println!("class:{:?}",class.name.as_str());
        let mut point = Rc::new(RefCell::new(class));

        (*point)
            .borrow_mut()
            .constant_pool
            .set_class(point.clone());

        (*point)
            .borrow_mut()
            .constant_pool
            .lazy_init_for_constants(&point);

        (*point).borrow_mut().methods = Method::new_methods(point.clone(), class_file.methods());
        (*point).borrow_mut().fields = Field::new_fields(point.clone(), class_file.fields());
        return point;
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
    pub fn new_array_class(loader: Rc<RefCell<ClassLoader>>, class_name: &str) -> Class {
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
        let class = Class {
            access_flags: PUBLIC,
            name: class_name.to_string(),
            super_class_name: Some("java/lang/Object".to_string()),
            interfaces_name: vec![],
            constant_pool: ConstantPool::none(),
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
        return class;
    }

    #[inline]
    pub fn primitive_class(class_name: &str) -> Class {
        let boot_loader = Jvm::boot_class_loader().basic_loader();
        return Class {
            access_flags: PUBLIC,
            name: class_name.to_string(),
            super_class_name: None,
            interfaces_name: vec![],
            constant_pool: ConstantPool::none(),
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
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        return 0 != self.access_flags & PUBLIC;
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        return 0 != self.access_flags & FINAL;
    }

    #[inline]
    pub fn is_super(&self) -> bool {
        return 0 != self.access_flags & SUPER;
    }

    #[inline]
    pub fn is_interface(&self) -> bool {
        return 0 != self.access_flags & INTERFACE;
    }

    #[inline]
    pub fn is_abstract(&self) -> bool {
        return 0 != self.access_flags & ABSTRACT;
    }

    #[inline]
    pub fn is_synthetic(&self) -> bool {
        return 0 != self.access_flags & SYNTHETIC;
    }

    #[inline]
    pub fn is_annotation(&self) -> bool {
        return 0 != self.access_flags & ANNOTATION;
    }

    #[inline]
    pub fn is_enum(&self) -> bool {
        return 0 != self.access_flags & ENUM;
    }

    pub fn is_accessible_to(&self, other: &Self) -> bool {
        return self.is_public() || self.package_name() == other.package_name();
    }

    pub fn package_name(&self) -> &str {
        let index = self.name.rfind('/');
        let name = match index {
            Some(seq) => {
                let (package, _) = self.name.split_at(seq);
                package
            }
            None => "",
        };
        return name;
    }

    // self extends c
    pub fn is_sub_class_of(&self, other: &Self) -> bool {
        let mut super_class = self.super_class.clone();
        while super_class.is_some() {
            let rc = super_class.unwrap();
            let rc_super_class = (*rc).borrow();
            if other == rc_super_class.deref() {
                return true;
            }
            super_class = rc_super_class.super_class.clone();
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
                return sc == tc || (*tc).borrow().is_assignable_from((*sc).borrow().deref());
            }
        }
        return false;
    }

    // self implements interface
    pub fn is_implements(&self, interface: &Self) -> bool {
        let cur_interfaces = self.interfaces.as_ref();
        if cur_interfaces.is_some() {
            for i in cur_interfaces.unwrap() {
                let interface_class = (*i).borrow();
                if interface_class.deref() == interface
                    || interface_class.is_sub_interface_of(interface)
                {
                    return true;
                }
            }
        }
        let mut super_class = self.super_class.clone();
        while super_class.is_some() {
            let rc = super_class.unwrap();
            let ref_class = (*rc).borrow();
            let interfaces = ref_class.interfaces.as_ref();
            if interfaces.is_some() {
                for i in interfaces.unwrap() {
                    let interface_class = (*i).borrow();
                    if interface_class.deref() == interface
                        || interface_class.is_sub_interface_of(interface)
                    {
                        return true;
                    }
                }
            }
            super_class = ref_class.super_class.clone();
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
        let interfaces = self.interfaces.as_ref();
        if interfaces.is_some() {
            for interface in interfaces.unwrap() {
                let interface = interface.clone();
                if (*interface).borrow().deref() == other
                    || (*interface).borrow().is_sub_interface_of(other)
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

    pub fn get_main_method(&self) -> Option<Rc<Method>> {
        for method in self.methods() {
            let method = method.clone();
            if method.name() == "main" && method.descriptor() == "([Ljava/lang/String;)V" {
                return Some(method);
            }
        }
        return None;
    }

    #[inline]
    pub fn is_java_lang_object(&self) -> bool {
        return self.name.as_str() == "java/lang/Object";
    }

    #[inline]
    pub fn is_java_lang_cloneable(&self) -> bool {
        return self.name.as_str() == "java/lang/Cloneable";
    }

    #[inline]
    pub fn is_java_io_serializable(&self) -> bool {
        return self.name.as_str() == "java/io/Serializable";
    }

    pub fn get_field(
        mut class_ptr: Option<Rc<RefCell<Class>>>,
        name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Option<Rc<RefCell<Field>>> {
        while class_ptr.is_some() {
            let class = class_ptr.unwrap();
            for field in (*class).borrow().fields() {
                let borrow = (**field).borrow();
                if borrow.parent().is_static() == is_static
                    && borrow.name() == name
                    && borrow.descriptor() == descriptor
                {
                    return Some(field.clone());
                }
            }
            class_ptr = (*class).borrow().super_class();
        }
        return None;
    }

    pub fn get_method(
        mut class_ptr: Option<Rc<RefCell<Class>>>,
        name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Option<Rc<Method>> {
        while class_ptr.is_some() {
            let class = class_ptr.unwrap();
            for field in (*class).borrow().methods() {
                if field.is_static() == is_static
                    && field.name() == name
                    && field.descriptor() == descriptor
                {
                    return Some(field.clone());
                }
            }
            class_ptr = (*class).borrow().super_class();
        }
        return None;
    }

    #[inline]
    pub fn new_object(class: &Rc<RefCell<Class>>) -> Object {
        return Object::new(class.clone());
    }

    #[inline]
    pub fn new_class_loader_object(class: &Rc<RefCell<Class>>) -> Object {
        let mut object = Object::new(class.clone());
        object.set_meta_data(MetaData::ClassLoader(boxed(ClassLoader::with_verbose(
            false,
        ))));
        return object;
    }

    #[inline]
    pub fn set_class_loader(&mut self, class_loader: Rc<RefCell<ClassLoader>>) {
        self.loader = Some(class_loader);
    }

    #[inline]
    pub fn set_super_class(&mut self, super_class: Rc<RefCell<Class>>) {
        self.super_class = Some(super_class);
    }

    #[inline]
    pub fn set_interfaces(&mut self, interfaces: Interfaces) {
        self.interfaces = Some(interfaces);
    }

    #[inline]
    pub fn set_instance_slot_count(&mut self, count: u32) {
        self.instance_slot_count = count;
    }

    #[inline]
    pub fn set_static_slot_count(&mut self, count: u32) {
        self.static_slot_count = count;
    }

    #[inline]
    pub fn set_static_vars(&mut self, vars: Slots) {
        self.static_vars = Some(vars);
    }

    #[inline]
    pub fn name(&self) -> &str {
        return self.name.as_str();
    }

    #[inline]
    pub fn super_class_name(&self) -> Option<&String> {
        return self.super_class_name.as_ref();
    }

    #[inline]
    pub fn interfaces_name(&self) -> &Vec<String> {
        return &self.interfaces_name;
    }

    #[inline]
    pub fn loader(&self) -> Rc<RefCell<ClassLoader>> {
        let loader = self.loader.as_ref().unwrap();
        return loader.clone();
    }

    #[inline]
    pub fn java_class(&self) -> Option<&Rc<RefCell<Object>>> {
        return self.java_class.as_ref();
    }

    #[inline]
    pub fn get_java_class(&self) -> Option<Rc<RefCell<Object>>> {
        return self.java_class.clone();
    }

    #[inline]
    pub fn set_java_class(&mut self, object: Option<Rc<RefCell<Object>>>) {
        return self.java_class = object;
    }

    #[inline]
    pub fn super_class(&self) -> Option<Rc<RefCell<Class>>> {
        if self.super_class.is_some() {
            return self.super_class.clone();
        }
        return None;
    }

    #[inline]
    pub fn instance_slot_count(&self) -> u32 {
        return self.instance_slot_count;
    }

    #[inline]
    pub fn static_slot_count(&self) -> u32 {
        return self.static_slot_count;
    }

    #[inline]
    pub fn fields(&self) -> &Vec<Rc<RefCell<Field>>> {
        return &self.fields;
    }

    #[inline]
    pub fn interfaces(&self) -> Option<&Interfaces> {
        return self.interfaces.as_ref();
    }

    #[inline]
    pub fn methods(&self) -> &Vec<Rc<Method>> {
        return &self.methods;
    }

    #[inline]
    pub fn constant_pool(&self) -> &ConstantPool {
        return &self.constant_pool;
    }

    #[inline]
    pub fn mut_constant_pool(&mut self) -> &mut ConstantPool {
        return &mut self.constant_pool;
    }

    #[inline]
    pub fn initialized(&self) -> bool {
        return self.initialized;
    }

    #[inline]
    pub fn set_initialized(&mut self) {
        self.initialized = true;
    }

    #[inline]
    pub fn mut_fields(&mut self) -> &mut Vec<Rc<RefCell<Field>>> {
        return &mut self.fields;
    }

    #[inline]
    pub fn mut_static_vars(&mut self) -> Option<&mut Slots> {
        return self.static_vars.as_mut();
    }

    #[inline]
    pub fn static_vars(&self) -> Option<&Slots> {
        return self.static_vars.as_ref();
    }

    #[inline]
    pub fn get_clinit_method(class: Rc<RefCell<Self>>) -> Option<Rc<Method>> {
        return Self::get_static_method(class, "<clinit>", "()V");
    }

    #[inline]
    pub fn get_static_method(
        class: Rc<RefCell<Class>>,
        name: &str,
        desc: &str,
    ) -> Option<Rc<Method>> {
        return Class::get_method(Some(class), name, desc, true);
    }

    pub fn get_instance_method(
        class: Rc<RefCell<Class>>,
        name: &str,
        desc: &str,
    ) -> Option<Rc<Method>> {
        return Class::get_method(Some(class), name, desc, false);
    }

    #[inline]
    pub fn java_name(&self) -> String {
        let string = self.name.replace('/', ".");
        return string;
    }

    pub fn is_primitive(&self) -> bool {
        let primitive = PrimitiveTypes::instance()
            .unwrap()
            .primitive_types()
            .get(self.name());
        return primitive.is_some();
    }

    pub fn set_static_ref_var(
        class: Rc<RefCell<Self>>,
        name: &str,
        descriptor: &str,
        reference: Option<Rc<RefCell<Object>>>,
    ) {
        let field = Class::get_field(Some(class.clone()), name, descriptor, true);
        let mut borrow = (*class).borrow_mut();
        let slots = borrow.mut_static_vars().unwrap();
        slots.set_ref((*field.unwrap()).borrow().slot_id(), reference);
    }

    pub fn get_static_ref_var(
        class: Rc<RefCell<Self>>,
        name: &str,
        descriptor: &str,
    ) -> Option<Rc<RefCell<Object>>> {
        let field = Class::get_field(Some(class.clone()), name, descriptor, true);
        let borrow = (*class).borrow();
        let slots = borrow.static_vars.as_ref().unwrap();
        return slots.get_ref((*field.unwrap()).borrow().slot_id());
    }

    #[inline]
    pub fn get_static_ref_by_slot_id(
        class: Rc<RefCell<Self>>,
        slot_id: usize,
    ) -> Option<Rc<RefCell<Object>>> {
        let borrow = (*class).borrow();
        return borrow
            .static_vars
            .as_ref()
            .map_or_else(|| None, |slots| slots.get_ref(slot_id));
    }

    #[inline]
    pub fn get_static_long_by_slot_id(class: Rc<RefCell<Self>>, slot_id: usize) -> i64 {
        let borrow = (*class).borrow();
        return borrow
            .static_vars
            .as_ref()
            .map_or_else(|| 0, |slots| slots.get_long(slot_id));
    }

    #[inline]
    pub fn set_static_long_by_slot_id(class: Rc<RefCell<Self>>, slot_id: usize, value: i64) {
        let mut borrow = (*class).borrow_mut();
        let static_vars = borrow.static_vars.as_mut().unwrap();
        static_vars.set_long(slot_id, value);
    }

    #[inline]
    pub fn source_file(&self) -> String {
        if self.source_file.is_none() {
            return "Unknown".to_string();
        }
        return self.source_file.clone().unwrap();
    }

    #[inline]
    pub fn access_flags(&self) -> u16 {
        return self.access_flags;
    }

    #[inline]
    pub fn get_constructor(class: Rc<RefCell<Class>>, descriptor: &str) -> Option<Rc<Method>> {
        return Self::get_instance_method(class, "<init>", descriptor);
    }

    pub fn get_constructors(&self, public_only: bool) -> Vec<Rc<Method>> {
        let mut constructors = Vec::with_capacity(self.methods.len());
        for method in &self.methods {
            if method.is_constructor() {
                if !public_only || method.is_public() {
                    constructors.push(method.clone());
                }
            }
        }
        return constructors;
    }

    pub fn get_fields(&self, public_only: bool) -> Vec<Rc<RefCell<Field>>> {
        if public_only {
            let mut public_fields = Vec::with_capacity(self.fields.len());
            for field in &self.fields {
                if (*field).borrow().is_public() {
                    public_fields.push(field.clone());
                }
            }
            return public_fields;
        } else {
            return self.fields.clone();
        }
    }

    pub fn get_methods(&self, public_only: bool) -> Vec<Rc<Method>> {
        let mut methods = Vec::with_capacity(self.methods.len());
        for method in &self.methods {
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
        let mut super_class = self.super_class.clone();
        while super_class.is_some() {
            let rc = super_class.unwrap();
            let rc_super_class = (*rc).borrow();
            if rc_super_class.name() == "java/lang/ClassLoader" {
                return true;
            }
            super_class = rc_super_class.super_class.clone();
        }
        return false;
    }

    ///about array's class
    /// like int[]
    #[inline]
    pub fn new_array(class: &Rc<RefCell<Class>>, count: usize) -> ArrayObject {
        if !(**class).borrow().is_array() {
            panic!("Not array class: {}", (**class).borrow().name());
        }
        match (**class).borrow().name() {
            "[Z" | "[B" => ArrayObject::from_data(class.clone(), Bytes(vec![0; count])),
            "[C" => ArrayObject::from_data(class.clone(), Chars(vec![0; count])),
            "[S" => ArrayObject::from_data(class.clone(), Shorts(vec![0; count])),
            "[I" => ArrayObject::from_data(class.clone(), Ints(vec![0; count])),
            "[J" => ArrayObject::from_data(class.clone(), Longs(vec![0; count])),
            "[F" => ArrayObject::from_data(class.clone(), Floats(vec![0f32; count])),
            "[D" => ArrayObject::from_data(class.clone(), Doubles(vec![0f64; count])),
            _ => ArrayObject::from_data(class.clone(), References(vec![None; count])),
        }
    }

    pub fn is_array(&self) -> bool {
        return self.name.starts_with('[');
    }

    pub fn array_class(&self) -> Rc<RefCell<Class>> {
        let array_class_name = PrimitiveTypes::instance()
            .unwrap()
            .get_array_class_name(self.name.as_str());
        let class_loader = self.get_class_loader();
        return ClassLoader::load_class(class_loader, array_class_name.as_str());
    }

    pub fn component_class(&self) -> Rc<RefCell<Class>> {
        let component_class_name = PrimitiveTypes::instance()
            .unwrap()
            .get_component_class_name(self.name.as_str());
        let class_loader = self.get_class_loader();
        return ClassLoader::load_class(class_loader, component_class_name.as_str());
    }

    fn get_class_loader(&self) -> Option<Rc<RefCell<Object>>> {
        let java_class = self.get_java_class();
        if java_class.is_none() {
            return None;
        }
        return (*java_class.unwrap())
            .borrow()
            .get_ref_var("classLoader", "Ljava/lang/ClassLoader;");
    }

    pub fn init_static_final_variable(&mut self, field: Rc<RefCell<Field>>) {
        let cp_index = (*field).borrow().const_value_index();
        let slot_id = (*field).borrow().slot_id();
        let vars = self.static_vars.as_mut().expect("static_vars is none");
        if cp_index > 0 {
            match (*field).borrow().parent().descriptor() {
                "Z" | "B" | "C" | "S" | "I" => {
                    let val = self.constant_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Integer(v) => vars.set_int(slot_id, *v),
                        _ => {}
                    }
                }
                "J" => {
                    let val = self.constant_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Long(v) => vars.set_long(slot_id, *v),
                        _ => {}
                    }
                }
                "F" => {
                    let val = self.constant_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Float(v) => vars.set_float(slot_id, *v),
                        _ => {}
                    }
                }
                "D" => {
                    let val = self.constant_pool.get_constant_immutable(cp_index);
                    match val {
                        Constant::Double(v) => vars.set_double(slot_id, *v),
                        _ => {}
                    }
                }
                "Ljava/lang/String;" => {
                    let val = self.constant_pool.get_constant_immutable(cp_index);
                    let mete_str = match val {
                        Constant::Str(v) => v.as_str(),
                        _ => panic!("It's not string"),
                    };
                    let java_string = StringPool::java_string(mete_str.to_string());
                    vars.set_ref(slot_id, Some(java_string));
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
