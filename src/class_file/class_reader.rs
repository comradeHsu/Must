
pub struct ClassReader {
    pub data:Vec<u8>
}

impl ClassReader {

    pub fn new(data:Vec<u8>) -> ClassReader {
        return ClassReader{data };
    }

    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        let val = self.data.remove(0usize);
        return val;
    }

    #[inline]
    pub fn read_char(&mut self) -> char {
        return self.read_u8() as char;
    }

    #[inline]
    pub fn read_u16(&mut self) -> u16 {
        let mut head = self.data.remove(0usize) as u16;
        let tail = self.data.remove(0usize) as u16;
        head = head << 8;
        return head | tail;
    }

    #[inline]
    pub fn read_u32(&mut self) -> u32 {
        let mut result = 0u32;
        for i in 0..4usize {
            let mut head = self.data.remove(0usize) as u32;
            head = head << (24 - (i as u32 * 8));
            result = result | head;
        }
        return result;
    }

    #[inline]
    pub fn read_u64(&mut self) -> u64 {
        let mut result = 0u64;
        for i in 0..8usize {
            let mut head = self.data.remove(0usize) as u64;
            head = head << (56 - (i as u64 * 8));
            result = result | head;
        }
        return result;
    }

    #[inline]
    pub fn read_u16_table(&mut self) -> Vec<u16> {
        let n = self.read_u16();
        let mut table = Vec::new();
        for _i in 0..n {
            table.push(self.read_u16());
        }
        return table;
    }

    #[inline]
    pub fn read_bytes(&mut self, n:usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        for _i in 0..n {
            bytes.push(self.data.remove(0usize));
        }
        return bytes;
    }
}