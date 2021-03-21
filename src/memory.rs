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
    pub(crate) fn new() -> Self {
        Self {
            bytes: [0; MEMORY_SIZE],
        }
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
