pub struct BytecodeReader {
    code:Vec<u8>,
    pc:i32
}

impl BytecodeReader {
    #[inline]
    pub fn new() -> BytecodeReader {
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

    pub fn read_u16(&mut self) -> u16 {
        let mut bytes:[u8;2] = [0;2];
        bytes[0] = self.read_u8();
        bytes[1] = self.read_u8();
        return u16::from_be_bytes(bytes);
    }

    pub fn read_i16(&mut self) -> i16 {
        return self.read_u16() as i16;
    }

    pub fn read_i32(&mut self) -> i32 {
        let mut bytes:[u8;4] = [0;4];
        for i in 0..4usize {
            bytes[i] = self.read_u8();
        }
        return i32::from_be_bytes(bytes);
    }

    pub fn read_i32_table(&mut self) {

    }

    pub fn skip_padding(&mut self) {

    }
}