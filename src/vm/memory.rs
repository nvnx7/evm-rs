use hex;
use std::fmt;

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // Stores a word (32-byte) in memory
    pub fn store(&mut self, offset: usize, value: &[u8; 32]) {
        self.write(offset, value);
    }

    // Stores a byte in memory
    pub fn store8(&mut self, offset: usize, value: u8) {
        self.write(offset, &[value]);
    }

    // Loads a word (32-byte) from memory
    pub fn load(&mut self, offset: usize) -> Vec<u8> {
        self.expand(offset, 32);
        (&self.data[offset..(offset + 32)]).to_vec()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    // Stores a value in memory
    pub fn write(&mut self, offset: usize, value: &[u8]) {
        self.expand(offset, value.len());
        self.data
            .splice(offset..(offset + value.len()), value.iter().cloned());
    }

    // Untouched memory expansion in 32 byte steps
    fn expand(&mut self, offset: usize, size: usize) {
        if offset + size - 1 >= self.data.len() {
            let r = (offset + size) % 32;
            if r == 0 {
                self.data.resize(offset + size, 0);
            } else {
                self.data.resize(offset + size + 32 - r, 0);
            }
        }
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.data))
    }
}

#[cfg(test)]
mod test {
    use super::Memory;

    #[test]
    fn store8() {
        let mut mem = Memory::new();
        mem.store8(0, 44);
        let mut res = vec![0u8; 32];
        res[0] = 44;
        assert_eq!(mem.load(0), res);
    }

    #[test]
    fn store() {
        let mut mem = Memory::new();
        mem.store(0, &[42; 32]);
        assert_eq!(mem.load(0), vec![42u8; 32]);
    }

    #[test]
    fn store_expansion() {
        let mut mem = Memory::new();
        mem.store(0, &[42; 32]);
        assert_eq!(mem.size(), 32);

        mem.store(8, &[99; 32]);
        assert_eq!(mem.size(), 64);

        mem.store8(56, 33);
        assert_eq!(mem.size(), 64);
        mem.store(31, &[55; 32]);
        assert_eq!(mem.size(), 64);
        mem.store(32, &[90; 32]);
        assert_eq!(mem.size(), 64);
    }

    #[test]
    fn load_expansion() {
        let mut mem = Memory::new();
        assert_eq!(mem.load(20), vec![0; 32]);
        assert_eq!(mem.size(), 64);
        mem.load(31);
        assert_eq!(mem.size(), 64);
        mem.load(63);
        assert_eq!(mem.size(), 96);
        mem.load(100);
        assert_eq!(mem.size(), 160);
    }
}
