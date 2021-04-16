use crate::*;

pub trait Flags {
    fn get_negative(&self) -> bool;
    fn get_overflow(&self) -> bool;
    fn get_break(&self) -> bool;
    fn get_decimal(&self) -> bool;
    fn get_interrupt(&self) -> bool;
    fn get_zero(&self) -> bool;
    fn get_carry(&self) -> bool;

    fn set_negative(&mut self, switch: bool);
    fn set_overflow(&mut self, switch: bool);
    fn set_break(&mut self, switch: bool);
    fn set_decimal(&mut self, switch: bool);
    fn set_interrupt(&mut self, switch: bool);
    fn set_zero(&mut self, switch: bool);
    fn set_carry(&mut self, switch: bool);

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
    pub pc: Address,
    pub status: Byte,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0xFF,
            pc: Address(0xFFFC),
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
pub const FLAG_BREAK: u8 = 0b0001_0000;
pub const FLAG_DECIMAL_MODE: u8 = 0b0000_1000;
pub const FLAG_INTERRUPT: u8 = 0b0000_0100;
pub const FLAG_ZERO: u8 = 0b0000_0010;
pub const FLAG_CARRY: u8 = 0b0000_0001;

impl Flags for Registers {
    fn get_negative(&self) -> bool {
        self.status & FLAG_NEGATIVE == FLAG_NEGATIVE
    }

    fn get_overflow(&self) -> bool {
        self.status & FLAG_OVERFLOW == FLAG_OVERFLOW
    }

    fn get_break(&self) -> bool {
        self.status & FLAG_BREAK == FLAG_BREAK
    }

    fn get_decimal(&self) -> bool {
        self.status & FLAG_DECIMAL_MODE == FLAG_DECIMAL_MODE
    }

    fn get_interrupt(&self) -> bool {
        self.status & FLAG_INTERRUPT == FLAG_INTERRUPT
    }

    fn get_zero(&self) -> bool {
        self.status & FLAG_ZERO == FLAG_ZERO
    }

    fn get_carry(&self) -> bool {
        self.status & FLAG_CARRY == FLAG_CARRY
    }

    fn set_negative(&mut self, switch: bool) {
        if switch {
            self.status |= FLAG_NEGATIVE
        } else {
            self.status &= FLAG_NEGATIVE
        }
    }

    fn set_overflow(&mut self, switch: bool) {
        if switch {
            self.status |= FLAG_OVERFLOW
        } else {
            self.status &= !FLAG_OVERFLOW
        }
    }

    fn set_break(&mut self, switch: bool) {
        if switch {
            self.status |= FLAG_BREAK
        } else {
            self.status &= !FLAG_BREAK
        }
    }

    fn set_decimal(&mut self, switch: bool) {
        if switch {
            self.status |= FLAG_DECIMAL_MODE
        } else {
            self.status &= !FLAG_DECIMAL_MODE
        }
    }

    fn set_interrupt(&mut self, switch: bool) {
        if switch {
            self.status |= FLAG_INTERRUPT
        } else {
            self.status &= !FLAG_INTERRUPT
        }
    }

    fn set_zero(&mut self, switch: bool) {
        if switch {
            self.status |= FLAG_ZERO
        } else {
            self.status &= !FLAG_ZERO
        }
    }

    fn set_carry(&mut self, switch: bool) {
        if switch {
            self.status |= FLAG_CARRY
        } else {
            self.status &= !FLAG_CARRY
        }
    }

    fn set_negative_a(&mut self) {
        if self.a & FLAG_NEGATIVE > 0 {
            self.status |= FLAG_NEGATIVE
        } else {
            self.status &= FLAG_NEGATIVE
        }
    }

    fn set_overflow_a(&mut self, before: Byte, after: Byte) {
        if ((self.a ^ before) & 0x80) != 0 && !((self.a ^ after) & 0x80) != 0 {
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
        if self.x & FLAG_NEGATIVE > 0 {
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
        if self.y & FLAG_NEGATIVE > 0 {
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
