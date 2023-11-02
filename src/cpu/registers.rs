use crate::memory::{Byte, Word};

pub struct Registers {
    pub a: Byte, // Accumulator
    pub x: Byte,
    pub y: Byte,
    pub sp: Byte,
    pub flags: u8,
    pub pc: Word,
}

impl Registers {
    pub const CARRY_FLAG: u8 = 1 << 0;
    pub const ZERO_FLAG: u8 = 1 << 1;
    pub const IRQ_FLAG: u8 = 1 << 2;
    pub const DECIMAL_FLAG: u8 = 1 << 3;
    pub const BREAK_FLAG: u8 = 1 << 4;
    pub const UNUSED_FLAG: u8 = 1 << 5;
    pub const OVERFLOW_FLAG: u8 = 1 << 6;
    pub const NEGATIVE_FLAG: u8 = 1 << 7;

    pub const STACK: u16 = 0x0100;
    pub const STACK_RESET: u8 = 0xfd;
    pub const RESET_VECTOR: u16 = 0xFFFC;

    pub fn new() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x0000,
            sp: Self::STACK_RESET,
            flags: Self::IRQ_FLAG | Self::UNUSED_FLAG,
        }
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        (self.flags & flag) != 0
    }

    pub fn set_flag(&mut self, flag: u8, on: bool) {
        if on {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    pub fn set_flags(&mut self, value: u8) {
        self.flags =
            (value | Registers::UNUSED_FLAG | Registers::BREAK_FLAG) - Registers::BREAK_FLAG;
    }

    pub fn set_zn(&mut self, value: Byte) -> Byte {
        self.set_flag(Registers::ZERO_FLAG, value == 0x00);
        self.set_flag(Registers::NEGATIVE_FLAG, (value & 0x80) != 0);
        value
    }

    pub fn set_a(&mut self, value: Byte) {
        self.a = self.set_zn(value);
    }

    pub fn set_y(&mut self, value: Byte) {
        self.y = self.set_zn(value);
    }

    pub fn set_x(&mut self, value: Byte) {
        self.x = self.set_zn(value);
    }
}
