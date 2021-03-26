use crate::*;

pub trait Flags {
    fn get_negative(&self) -> bool;
    fn get_overflow(&self) -> bool;
    fn get_zero(&self) -> bool;
    fn get_carry(&self) -> bool;

    fn set_negative(&mut self, value: Byte);
    fn set_zero(&mut self, value: Byte);
    fn set_carry(&mut self, value: Word);

    fn set_negative_a(&mut self);
    fn set_overflow_a(&mut self, value: Byte, before: Byte);
    fn set_zero_a(&mut self);

    fn set_negative_x(&mut self);
    fn set_zero_x(&mut self);

    fn set_negative_y(&mut self);
    fn set_zero_y(&mut self);
}

#[derive(Copy, Clone)]
pub struct Registers {
    pub a: Byte,
    pub x: Byte,
    pub y: Byte,
    pub sp: Byte,
    pub pc: Word,
    pub status: Byte,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0xFF,
            pc: 0xFFFC,
            status: Byte::default(),
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

pub const FLAG_NEGATIVE: u8 = 0b1000_0000;
pub const FLAG_OVERFLOW: u8 = 0b0100_0000;
pub const FLAG_UNUSED: u8 = 0b0010_0000;
pub const FLAG_BRK: u8 = 0b0001_0000;
pub const FLAG_DECIMAL_MODE: u8 = 0b0000_1000;
pub const FLAG_DISABLE_INTERRUPTS: u8 = 0b0000_0100;
pub const FLAG_ZERO: u8 = 0b0000_0010;
pub const FLAG_CARRY: u8 = 0b0000_0001;

impl Flags for Registers {
    fn get_negative(&self) -> bool {
        self.status & FLAG_NEGATIVE == FLAG_NEGATIVE
    }

    fn get_overflow(&self) -> bool {
        self.status & FLAG_OVERFLOW == FLAG_OVERFLOW
    }

    fn get_zero(&self) -> bool {
        self.status & FLAG_ZERO == FLAG_ZERO
    }

    fn get_carry(&self) -> bool {
        self.status & FLAG_CARRY == FLAG_CARRY
    }

    fn set_negative(&mut self, value: u8) {
        if value & FLAG_NEGATIVE == FLAG_NEGATIVE {
            self.status |= FLAG_NEGATIVE
        } else {
            self.status &= FLAG_NEGATIVE
        }
    }

    fn set_zero(&mut self, value: u8) {
        if value == 0 {
            self.status |= FLAG_ZERO
        } else {
            self.status &= !FLAG_ZERO
        }
    }

    fn set_carry(&mut self, value: Word) {
        if value > 0xFF {
            self.status |= FLAG_CARRY
        } else {
            self.status &= !FLAG_CARRY
        }
    }

    fn set_negative_a(&mut self) {
        if self.a & FLAG_NEGATIVE == FLAG_NEGATIVE {
            self.status |= FLAG_NEGATIVE
        } else {
            self.status &= FLAG_NEGATIVE
        }
    }

    fn set_overflow_a(&mut self, value: Byte, before: Byte) {
        if !((self.a ^ value) & 0x80) != 0 && ((self.a ^ before) & 0x80) != 0 {
            self.status |= FLAG_OVERFLOW
        } else {
            self.status &= !FLAG_OVERFLOW
        }
    }

    fn set_zero_a(&mut self) {
        if self.a == 0 {
            self.status |= FLAG_ZERO
        } else {
            self.status &= !FLAG_ZERO
        }
    }

    fn set_negative_x(&mut self) {
        if self.x & FLAG_NEGATIVE == FLAG_NEGATIVE {
            self.status |= FLAG_NEGATIVE
        } else {
            self.status &= FLAG_NEGATIVE
        }
    }

    fn set_zero_x(&mut self) {
        if self.x == 0 {
            self.status |= FLAG_ZERO
        } else {
            self.status &= !FLAG_ZERO
        }
    }

    fn set_negative_y(&mut self) {
        if self.y & FLAG_NEGATIVE == FLAG_NEGATIVE {
            self.status |= FLAG_NEGATIVE
        } else {
            self.status &= FLAG_NEGATIVE
        }
    }

    fn set_zero_y(&mut self) {
        if self.y == 0 {
            self.status |= FLAG_ZERO
        } else {
            self.status &= !FLAG_ZERO
        }
    }
}
