#[repr(u8)]
pub enum CPUFlag {
    Carry       = 0b00000001,
    Zero        = 0b00000010,
    Interrupt   = 0b00000100,
    Decimal     = 0b00001000,
    Break       = 0b00010000,
    Unused      = 0b00100000,
    Overflow    = 0b01000000,
    Negative    = 0b10000000,
}

impl std::ops::BitOr<CPUFlag> for CPUFlag {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl std::ops::BitOr<CPUFlag> for u8 {
    type Output = u8;

    fn bitor(self, rhs: CPUFlag) -> Self::Output {
        self | rhs as u8
    }
}

impl std::ops::BitOrAssign<CPUFlag> for u8 {
    fn bitor_assign(&mut self, rhs: CPUFlag) {
        *self |= rhs as u8;
    }
} 

