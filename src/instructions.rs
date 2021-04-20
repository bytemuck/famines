pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6D;
pub const ADC_ABSOLUTE_X: u8 = 0x7D;
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
pub const ADC_INDIRECT_X: u8 = 0x61;
pub const ADC_INDIRECT_Y: u8 = 0x71;

pub const AND_IMMEDIATE: u8 = 0x29;
pub const AND_ZERO_PAGE: u8 = 0x25;
pub const AND_ZERO_PAGE_X: u8 = 0x35;
pub const AND_ABSOLUTE: u8 = 0x2D;
pub const AND_ABSOLUTE_X: u8 = 0x3D;
pub const AND_ABSOLUTE_Y: u8 = 0x39;
pub const AND_INDIRECT_X: u8 = 0x21;
pub const AND_INDIRECT_Y: u8 = 0x31;

pub const ASL_ACCUMULATOR: u8 = 0x0A;
pub const ASL_ZERO_PAGE: u8 = 0x06;
pub const ASL_ZERO_PAGE_X: u8 = 0x16;
pub const ASL_ABSOLUTE: u8 = 0x0E;
pub const ASL_ABSOLUTE_X: u8 = 0x1E;

pub const BCC_RELATIVE: u8 = 0x90;

pub const BCS_RELATIVE: u8 = 0xB0;

pub const BEQ_RELATIVE: u8 = 0xF0;

pub const BIT_ZERO_PAGE: u8 = 0x24;
pub const BIT_ABSOLUTE: u8 = 0x2C;

pub const BMI_RELATIVE: u8 = 0x30;

pub const BNE_RELATIVE: u8 = 0xD0;

pub const BPL_RELATIVE: u8 = 0x10;

pub const BRK_IMPLIED: u8 = 0x00;

pub const BVC_RELATIVE: u8 = 0x50;

pub const BVS_RELATIVE: u8 = 0x70;

pub const CLC_IMPLIED: u8 = 0x18;

pub const CLD_IMPLIED: u8 = 0xD8;

pub const CLI_IMPLIED: u8 = 0x58;

pub const CLV_IMPLIED: u8 = 0xB8;

pub const CMP_IMMEDIATE: u8 = 0xC9;
pub const CMP_ZERO_PAGE: u8 = 0xC5;
pub const CMP_ZERO_PAGE_X: u8 = 0xD5;
pub const CMP_ABSOLUTE: u8 = 0xCD;
pub const CMP_ABSOLUTE_X: u8 = 0xDD;
pub const CMP_ABSOLUTE_Y: u8 = 0xD9;
pub const CMP_INDIRECT_X: u8 = 0xC1;
pub const CMP_INDIRECT_Y: u8 = 0xD1;

pub const CPX_IMMEDIATE: u8 = 0xE0;
pub const CPX_ZERO_PAGE: u8 = 0xE4;
pub const CPX_ABSOLUTE: u8 = 0xEC;

pub const CPY_IMMEDIATE: u8 = 0xC0;
pub const CPY_ZERO_PAGE: u8 = 0xC4;
pub const CPY_ABSOLUTE: u8 = 0xCC;

pub const DEC_ZERO_PAGE: u8 = 0xC6;
pub const DEC_ZERO_PAGE_X: u8 = 0xD6;
pub const DEC_ABSOLUTE: u8 = 0xCE;
pub const DEC_ABSOLUTE_X: u8 = 0xDE;

pub const DEX_IMPLIED: u8 = 0xCA;

pub const DEY_IMPLIED: u8 = 0x88;

pub const EOR_IMMEDIATE: u8 = 0x49;
pub const EOR_ZERO_PAGE: u8 = 0x45;
pub const EOR_ZERO_PAGE_X: u8 = 0x55;
pub const EOR_ABSOLUTE: u8 = 0x4D;
pub const EOR_ABSOLUTE_X: u8 = 0x5D;
pub const EOR_ABSOLUTE_Y: u8 = 0x59;
pub const EOR_INDIRECT_X: u8 = 0x41;
pub const EOR_INDIRECT_Y: u8 = 0x51;

pub const INC_ZERO_PAGE: u8 = 0xE6;
pub const INC_ZERO_PAGE_X: u8 = 0xF6;
pub const INC_ABSOLUTE: u8 = 0xEE;
pub const INC_ABSOLUTE_X: u8 = 0xFE;

pub const INX_IMPLIED: u8 = 0xE8;

pub const INY_IMPLIED: u8 = 0xC8;

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

pub const LSR_ACCUMULATOR: u8 = 0x4A;
pub const LSR_ZERO_PAGE: u8 = 0x46;
pub const LSR_ZERO_PAGE_X: u8 = 0x56;
pub const LSR_ABSOLUTE: u8 = 0x4E;
pub const LSR_ABSOLUTE_X: u8 = 0x5E;

pub const NOP_IMPLIED: u8 = 0xEA;

pub const ORA_IMMEDIATE: u8 = 0x09;
pub const ORA_ZERO_PAGE: u8 = 0x05;
pub const ORA_ZERO_PAGE_X: u8 = 0x15;
pub const ORA_ABSOLUTE: u8 = 0x0D;
pub const ORA_ABSOLUTE_X: u8 = 0x1D;
pub const ORA_ABSOLUTE_Y: u8 = 0x19;
pub const ORA_INDIRECT_X: u8 = 0x01;
pub const ORA_INDIRECT_Y: u8 = 0x11;

pub const PHA_IMPLIED: u8 = 0x48;

pub const PHP_IMPLIED: u8 = 0x08;

pub const PLA_IMPLIED: u8 = 0x68;

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum AddrFuncResult {
    Implied,
    Immediate(Byte),
    Relative(RelativeAddress),
    Address(Address),
}

use crate::*;

pub(crate) const INSTRUCTION_CODE: [Option<(ExecFunc, AddrFunc)>; 256] = [
    Some((brk, implied)),                            // 0x00
    Some((ora, indexed_indirect_x)),                 // 0x01
    None,                                            // 0x02
    None,                                            // 0x03
    None,                                            // 0x04
    Some((ora, zero_page)),                          // 0x05
    Some((asl, zero_page)),                          // 0x06
    None,                                            // 0x07
    Some((php, implied)),                            // 0x08
    Some((ora, immediate)),                          // 0x09
    Some((asl, implied)),                            // 0x0A
    None,                                            // 0x0B
    None,                                            // 0x0C
    Some((ora, absolute)),                           // 0x0D
    Some((asl, absolute)),                           // 0x0E
    None,                                            // 0x0F
    Some((bpl, relative)),                           // 0x10
    Some((ora, indirect_indexed_y_more_if_crossed)), // 0x11
    None,                                            // 0x12
    None,                                            // 0x13
    None,                                            // 0x14
    Some((ora, zero_page_x)),                        // 0x15
    Some((asl, zero_page_x)),                        // 0x16
    None,                                            // 0x17
    Some((clc, implied)),                            // 0x18
    Some((ora, absolute_y_more_if_crossed)),         // 0x19
    None,                                            // 0x1A
    None,                                            // 0x1B
    None,                                            // 0x1C
    Some((ora, absolute_x_more_if_crossed)),         // 0x1D
    Some((asl, absolute_x)),                         // 0x1E
    None,                                            // 0x1F
    Some((jsr, absolute)),                           // 0x20
    Some((and, indexed_indirect_x)),                 // 0x21
    None,                                            // 0x22
    None,                                            // 0x23
    Some((bit, zero_page)),                          // 0x24
    Some((and, zero_page)),                          // 0x25
    None,                                            // 0x26
    None,                                            // 0x27
    None,                                            // 0x28
    Some((and, immediate)),                          // 0x29
    None,                                            // 0x2A
    None,                                            // 0x2B
    Some((bit, absolute)),                           // 0x2C
    Some((and, absolute)),                           // 0x2D
    None,                                            // 0x2E
    None,                                            // 0x2F
    Some((bmi, relative)),                           // 0x30
    Some((and, indirect_indexed_y_more_if_crossed)), // 0x31
    None,                                            // 0x32
    None,                                            // 0x33
    None,                                            // 0x34
    Some((and, zero_page_x)),                        // 0x35
    None,                                            // 0x36
    None,                                            // 0x37
    None,                                            // 0x38
    Some((and, absolute_y_more_if_crossed)),         // 0x39
    None,                                            // 0x3A
    None,                                            // 0x3B
    None,                                            // 0x3C
    Some((and, absolute_x_more_if_crossed)),         // 0x3D
    None,                                            // 0x3E
    None,                                            // 0x3F
    None,                                            // 0x40
    Some((eor, indexed_indirect_x)),                 // 0x41
    None,                                            // 0x42
    None,                                            // 0x43
    None,                                            // 0x44
    Some((eor, zero_page)),                          // 0x45
    Some((lsr, zero_page)),                          // 0x46
    None,                                            // 0x47
    Some((pha, implied)),                            // 0x48
    Some((eor, immediate)),                          // 0x49
    Some((lsr, implied)),                            // 0x4A
    None,                                            // 0x4B
    None,                                            // 0x4C
    Some((eor, absolute)),                           // 0x4D
    Some((lsr, absolute)),                           // 0x4E
    None,                                            // 0x4F
    Some((bvc, relative)),                           // 0x50
    Some((eor, indirect_indexed_y_more_if_crossed)), // 0x51
    None,                                            // 0x52
    None,                                            // 0x53
    None,                                            // 0x54
    Some((eor, zero_page_x)),                        // 0x55
    Some((lsr, zero_page_x)),                        // 0x56
    None,                                            // 0x57
    Some((cli, implied)),                            // 0x58
    Some((eor, absolute_y_more_if_crossed)),         // 0x59
    None,                                            // 0x5A
    None,                                            // 0x5B
    None,                                            // 0x5C
    Some((eor, absolute_x_more_if_crossed)),         // 0x5D
    Some((lsr, absolute_x)),                         // 0x5E
    None,                                            // 0x5F
    None,                                            // 0x60
    Some((adc, indexed_indirect_x)),                 // 0x61
    None,                                            // 0x62
    None,                                            // 0x63
    None,                                            // 0x64
    Some((adc, zero_page)),                          // 0x65
    None,                                            // 0x66
    None,                                            // 0x67
    Some((pla, implied)),                            // 0x68
    Some((adc, immediate)),                          // 0x69
    None,                                            // 0x6A
    None,                                            // 0x6B
    None,                                            // 0x6C
    Some((adc, absolute)),                           // 0x6D
    None,                                            // 0x6E
    None,                                            // 0x6F
    Some((bvs, relative)),                           // 0x70
    Some((adc, indirect_indexed_y_more_if_crossed)), // 0x71
    None,                                            // 0x72
    None,                                            // 0x73
    None,                                            // 0x74
    Some((adc, zero_page_x)),                        // 0x75
    None,                                            // 0x76
    None,                                            // 0x77
    None,                                            // 0x78
    Some((adc, absolute_y_more_if_crossed)),         // 0x79
    None,                                            // 0x7A
    None,                                            // 0x7B
    None,                                            // 0x7C
    Some((adc, absolute_x_more_if_crossed)),         // 0x7D
    None,                                            // 0x7E
    None,                                            // 0x7F
    None,                                            // 0x80
    None,                                            // 0x81
    None,                                            // 0x82
    None,                                            // 0x83
    None,                                            // 0x84
    None,                                            // 0x85
    None,                                            // 0x86
    None,                                            // 0x87
    Some((dey, implied)),                            // 0x88
    None,                                            // 0x89
    None,                                            // 0x8A
    None,                                            // 0x8B
    None,                                            // 0x8C
    None,                                            // 0x8D
    None,                                            // 0x8E
    None,                                            // 0x8F
    Some((bcc, relative)),                           // 0x90
    None,                                            // 0x91
    None,                                            // 0x92
    None,                                            // 0x93
    None,                                            // 0x94
    None,                                            // 0x95
    None,                                            // 0x96
    None,                                            // 0x97
    None,                                            // 0x98
    None,                                            // 0x99
    None,                                            // 0x9A
    None,                                            // 0x9B
    None,                                            // 0x9C
    None,                                            // 0x9D
    None,                                            // 0x9E
    None,                                            // 0x9F
    Some((ldy, immediate)),                          // 0xA0
    Some((lda, indexed_indirect_x)),                 // 0xA1
    Some((ldx, immediate)),                          // 0xA2
    None,                                            // 0xA3
    Some((ldy, zero_page)),                          // 0xA4
    Some((lda, zero_page)),                          // 0xA5
    Some((ldx, zero_page)),                          // 0xA6
    None,                                            // 0xA7
    None,                                            // 0xA8
    Some((lda, immediate)),                          // 0xA9
    None,                                            // 0xAA
    None,                                            // 0xAB
    Some((ldy, absolute)),                           // 0xAC
    Some((lda, absolute)),                           // 0xAD
    Some((ldx, absolute)),                           // 0xAE
    None,                                            // 0xAF
    Some((bcs, relative)),                           // 0xB0
    Some((lda, indirect_indexed_y_more_if_crossed)), // 0xB1
    None,                                            // 0xB2
    None,                                            // 0xB3
    Some((ldy, zero_page_x)),                        // 0xB4
    Some((lda, zero_page_x)),                        // 0xB5
    Some((ldx, zero_page_y)),                        // 0xB6
    None,                                            // 0xB7
    Some((clv, implied)),                            // 0xB8
    Some((lda, absolute_y_more_if_crossed)),         // 0xB9
    None,                                            // 0xBA
    None,                                            // 0xBB
    Some((ldy, absolute_x_more_if_crossed)),         // 0xBC
    Some((lda, absolute_x_more_if_crossed)),         // 0xBD
    Some((ldx, absolute_y_more_if_crossed)),         // 0xBE
    None,                                            // 0xBF
    Some((cpy, immediate)),                          // 0xC0
    Some((cmp, indexed_indirect_x)),                 // 0xC1
    None,                                            // 0xC2
    None,                                            // 0xC3
    Some((cpy, zero_page)),                          // 0xC4
    Some((cmp, zero_page)),                          // 0xC5
    Some((dec, zero_page)),                          // 0xC6
    None,                                            // 0xC7
    Some((iny, implied)),                            // 0xC8
    Some((cmp, immediate)),                          // 0xC9
    Some((dex, implied)),                            // 0xCA
    None,                                            // 0xCB
    Some((cpy, absolute)),                           // 0xCC
    Some((cmp, absolute)),                           // 0xCD
    Some((dec, absolute)),                           // 0xCE
    None,                                            // 0xCF
    Some((bne, relative)),                           // 0xD0
    Some((cmp, indirect_indexed_y_more_if_crossed)), // 0xD1
    None,                                            // 0xD2
    None,                                            // 0xD3
    None,                                            // 0xD4
    Some((cmp, zero_page_x)),                        // 0xD5
    Some((dec, zero_page_x)),                        // 0xD6
    None,                                            // 0xD7
    Some((cld, implied)),                            // 0xD8
    Some((cmp, absolute_y_more_if_crossed)),         // 0xD9
    None,                                            // 0xDA
    None,                                            // 0xDB
    None,                                            // 0xDC
    Some((cmp, absolute_x_more_if_crossed)),         // 0xDD
    Some((dec, absolute_x)),                         // 0xDE
    None,                                            // 0xDF
    Some((cpx, immediate)),                          // 0xE0
    None,                                            // 0xE1
    None,                                            // 0xE2
    None,                                            // 0xE3
    Some((cpx, zero_page)),                          // 0xE4
    None,                                            // 0xE5
    Some((inc, zero_page)),                          // 0xE6
    None,                                            // 0xE7
    Some((inx, implied)),                            // 0xE8
    None,                                            // 0xE9
    Some((nop, implied)),                            // 0xEA
    None,                                            // 0xEB
    Some((cpx, absolute)),                           // 0xEC
    None,                                            // 0xED
    Some((inc, absolute)),                           // 0xEE
    None,                                            // 0xEF
    Some((beq, relative)),                           // 0xF0
    None,                                            // 0xF1
    None,                                            // 0xF2
    None,                                            // 0xF3
    None,                                            // 0xF4
    None,                                            // 0xF5
    Some((inc, zero_page_x)),                        // 0xF6
    None,                                            // 0xF7
    None,                                            // 0xF8
    None,                                            // 0xF9
    None,                                            // 0xFA
    None,                                            // 0xFB
    None,                                            // 0xFC
    None,                                            // 0xFD
    Some((inc, absolute_x)),                         // 0xFE
    None,                                            // 0xFF
];
