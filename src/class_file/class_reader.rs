use podio::BigEndian;

pub struct ClassReader {
    data:Vec<u8>
}

impl ClassReader {
    pub fn read_u8(&mut self) -> u8 {
        let val = self.data.remove(0usize);
        return val;
    }

    pub fn read_u16(&mut self) -> u16 {
        let head = self.data.remove(0usize) as u16;
        let tail = self.data.remove(0usize) as u16;
        return head | tail;
    }

    pub fn read_u32(&mut self) -> u32 {
        let head = self.data.remove(0usize) as u32;
        let mid = self.data.remove(0usize) as u32;
        let tail = self.data.remove(0usize) as u32;
        let temp = head | mid;
        return temp | tail;
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut result = 0u64;
        for i in 0..4usize {
            let head = self.data.remove(0usize) as u64;
            result = result | head;
        }
        return result;
    }

    pub fn read_u16_table(&mut self) -> Vec<u16> {
        let n = self.read_u16();
        let mut table = Vec::new();
        for _i in 0..n {
            table.push(self.read_u16());
        }
        return table;
    }

    pub fn read_bytes(&mut self, n:usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        for _i in 0..n {
            bytes.push(self.data.remove(0usize));
        }
        return bytes;
    }
}