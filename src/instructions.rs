pub const INC_ZERO_PAGE: u8 = 0xE6;
pub const INC_ZERO_PAGE_X: u8 = 0xF6;
pub const INC_ABSOLUTE: u8 = 0xEE;
pub const INC_ABSOLUTE_X: u8 = 0xFE;

pub const JSR_ABSOLUTE: u8 = 0x20;

pub const LDA_IMMEDIATE: u8 = 0xA9;
pub const LDA_ZERO_PAGE: u8 = 0xA5;
pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
pub const LDA_ABSOLUTE: u8 = 0xAD;
pub const LDA_ABSOLUTE_X: u8 = 0xBD;
pub const LDA_ABSOLUTE_Y: u8 = 0xB9;
pub const LDA_INDIRECT_X: u8 = 0xA1;
pub const LDA_INDIRECT_Y: u8 = 0xB1;

pub const LDX_IMMEDIATE: u8 = 0xA2;
pub const LDX_ZERO_PAGE: u8 = 0xA6;
pub const LDX_ZERO_PAGE_Y: u8 = 0xB6;
pub const LDX_ABSOLUTE: u8 = 0xAE;
pub const LDX_ABSOLUTE_Y: u8 = 0xBE;

pub const LDY_IMMEDIATE: u8 = 0xA0;
pub const LDY_ZERO_PAGE: u8 = 0xA4;
pub const LDY_ZERO_PAGE_X: u8 = 0xB4;
pub const LDY_ABSOLUTE: u8 = 0xAC;
pub const LDY_ABSOLUTE_X: u8 = 0xBC;

pub type Byte = u8;
pub type SignedByte = i8;
pub type Word = u16;
pub type SignedWord = i16;

#[derive(Copy, Clone)]
pub enum Instruction {
    Unknown,
    INC,
    JSR,
    LDA,
    LDX,
    LDY,
}

#[derive(Copy, Clone)]
pub enum ImpliedRegister {
    None,
    X,
    Y,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    Unknown,
    Accumulator,
    Immediate,
    Implied,
    Relative,
    Absolute(bool), // true if variant cycles (+1 if page crossed)
    ZeroPage,
    Indirect,
    IndexedIndirect,
    IndirectIndexed(bool),
}

#[derive(Copy, Clone)]
pub enum InstructionInput {
    Unknown,
    Implied,
    Immediate(Byte),
    Relative(SignedByte),
    Address(Word),
}

pub type DecodedInstruction = (Instruction, InstructionInput, Byte);

#[rustfmt::skip]
pub static INSTRUCTION_CODE: [(Instruction, AddressingMode, ImpliedRegister, u8); 256] = [
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x00
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x01
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x02
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x03
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x04
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x05
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x06
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x07
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x08
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x09
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x0A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x0B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x0C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x0D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x0E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x0F

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x10
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x11
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x12
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x13
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x14
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x15
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x16
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x17
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x18
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x19
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x1A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x1B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x1C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x1D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x1E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x1F

    (Instruction::JSR, AddressingMode::Absolute(true), ImpliedRegister::None, 6),      // 0x20
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x21
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x22
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x23
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x24
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x25
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x26
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x27
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x28
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x29
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x2A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x2B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x2C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x2D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x2E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x2F

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x30
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x31
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x32
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x33
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x34
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x35
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x36
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x37
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x38
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x39
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x3A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x3B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x3C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x3D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x3E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x3F

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x40
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x41
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x42
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x43
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x44
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x45
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x46
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x47
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x48
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x49
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x4A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x4B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x4C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x4D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x4E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x4F

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x50
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x51
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x52
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x53
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x54
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x55
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x56
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x57
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x58
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x59
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x5A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x5B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x5C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x5D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x5E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x5F

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x60
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x61
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x62
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x63
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x64
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x65
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x66
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x67
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x68
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x69
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x6A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x6B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x6C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x6D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x6E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x6F

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x70
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x71
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x72
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x73
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x74
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x75
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x76
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x77
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x78
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x79
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x7A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x7B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x7C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x7D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x7E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x7F

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x80
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x81
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x82
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x83
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x84
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x85
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x86
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x87
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x88
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x89
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x8A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x8B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x8C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x8D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x8E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x8F

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x90
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x91
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x92
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x93
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x94
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x95
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x96
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x97
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x98
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x99
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x9A
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x9B
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x9C
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x9D
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x9E
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0x9F

    (Instruction::LDY, AddressingMode::Immediate, ImpliedRegister::None, 2),           // 0xA0
    (Instruction::LDA, AddressingMode::IndexedIndirect, ImpliedRegister::X, 6),        // 0xA1
    (Instruction::LDX, AddressingMode::Immediate, ImpliedRegister::None, 2),           // 0xA2
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xA3
    (Instruction::LDY, AddressingMode::ZeroPage, ImpliedRegister::None, 3),            // 0xA4
    (Instruction::LDA, AddressingMode::ZeroPage, ImpliedRegister::None, 3),            // 0xA5
    (Instruction::LDX, AddressingMode::ZeroPage, ImpliedRegister::None, 3),            // 0xA6
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xA7
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xA8
    (Instruction::LDA, AddressingMode::Immediate, ImpliedRegister::None, 2),           // 0xA9
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xAA
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xAB
    (Instruction::LDY, AddressingMode::Absolute(true), ImpliedRegister::None, 4),      // 0xAC
    (Instruction::LDA, AddressingMode::Absolute(true), ImpliedRegister::None, 4),      // 0xAD
    (Instruction::LDX, AddressingMode::Absolute(true), ImpliedRegister::None, 4),      // 0xAE
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xAF

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xB0
    (Instruction::LDA, AddressingMode::IndirectIndexed(true), ImpliedRegister::Y, 5),  // 0xB1
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xB2
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xB3
    (Instruction::LDY, AddressingMode::ZeroPage, ImpliedRegister::X, 4),               // 0xB4
    (Instruction::LDA, AddressingMode::ZeroPage, ImpliedRegister::X, 4),               // 0xB5
    (Instruction::LDX, AddressingMode::ZeroPage, ImpliedRegister::Y, 4),               // 0xB6
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xB7
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xB8
    (Instruction::LDA, AddressingMode::Absolute(true), ImpliedRegister::Y, 4),         // 0xB9
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xBA
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xBB
    (Instruction::LDY, AddressingMode::Absolute(true), ImpliedRegister::X, 4),         // 0xBC
    (Instruction::LDA, AddressingMode::Absolute(true), ImpliedRegister::X, 4),         // 0xBD
    (Instruction::LDX, AddressingMode::Absolute(true), ImpliedRegister::Y, 4),         // 0xBE
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xBF

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC0
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC1
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC2
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC3
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC4
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC5
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC6
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC7
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC8
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xC9
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xCA
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xCB
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xCC
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xCD
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xCE
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xCF

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD0
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD1
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD2
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD3
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD4
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD5
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD6
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD7
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD8
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xD9
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xDA
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xDB
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xDC
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xDD
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xDE
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xDF

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE0
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE1
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE2
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE3
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE4
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE5
    (Instruction::INC, AddressingMode::ZeroPage, ImpliedRegister::None, 5),            // 0xE6
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE7
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE8
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xE9
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xEA
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xEB
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xEC
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xED
    (Instruction::INC, AddressingMode::Absolute(false), ImpliedRegister::None, 6),     // 0xEE
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xEF

    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF0
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF1
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF2
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF3
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF4
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF5
    (Instruction::INC, AddressingMode::ZeroPage, ImpliedRegister::X, 6),               // 0xF6
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF7
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF8
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xF9
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xFA
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xFB
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xFC
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xFD
    (Instruction::INC, AddressingMode::Absolute(false), ImpliedRegister::X, 7),        // 0xFE
    (Instruction::Unknown, AddressingMode::Unknown, ImpliedRegister::None, 0),         // 0xFF
];
