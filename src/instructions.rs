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

#[derive(Copy, Clone)]
pub(crate) enum Instruction {
    INC,
    JSR,
    LDA,
    LDX,
    LDY,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum AddrFuncResult {
    Implied,
    Immediate(Byte),
    Relative(Word),
    Address(Word),
}

use crate::*;
use Instruction::*;

pub(crate) const INSTRUCTION_CODE: [Option<(Instruction, AddrFunc, ExecFunc)>; 256] = [
    None,                                                 // 0x00
    None,                                                 // 0x01
    None,                                                 // 0x02
    None,                                                 // 0x03
    None,                                                 // 0x04
    None,                                                 // 0x05
    None,                                                 // 0x06
    None,                                                 // 0x07
    None,                                                 // 0x08
    None,                                                 // 0x09
    None,                                                 // 0x0A
    None,                                                 // 0x0B
    None,                                                 // 0x0C
    None,                                                 // 0x0D
    None,                                                 // 0x0E
    None,                                                 // 0x0F
    None,                                                 // 0x10
    None,                                                 // 0x11
    None,                                                 // 0x12
    None,                                                 // 0x13
    None,                                                 // 0x14
    None,                                                 // 0x15
    None,                                                 // 0x16
    None,                                                 // 0x17
    None,                                                 // 0x18
    None,                                                 // 0x19
    None,                                                 // 0x1A
    None,                                                 // 0x1B
    None,                                                 // 0x1C
    None,                                                 // 0x1D
    None,                                                 // 0x1E
    None,                                                 // 0x1F
    Some((JSR, absolute, jsr)),                           // 0x20
    None,                                                 // 0x21
    None,                                                 // 0x22
    None,                                                 // 0x23
    None,                                                 // 0x24
    None,                                                 // 0x25
    None,                                                 // 0x26
    None,                                                 // 0x27
    None,                                                 // 0x28
    None,                                                 // 0x29
    None,                                                 // 0x2A
    None,                                                 // 0x2B
    None,                                                 // 0x2C
    None,                                                 // 0x2D
    None,                                                 // 0x2E
    None,                                                 // 0x2F
    None,                                                 // 0x30
    None,                                                 // 0x31
    None,                                                 // 0x32
    None,                                                 // 0x33
    None,                                                 // 0x34
    None,                                                 // 0x35
    None,                                                 // 0x36
    None,                                                 // 0x37
    None,                                                 // 0x38
    None,                                                 // 0x39
    None,                                                 // 0x3A
    None,                                                 // 0x3B
    None,                                                 // 0x3C
    None,                                                 // 0x3D
    None,                                                 // 0x3E
    None,                                                 // 0x3F
    None,                                                 // 0x40
    None,                                                 // 0x41
    None,                                                 // 0x42
    None,                                                 // 0x43
    None,                                                 // 0x44
    None,                                                 // 0x45
    None,                                                 // 0x46
    None,                                                 // 0x47
    None,                                                 // 0x48
    None,                                                 // 0x49
    None,                                                 // 0x4A
    None,                                                 // 0x4B
    None,                                                 // 0x4C
    None,                                                 // 0x4D
    None,                                                 // 0x4E
    None,                                                 // 0x4F
    None,                                                 // 0x50
    None,                                                 // 0x51
    None,                                                 // 0x52
    None,                                                 // 0x53
    None,                                                 // 0x54
    None,                                                 // 0x55
    None,                                                 // 0x56
    None,                                                 // 0x57
    None,                                                 // 0x58
    None,                                                 // 0x59
    None,                                                 // 0x5A
    None,                                                 // 0x5B
    None,                                                 // 0x5C
    None,                                                 // 0x5D
    None,                                                 // 0x5E
    None,                                                 // 0x5F
    None,                                                 // 0x60
    None,                                                 // 0x61
    None,                                                 // 0x62
    None,                                                 // 0x63
    None,                                                 // 0x64
    None,                                                 // 0x65
    None,                                                 // 0x66
    None,                                                 // 0x67
    None,                                                 // 0x68
    None,                                                 // 0x69
    None,                                                 // 0x6A
    None,                                                 // 0x6B
    None,                                                 // 0x6C
    None,                                                 // 0x6D
    None,                                                 // 0x6E
    None,                                                 // 0x6F
    None,                                                 // 0x70
    None,                                                 // 0x71
    None,                                                 // 0x72
    None,                                                 // 0x73
    None,                                                 // 0x74
    None,                                                 // 0x75
    None,                                                 // 0x76
    None,                                                 // 0x77
    None,                                                 // 0x78
    None,                                                 // 0x79
    None,                                                 // 0x7A
    None,                                                 // 0x7B
    None,                                                 // 0x7C
    None,                                                 // 0x7D
    None,                                                 // 0x7E
    None,                                                 // 0x7F
    None,                                                 // 0x80
    None,                                                 // 0x81
    None,                                                 // 0x82
    None,                                                 // 0x83
    None,                                                 // 0x84
    None,                                                 // 0x85
    None,                                                 // 0x86
    None,                                                 // 0x87
    None,                                                 // 0x88
    None,                                                 // 0x89
    None,                                                 // 0x8A
    None,                                                 // 0x8B
    None,                                                 // 0x8C
    None,                                                 // 0x8D
    None,                                                 // 0x8E
    None,                                                 // 0x8F
    None,                                                 // 0x90
    None,                                                 // 0x91
    None,                                                 // 0x92
    None,                                                 // 0x93
    None,                                                 // 0x94
    None,                                                 // 0x95
    None,                                                 // 0x96
    None,                                                 // 0x97
    None,                                                 // 0x98
    None,                                                 // 0x99
    None,                                                 // 0x9A
    None,                                                 // 0x9B
    None,                                                 // 0x9C
    None,                                                 // 0x9D
    None,                                                 // 0x9E
    None,                                                 // 0x9F
    Some((LDY, immediate, ldy)),                          // 0xA0
    Some((LDA, indexed_indirect_x, lda)),                 // 0xA1
    Some((LDX, immediate, ldx)),                          // 0xA2
    None,                                                 // 0xA3
    Some((LDY, zero_page, ldy)),                          // 0xA4
    Some((LDA, zero_page, lda)),                          // 0xA5
    Some((LDX, zero_page, ldx)),                          // 0xA6
    None,                                                 // 0xA7
    None,                                                 // 0xA8
    Some((LDA, immediate, lda)),                          // 0xA9
    None,                                                 // 0xAA
    None,                                                 // 0xAB
    Some((LDY, absolute, ldy)),                           // 0xAC
    Some((LDA, absolute, lda)),                           // 0xAD
    Some((LDX, absolute, ldx)),                           // 0xAE
    None,                                                 // 0xAF
    None,                                                 // 0xB0
    Some((LDA, indirect_indexed_y_more_if_crossed, lda)), // 0xB1
    None,                                                 // 0xB2
    None,                                                 // 0xB3
    Some((LDY, zero_page_x, ldy)),                        // 0xB4
    Some((LDA, zero_page_x, lda)),                        // 0xB5
    Some((LDX, zero_page_y, ldx)),                        // 0xB6
    None,                                                 // 0xB7
    None,                                                 // 0xB8
    Some((LDA, absolute_y_more_if_crossed, lda)),         // 0xB9
    None,                                                 // 0xBA
    None,                                                 // 0xBB
    Some((LDY, absolute_x_more_if_crossed, ldy)),         // 0xBC
    Some((LDA, absolute_x_more_if_crossed, lda)),         // 0xBD
    Some((LDX, absolute_y_more_if_crossed, ldx)),         // 0xBE
    None,                                                 // 0xBF
    None,                                                 // 0xC0
    None,                                                 // 0xC1
    None,                                                 // 0xC2
    None,                                                 // 0xC3
    None,                                                 // 0xC4
    None,                                                 // 0xC5
    None,                                                 // 0xC6
    None,                                                 // 0xC7
    None,                                                 // 0xC8
    None,                                                 // 0xC9
    None,                                                 // 0xCA
    None,                                                 // 0xCB
    None,                                                 // 0xCC
    None,                                                 // 0xCD
    None,                                                 // 0xCE
    None,                                                 // 0xCF
    None,                                                 // 0xD0
    None,                                                 // 0xD1
    None,                                                 // 0xD2
    None,                                                 // 0xD3
    None,                                                 // 0xD4
    None,                                                 // 0xD5
    None,                                                 // 0xD6
    None,                                                 // 0xD7
    None,                                                 // 0xD8
    None,                                                 // 0xD9
    None,                                                 // 0xDA
    None,                                                 // 0xDB
    None,                                                 // 0xDC
    None,                                                 // 0xDD
    None,                                                 // 0xDE
    None,                                                 // 0xDF
    None,                                                 // 0xE0
    None,                                                 // 0xE1
    None,                                                 // 0xE2
    None,                                                 // 0xE3
    None,                                                 // 0xE4
    None,                                                 // 0xE5
    Some((INC, zero_page, inc)),                          // 0xE6
    None,                                                 // 0xE7
    None,                                                 // 0xE8
    None,                                                 // 0xE9
    None,                                                 // 0xEA
    None,                                                 // 0xEB
    None,                                                 // 0xEC
    None,                                                 // 0xED
    Some((INC, absolute, inc)),                           // 0xEE
    None,                                                 // 0xEF
    None,                                                 // 0xF0
    None,                                                 // 0xF1
    None,                                                 // 0xF2
    None,                                                 // 0xF3
    None,                                                 // 0xF4
    None,                                                 // 0xF5
    Some((INC, zero_page_x, inc)),                        // 0xF6
    None,                                                 // 0xF7
    None,                                                 // 0xF8
    None,                                                 // 0xF9
    None,                                                 // 0xFA
    None,                                                 // 0xFB
    None,                                                 // 0xFC
    None,                                                 // 0xFD
    Some((INC, absolute_x, inc)),                         // 0xFE
    None,                                                 // 0xFF
];
