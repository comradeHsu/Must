pub mod java_lang_instrument;

use crate::class_loader::class_loader::ClassLoader;
use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{invoke, ReturnType};
use crate::jvm::Jvm;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

const CONSTRUCTOR_DESC: &str = "(JZZ)V";

pub fn create_instrumentation() -> Rc<RefCell<Object>> {
    let class = ClassLoader::load_class(
        Jvm::instance().unwrap().boot_class_loader(),
        "sun.instrument.InstrumentationImpl",
    );
    let constructor = Class::get_constructor(class, CONSTRUCTOR_DESC);
    let parameters = vec![
        Parameter::Long(0),
        Parameter::Boolean(false),
        Parameter::Boolean(false),
    ];
    let instrument = invoke(
        constructor.unwrap(),
        Parameters::with_parameters(parameters),
        ReturnType::Object,
    )
    .object();
    return instrument.unwrap();
}
