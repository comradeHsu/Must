use crate::runtime::frame::Frame;
use std::collections::HashMap;

pub type NativeMethod = fn(frame: &Frame);

pub struct Registry {
    methods: HashMap<String, NativeMethod>,
}

static mut REGISTRY: Option<Registry> = None;

impl Registry {
    #[inline]
    fn instance() -> &'static Registry {
        unsafe {
            if REGISTRY.is_none() {
                REGISTRY = Some(Registry {
                    methods: HashMap::new(),
                });
            }
            return REGISTRY.as_ref().unwrap();
        }
    }

    #[inline]
    fn mut_instance() -> &'static mut Registry {
        unsafe {
            if REGISTRY.is_none() {
                REGISTRY = Some(Registry {
                    methods: HashMap::new(),
                });
            }
            return REGISTRY.as_mut().unwrap();
        }
    }

    pub fn register(class_name: &str, method_name: &str, method_desc: &str, method: NativeMethod) {
        let key = class_name.to_string() + "_" + method_name + "_" + method_desc;
        Registry::mut_instance().methods.insert(key, method);
    }

    pub fn find_native_method(
        class_name: &str,
        method_name: &str,
        method_desc: &str,
    ) -> Option<NativeMethod> {
        let key = class_name.to_string() + "_" + method_name + "_" + method_desc;
        let result = Registry::instance().methods.get(&key);
        if result.is_some() {
            return Some(*result.unwrap());
        }
        if method_desc == "()V" && method_name == "registerNatives" {
            return Some(|_f| {});
        }
        return None;
    }
}

#[cfg(test)]
mod test {
    use crate::native::registry::Registry;
    use crate::oops::method::Method;
    use crate::runtime::frame::Frame;
    
    
    use std::rc::Rc;

    #[test]
    fn test_register() {
        Registry::register("java/lang/Object", "clone", "V", |_f| println!("clone"));
        Registry::register("java/lang/Object", "init", "V", |_f| println!("init"));
        let mut frame = Frame::new(Rc::new(Method::new()));
        let clone = Registry::find_native_method("java/lang/Object", "clone", "V").unwrap();
        clone(&mut frame);
        let clone_1 = Registry::find_native_method("java/lang/Object", "clone", "V").unwrap();
        clone_1(&mut frame);
    }
}
