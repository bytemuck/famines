use crate::*;

const MEMORY_SIZE: usize = 0xFFFF;

#[derive(Copy, Clone)]
pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bytes: [0; MEMORY_SIZE],
        }
    }

    pub fn get_byte(&self, address: Word) -> Byte {
        self[address]
    }

    pub fn write_byte(&mut self, value: u8, address: u16) {
        self[address] = value;
    }

    pub fn write_word(&mut self, value: u16, address: u16) {
        self[address] = (value & 0xFF) as u8;
        self[address + 1] = (value >> 8) as u8;
    }
}

impl std::ops::Index<Word> for Memory {
    type Output = Byte;

    fn index(&self, index: Word) -> &<Self as std::ops::Index<Word>>::Output {
        &self.bytes[index as usize]
    }
}

impl std::ops::IndexMut<Word> for Memory {
    fn index_mut(&mut self, index: Word) -> &mut <Self as std::ops::Index<Word>>::Output {
        &mut self.bytes[index as usize]
    }
}
