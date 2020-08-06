#[derive(Debug)]
pub struct MethodDescriptor {
    parameter_types: Vec<String>,
    return_type: String,
}

impl MethodDescriptor {
    #[inline]
    pub fn new() -> MethodDescriptor {
        return MethodDescriptor {
            parameter_types: vec![],
            return_type: "".to_string(),
        };
    }

    #[inline]
    pub fn add_parameter_type(&mut self, string: String) {
        self.parameter_types.push(string);
    }

    #[inline]
    pub fn set_return_type(&mut self, string: String) {
        self.return_type = string;
    }

    #[inline]
    pub fn return_type(&self) -> &String {
        return &self.return_type;
    }

    #[inline]
    pub fn parameter_types(&self) -> &Vec<String> {
        return &self.parameter_types;
    }
}

pub struct MethodDescriptorParser<'a> {
    raw: String,
    offset: usize,
    parsed: &'a mut MethodDescriptor,
}

impl<'a> MethodDescriptorParser<'a> {
    pub fn parse_method_descriptor(descriptor: &str) -> MethodDescriptor {
        let mut method_desc = MethodDescriptor::new();
        let mut parser = MethodDescriptorParser {
            raw: "".to_string(),
            offset: 0,
            parsed: &mut method_desc,
        };
        parser.parse(descriptor);
        return method_desc;
    }

    fn parse(&mut self, descriptor: &str) {
        self.raw = descriptor.to_string();
        self.start_params();
        self.parse_param_types();
        self.end_params();
        self.parse_return_type();
        self.finish();
    }

    fn start_params(&mut self) {
        if self.read_u8() != '(' {
            self.cause_panic();
        }
    }

    fn end_params(&mut self) {
        if self.read_u8() != ')' {
            self.cause_panic();
        }
    }

    fn finish(&self) {
        if self.offset != self.raw.len() {
            self.cause_panic();
        }
    }

    fn cause_panic(&self) {
        panic!("Bad descriptor: {}", self.raw);
    }

    fn read_u8(&mut self) -> char {
        let bytes = self.raw.as_bytes();
        let b = bytes[self.offset] as char;
        self.offset += 1;
        return b;
    }

    fn unread_u8(&mut self) {
        self.offset -= 1;
    }

    fn parse_param_types(&mut self) {
        loop {
            let t = self.parse_field_type();
            if t.as_str() != "" {
                self.parsed.add_parameter_type(t.to_string());
            } else {
                break;
            }
        }
    }

    fn parse_return_type(&mut self) {
        if self.read_u8() == 'V' {
            self.parsed.set_return_type("V".to_string());
            return;
        }
        self.unread_u8();
        let t = self.parse_field_type();
        if t.as_str() != "" {
            self.parsed.set_return_type(t.to_string());
            return;
        }
        self.cause_panic();
    }

    fn parse_field_type(&mut self) -> String {
        match self.read_u8() {
            'B' => return "B".to_string(),
            'C' => return "C".to_string(),
            'D' => return "D".to_string(),
            'F' => return "F".to_string(),
            'I' => return "I".to_string(),
            'J' => return "J".to_string(),
            'S' => return "S".to_string(),
            'Z' => return "Z".to_string(),
            'L' => return self.parse_object_type().to_string(),
            '[' => return self.parse_array_type().to_string(),
            _ => {
                self.unread_u8();
                return "".to_string();
            }
        }
    }

    fn parse_object_type(&mut self) -> &str {
        let (_, unread) = self.raw.split_at(self.offset);
        let semicolon_index = unread.find(';');
        if semicolon_index.is_none() {
            self.cause_panic();
            return "";
        } else {
            let obj_start = self.offset - 1;
            let obj_end = self.offset + semicolon_index.unwrap() + 1;
            self.offset = obj_end;
            let descriptor: &str = self.raw.get(obj_start..obj_end).unwrap();
            return descriptor;
        }
    }

    fn parse_array_type(&mut self) -> &str {
        let arr_start = self.offset - 1;
        self.parse_field_type();
        let arr_end = self.offset;
        let descriptor: &str = self.raw.get(arr_start..arr_end).unwrap();
        return descriptor;
    }
}
