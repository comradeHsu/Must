use crate::oops::object::Object;


/// invoke java method of parameters
pub struct Parameters {
    parameters: Vec<Parameter>,
}

impl Parameters {
    #[inline]
    pub fn new() -> Parameters {
        return Parameters { parameters: vec![] };
    }

    #[inline]
    pub fn with_parameters(parameters: Vec<Parameter>) -> Parameters {
        return Parameters { parameters };
    }

    #[inline]
    pub fn size(&self) -> usize {
        return self.parameters.len();
    }

    #[inline]
    pub fn get_parameter(&self, index: usize) -> &Parameter {
        return self
            .parameters
            .get(index)
            .expect("The Parameters hasn't parameter");
    }

    #[inline]
    pub fn append_parameter(&mut self, parameter: Parameter) {
        self.parameters.push(parameter)
    }
}

pub enum Parameter {
    Boolean(bool),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Char(char),
    Object(Option<Object>),
}
