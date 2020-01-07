pub struct BytecodeReader {
    code:Vec<u8>,
    pc:i32
}

impl BytecodeReader {
    #[inline]
    pub const fn new() -> BytecodeReader {
        return BytecodeReader{ code: vec![], pc: 0 };
    }

    #[inline]
    pub fn reset(&mut self,code:Vec<u8>, pc:i32) {
        self.code = code;
        self.pc = pc;
    }

    pub fn read_u8(&mut self) -> u8 {
        let num = self.code.get(self.pc as usize);
        self.pc += 1;
        return *num.unwrap();
    }

    pub fn read_i8(&mut self) -> i8 {
        return self.read_u8() as i8;
    }
}