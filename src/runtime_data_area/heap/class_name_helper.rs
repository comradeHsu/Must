use std::collections::HashMap;

static mut PRIMITIVE_TYPES: Option<PrimitiveTypes> = None;

pub struct PrimitiveTypes {
    data: HashMap<&'static str, &'static str>,
}

impl PrimitiveTypes {
    pub fn instance() -> Option<&'static PrimitiveTypes> {
        unsafe {
            if PRIMITIVE_TYPES.is_none() {
                PRIMITIVE_TYPES = Some(PrimitiveTypes::init());
            }
            return PRIMITIVE_TYPES.as_ref();
        }
    }

    fn init() -> PrimitiveTypes {
        let mut map = HashMap::new();
        map.insert("void", "V");
        map.insert("boolean", "Z");
        map.insert("byte", "B");
        map.insert("short", "S");
        map.insert("int", "I");
        map.insert("long", "J");
        map.insert("char", "C");
        map.insert("float", "F");
        map.insert("double", "D");
        return PrimitiveTypes { data: map };
    }

    pub fn get_array_class_name(&self, class_name: &str) -> String {
        return "[".to_string() + self.to_descriptor(class_name).as_str();
    }

    fn to_descriptor(&self, class_name: &str) -> String {
        if class_name.starts_with('[') {
            return class_name.to_string();
        }
        let rs = self.data.get(class_name);
        if rs.is_some() {
            return rs.unwrap().to_string();
        }
        return "L".to_string() + class_name + ";";
    }

    pub fn get_component_class_name(&self, class_name: &str) -> String {
        if class_name.starts_with('[') {
            let (_, component_type_descriptor) = class_name.split_at(1);
            return self.to_class_name(component_type_descriptor);
        }
        panic!("Not array: {}", class_name);
    }

    pub fn to_class_name(&self, descriptor: &str) -> String {
        if descriptor.starts_with('[') {
            // array
            return descriptor.to_string();
        }
        if descriptor.starts_with('L') {
            // object
            let (_, desc) = descriptor.split_at(1);
            let (target, _) = desc.split_at(desc.len() - 1);
            return target.to_string();
        }

        let data = &self.data;
        for (k, v) in data {
            if *v == descriptor {
                // primitive
                return k.to_string();
            }
        }

        panic!("Invalid descriptor: {}", descriptor);
    }

    #[inline]
    pub fn primitive_types(&self) -> &HashMap<&'static str, &'static str> {
        return &self.data;
    }
}
