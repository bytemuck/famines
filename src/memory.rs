const MAX_MEMORY_SIZE: u16 = u16::MAX;

pub struct Memory {
    pub data: [u8; MAX_MEMORY_SIZE as usize],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; MAX_MEMORY_SIZE as usize],
        }
    }

    pub(crate) fn initialize(&mut self) {
        self.data = [0; MAX_MEMORY_SIZE as usize];
    }

    pub fn write_u16(&mut self, value: u32, address: u32, cycles: &mut u32) {
        self.data[address as usize]      = (value & 0xFF) as u8;
        self.data[address as usize + 1]  = (value >> 8) as u8;
        *cycles -= 2;
    }
}

impl std::ops::Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &<Self as std::ops::Index<u16>>::Output {
        &self.data[index as usize]
    }
}

impl std::ops::IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut <Self as std::ops::Index<u16>>::Output {
        &mut self.data[index as usize]
    }
}