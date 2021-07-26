use std::usize;

use crate::*;

pub const MEMORY_SIZE: usize = 0xFFFF + 1; // 64 636 Bytes, 524 288 Bits

pub const STACK_TOP: u16 = 0x01FF;
pub const STACK_BOTTOM: u16 = 0x0100;

pub const INTERRUPT_HANDLER_TOP: u16 = 0xFFFB;
pub const INTERRUPT_HANDLER_BOTTOM: u16 = 0xFFFA;

pub const POWER_ON_RESET_LOCATION_TOP: u16 = 0xFFFD;
pub const POWER_ON_RESET_LOCATION_BOTTOM: u16 = 0xFFFC;

pub const BREAK_AND_INTERRUPT_REQUEST_HANDLER_TOP: u16 = 0xFFFF;
pub const BREAK_AND_INTERRUPT_REQUEST_HANDLER_BOTTOM: u16 = 0xFFFE;

#[derive(Copy, Clone)]
pub struct Memory {
    pub bytes: [u8; MEMORY_SIZE],
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

impl std::ops::Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &<Self as std::ops::Index<u16>>::Output {
        &self.bytes[index as usize]
    }
}

impl std::ops::IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut <Self as std::ops::Index<u16>>::Output {
        &mut self.bytes[index as usize]
    }
}
