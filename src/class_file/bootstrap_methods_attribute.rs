use crate::class_file::attribute_info::AttributeInfo;
use crate::class_file::class_reader::ClassReader;

pub struct BootstrapMethodsAttribute {
    bootstrap_methods: Vec<BootstrapMethod>,
}

struct BootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

impl AttributeInfo for BootstrapMethodsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let num_bootstrap_methods = reader.read_u16();
        let mut bootstrap_methods = Vec::new();
        for _ in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod {
                bootstrap_method_ref: reader.read_u16(),
                bootstrap_arguments: reader.read_u16_table(),
            })
        }
        self.bootstrap_methods = bootstrap_methods;
    }
}
