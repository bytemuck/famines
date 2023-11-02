use crate::memory::Address;
use crate::memory::Byte;
use crate::memory::Memory;
use std::ops::{Deref, DerefMut};

pub struct RAM {
    pub bytes: [u8; 0x800],
}

impl RAM {
    pub fn new() -> Self {
        Self { bytes: [0; 0x800] }
    }
}

impl Deref for RAM {
    type Target = [u8; 0x800];

    fn deref(&self) -> &[u8; 0x800] {
        &self.bytes
    }
}

impl DerefMut for RAM {
    fn deref_mut(&mut self) -> &mut [u8; 0x800] {
        &mut self.bytes
    }
}

impl Memory for RAM {
    fn read_byte(&mut self, address: Address) -> Byte {
        self.bytes[address as usize & 0x7ff]
    }

    fn write_byte(&mut self, address: Address, value: Byte) {
        self.bytes[address as usize & 0x7ff] = value;
    }
}
