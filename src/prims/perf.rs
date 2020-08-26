use crate::invoke_support::parameter::{Parameter, Parameters};
use crate::invoke_support::{JavaCall, ReturnType};

use crate::jvm::Jvm;
use crate::oops::class::Class;
use crate::oops::object::Object;
use crate::prims::perf_data::{PerfDataManager, Units, Variability};
use crate::utils::{jstr_to_utf_nullable};
use std::mem::size_of;

pub struct Perf();

impl Perf {
    pub fn create_long(
        _perf: Option<Object>,
        name: Option<Object>,
        variability: i32,
        units: i32,
        value: i64,
    ) -> Option<Object> {
        if units <= 0 || units > Units::Hertz as i32 {
            println!("unexpected units argument, units = {}", units);
        }
        let name_str = jstr_to_utf_nullable(name);
        if PerfDataManager::get_instance().exists(name_str.as_str()) {
            panic!("PerfLong name already exists");
        }
        let pl = match Variability::from(variability) {
            Variability::Constant => PerfDataManager::get_mut_instance().create_long_constant(
                name_str.as_str(),
                Units::from(units),
                value,
            ),
            Variability::Monotonic => PerfDataManager::get_mut_instance().create_long_counter(
                name_str.as_str(),
                Units::from(units),
                value,
            ),
            Variability::Variable => PerfDataManager::get_mut_instance().create_long_variable(
                name_str.as_str(),
                Units::from(units),
                value,
            ),
        };
        let pointer = pl.get_address();
        return Self::new_direct_byte_buffer(pointer, size_of::<i64>());
    }

    fn new_direct_byte_buffer(pointer: usize, size_long: usize) -> Option<Object> {
        assert_ne!(size_long, 0, "size value {}", size_long);
        let boot_loader = Jvm::boot_class_loader();
        let class = boot_loader
            .find_or_create("java/nio/DirectByteBuffer")
            .unwrap();
        let method = class.get_instance_method("<init>", "(JI)V");
        let this = Class::new_object(&class);
        let param = vec![
            Parameter::Object(Some(this.clone())),
            Parameter::Long(pointer as i64),
            Parameter::Int(size_long as i32),
        ];
        JavaCall::invoke(
            method.unwrap(),
            Some(Parameters::with_parameters(param)),
            ReturnType::Void,
        );
        return Some(this);
    }
}
