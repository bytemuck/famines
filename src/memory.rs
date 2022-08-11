use std::usize;

use crate::*;

pub trait Memory {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn read_word(&self, addr: u16) -> u16 {
        let low = self.read_byte(addr) as u16;
        let high = self.read_byte(addr + 1) as u16;
        (high << 8) | (low as u16)
    }
    fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, (value >> 8) as u8);
        self.write_byte(addr + 1, (value & 0xFF) as u8);
    }
}