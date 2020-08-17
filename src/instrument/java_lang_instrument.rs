use crate::instrument::create_instrumentation;
use crate::oops::class::Class;
use crate::oops::method::Method;
use crate::oops::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub struct JavaLangInstrument {
    instrument: Object,
    transform_method: Rc<Method>,
}

static mut INSTRUMENT: Option<JavaLangInstrument> = None;

impl JavaLangInstrument {
    fn new() -> JavaLangInstrument {
        let desc = "(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/lang/Class;Ljava/security/ProtectionDomain;[BZ)[B";
        let instrument = create_instrumentation();
        let class = instrument.class();
        let method = Class::get_instance_method(class, "transform", desc);
        return JavaLangInstrument {
            instrument,
            transform_method: method.unwrap(),
        };
    }

    pub fn instance() -> &'static JavaLangInstrument {
        unsafe {
            if INSTRUMENT.is_none() {
                INSTRUMENT = Some(Self::new());
            }
            return INSTRUMENT.as_ref().unwrap();
        }
    }

    #[inline]
    pub fn get_transform_method(&self) -> Rc<Method> {
        return self.transform_method.clone();
    }

    #[inline]
    pub fn get_instrument(&self) -> Object {
        return self.instrument.clone();
    }
}
