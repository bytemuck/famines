use crate::*;

#[derive(Copy, Clone)]
pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    // status
    pub c: bool,
    pub z: bool,
    pub i: bool,
    pub d: bool,
    pub b: bool,
    pub unused: bool,
    pub v: bool,
    pub n: bool,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0xFFFC,
            sp: 0xFF,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            c: false,
            z: false,
            i: true,
            d: false,
            b: false,
            unused: true,
            v: false,
            n: false,
        }
    }

    pub fn to_byte(&self) -> u8 {
        return 0
            + if self.c { FLAG_CARRY } else { 0 }
            + if self.z { FLAG_ZERO } else { 0 }
            + if self.i { FLAG_INTERRUPT } else { 0 }
            + if self.d { FLAG_DECIMAL_MODE } else { 0 }
            + if self.b { FLAG_BREAK } else { 0 }
            + if self.unused { FLAG_UNUSED } else { 0 }
            + if self.v { FLAG_OVERFLOW } else { 0 }
            + if self.n { FLAG_NEGATIVE } else { 0 };
    }

    pub fn from_byte(&mut self, from: u8) {
        self.c = from & FLAG_CARRY > 0;
        self.z = from & FLAG_ZERO > 0;
        self.i = from & FLAG_INTERRUPT > 0;
        self.d = from & FLAG_DECIMAL_MODE > 0;
        self.b = from & FLAG_BREAK > 0;
        self.unused = from & FLAG_UNUSED > 0;
        self.v = from & FLAG_OVERFLOW > 0;
        self.n = from & FLAG_NEGATIVE > 0;
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
