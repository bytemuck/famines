use std::collections::HashMap;

use crate::CPU;

#[derive(Debug)]
pub enum AddressMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implied,
    None,
}

type OperationFn = fn(&mut CPU, &AddressMode);

pub struct Operation {
    pub name: &'static str,
    pub code: u8,
    pub length: u8,
    pub cycles: u8,
    pub mode: AddressMode,
    pub function: OperationFn,
}

pub const OPERATIONS: [Operation; 256] = [
    Operation {
        name: "BRK",
        code: 0x00,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::brk,
    }, // 0x00
    Operation {
        name: "ORA",
        code: 0x01,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectX,
        function: CPU::ora,
    }, // 0x01
    Operation {
        name: "",
        code: 0x02,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x02
    Operation {
        name: "",
        code: 0x03,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x03
    Operation {
        name: "",
        code: 0x04,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x04
    Operation {
        name: "ORA",
        code: 0x05,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::ora,
    }, // 0x05
    Operation {
        name: "ASL",
        code: 0x06,
        length: 2,
        cycles: 5,
        mode: AddressMode::ZeroPage,
        function: CPU::asl,
    }, // 0x06
    Operation {
        name: "",
        code: 0x07,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x07
    Operation {
        name: "PHP",
        code: 0x08,
        length: 1,
        cycles: 3,
        mode: AddressMode::Implied,
        function: CPU::php,
    }, // 0x08
    Operation {
        name: "ORA",
        code: 0x09,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::ora,
    }, // 0x09
    Operation {
        name: "ASL",
        code: 0x0A,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::asl,
    }, // 0x0A
    Operation {
        name: "",
        code: 0x0B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x0B
    Operation {
        name: "",
        code: 0x0C,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x0C
    Operation {
        name: "ORA",
        code: 0x0D,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::ora,
    }, // 0x0D
    Operation {
        name: "ASL",
        code: 0x0E,
        length: 3,
        cycles: 6,
        mode: AddressMode::Absolute,
        function: CPU::asl,
    }, // 0x0E
    Operation {
        name: "",
        code: 0x0F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x0F
    Operation {
        name: "BPL",
        code: 0x10,
        length: 2,
        cycles: 2, /* maybe + 1 or + 2 */
        mode: AddressMode::Implied,
        function: CPU::bpl,
    }, // 0x10
    Operation {
        name: "ORA",
        code: 0x11,
        length: 2,
        cycles: 5, /* maybe + 1 */
        mode: AddressMode::IndirectY,
        function: CPU::ora,
    }, // 0x11
    Operation {
        name: "",
        code: 0x12,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x12
    Operation {
        name: "",
        code: 0x13,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x13
    Operation {
        name: "",
        code: 0x14,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x14
    Operation {
        name: "ORA",
        code: 0x15,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::ora,
    }, // 0x15
    Operation {
        name: "ASL",
        code: 0x16,
        length: 2,
        cycles: 6,
        mode: AddressMode::ZeroPageX,
        function: CPU::asl,
    }, // 0x16
    Operation {
        name: "",
        code: 0x17,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x17
    Operation {
        name: "CLC",
        code: 0x18,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::clc,
    }, // 0x18
    Operation {
        name: "ORA",
        code: 0x19,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteY,
        function: CPU::ora,
    }, // 0x19
    Operation {
        name: "",
        code: 0x1A,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x1A
    Operation {
        name: "",
        code: 0x1B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x1B
    Operation {
        name: "",
        code: 0x1C,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x1C
    Operation {
        name: "ORA",
        code: 0x1D,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteX,
        function: CPU::ora,
    }, // 0x1D
    Operation {
        name: "ASL",
        code: 0x1E,
        length: 3,
        cycles: 7,
        mode: AddressMode::AbsoluteX,
        function: CPU::asl,
    }, // 0x1E
    Operation {
        name: "",
        code: 0x1F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x1F
    Operation {
        name: "JSR",
        code: 0x20,
        length: 3,
        cycles: 6,
        mode: AddressMode::Implied,
        function: CPU::jsr,
    }, // 0x20
    Operation {
        name: "AND",
        code: 0x21,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectX,
        function: CPU::and,
    }, // 0x21
    Operation {
        name: "",
        code: 0x22,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x22
    Operation {
        name: "",
        code: 0x23,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x23
    Operation {
        name: "BIT",
        code: 0x24,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::bit,
    }, // 0x24
    Operation {
        name: "AND",
        code: 0x25,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::and,
    }, // 0x25
    Operation {
        name: "ROL",
        code: 0x26,
        length: 2,
        cycles: 5,
        mode: AddressMode::ZeroPage,
        function: CPU::rol,
    }, // 0x26
    Operation {
        name: "",
        code: 0x27,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x27
    Operation {
        name: "PLP",
        code: 0x28,
        length: 1,
        cycles: 4,
        mode: AddressMode::Implied,
        function: CPU::plp,
    }, // 0x28
    Operation {
        name: "AND",
        code: 0x29,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::and,
    }, // 0x29
    Operation {
        name: "ROL",
        code: 0x2A,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::rol,
    }, // 0x2A
    Operation {
        name: "",
        code: 0x2B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x2B
    Operation {
        name: "BIT",
        code: 0x2C,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::bit,
    }, // 0x2C
    Operation {
        name: "AND",
        code: 0x2D,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::and,
    }, // 0x2D
    Operation {
        name: "ROL",
        code: 0x2E,
        length: 3,
        cycles: 6,
        mode: AddressMode::Absolute,
        function: CPU::rol,
    }, // 0x2E
    Operation {
        name: "",
        code: 0x2F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x2F
    Operation {
        name: "BMI",
        code: 0x30,
        length: 2,
        cycles: 2, /* maybe + 1 or + 2 */
        mode: AddressMode::Implied,
        function: CPU::bmi,
    }, // 0x30
    Operation {
        name: "AND",
        code: 0x31,
        length: 2,
        cycles: 5, /* maybe + 1 */
        mode: AddressMode::IndirectY,
        function: CPU::and,
    }, // 0x31
    Operation {
        name: "",
        code: 0x32,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x32
    Operation {
        name: "",
        code: 0x33,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x33
    Operation {
        name: "",
        code: 0x34,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x34
    Operation {
        name: "AND",
        code: 0x35,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::and,
    }, // 0x35
    Operation {
        name: "ROL",
        code: 0x36,
        length: 2,
        cycles: 6,
        mode: AddressMode::ZeroPageX,
        function: CPU::rol,
    }, // 0x36
    Operation {
        name: "",
        code: 0x37,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x37
    Operation {
        name: "SEC",
        code: 0x38,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::sec,
    }, // 0x38
    Operation {
        name: "AND",
        code: 0x39,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteY,
        function: CPU::and,
    }, // 0x39
    Operation {
        name: "",
        code: 0x3A,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x3A
    Operation {
        name: "",
        code: 0x3B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x3B
    Operation {
        name: "",
        code: 0x3C,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x3C
    Operation {
        name: "AND",
        code: 0x3D,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteX,
        function: CPU::and,
    }, // 0x3D
    Operation {
        name: "ROL",
        code: 0x3E,
        length: 3,
        cycles: 7,
        mode: AddressMode::AbsoluteX,
        function: CPU::rol,
    }, // 0x3E
    Operation {
        name: "",
        code: 0x3F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x3F
    Operation {
        name: "RTI",
        code: 0x40,
        length: 1,
        cycles: 6,
        mode: AddressMode::Implied,
        function: CPU::rti,
    }, // 0x40
    Operation {
        name: "EOR",
        code: 0x41,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectX,
        function: CPU::eor,
    }, // 0x41
    Operation {
        name: "",
        code: 0x42,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x42
    Operation {
        name: "",
        code: 0x43,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x43
    Operation {
        name: "",
        code: 0x44,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x44
    Operation {
        name: "EOR",
        code: 0x45,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::eor,
    }, // 0x45
    Operation {
        name: "LSR",
        code: 0x46,
        length: 2,
        cycles: 5,
        mode: AddressMode::ZeroPage,
        function: CPU::lsr,
    }, // 0x46
    Operation {
        name: "",
        code: 0x47,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x47
    Operation {
        name: "PHA",
        code: 0x48,
        length: 1,
        cycles: 3,
        mode: AddressMode::Implied,
        function: CPU::pha,
    }, // 0x48
    Operation {
        name: "EOR",
        code: 0x49,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::eor,
    }, // 0x49
    Operation {
        name: "LSR",
        code: 0x4A,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::lsr,
    }, // 0x4A
    Operation {
        name: "",
        code: 0x4B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x4B
    Operation {
        name: "JMP",
        code: 0x4C,
        length: 3,
        cycles: 3,
        mode: AddressMode::Absolute,
        function: CPU::jmp,
    }, // 0x4C
    Operation {
        name: "EOR",
        code: 0x4D,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::eor,
    }, // 0x4D
    Operation {
        name: "LSR",
        code: 0x4E,
        length: 3,
        cycles: 6,
        mode: AddressMode::Absolute,
        function: CPU::lsr,
    }, // 0x4E
    Operation {
        name: "",
        code: 0x4F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x4F
    Operation {
        name: "BVC",
        code: 0x50,
        length: 2,
        cycles: 2, /* maybe + 1 or + 2 */
        mode: AddressMode::Implied,
        function: CPU::bvc,
    }, // 0x50
    Operation {
        name: "EOR",
        code: 0x51,
        length: 2,
        cycles: 5, /* maybe + 1 */
        mode: AddressMode::IndirectY,
        function: CPU::eor,
    }, // 0x51
    Operation {
        name: "",
        code: 0x52,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x52
    Operation {
        name: "",
        code: 0x53,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x53
    Operation {
        name: "",
        code: 0x54,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x54
    Operation {
        name: "EOR",
        code: 0x55,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::eor,
    }, // 0x55
    Operation {
        name: "LSR",
        code: 0x56,
        length: 2,
        cycles: 6,
        mode: AddressMode::ZeroPageX,
        function: CPU::lsr,
    }, // 0x56
    Operation {
        name: "",
        code: 0x57,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x57
    Operation {
        name: "CLI",
        code: 0x58,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::cli,
    }, // 0x58
    Operation {
        name: "EOR",
        code: 0x59,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteY,
        function: CPU::eor,
    }, // 0x59
    Operation {
        name: "",
        code: 0x5A,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x5A
    Operation {
        name: "",
        code: 0x5B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x5B
    Operation {
        name: "",
        code: 0x5C,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x5C
    Operation {
        name: "EOR",
        code: 0x5D,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteX,
        function: CPU::eor,
    }, // 0x5D
    Operation {
        name: "LSR",
        code: 0x5E,
        length: 3,
        cycles: 7,
        mode: AddressMode::AbsoluteX,
        function: CPU::lsr,
    }, // 0x5E
    Operation {
        name: "",
        code: 0x5F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x5F
    Operation {
        name: "RTS",
        code: 0x60,
        length: 1,
        cycles: 6,
        mode: AddressMode::Implied,
        function: CPU::rts,
    }, // 0x60
    Operation {
        name: "ADC",
        code: 0x61,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectX,
        function: CPU::adc,
    }, // 0x61
    Operation {
        name: "",
        code: 0x62,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x62
    Operation {
        name: "",
        code: 0x63,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x63
    Operation {
        name: "",
        code: 0x64,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x64
    Operation {
        name: "ADC",
        code: 0x65,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::adc,
    }, // 0x65
    Operation {
        name: "ROR",
        code: 0x66,
        length: 2,
        cycles: 5,
        mode: AddressMode::ZeroPage,
        function: CPU::ror,
    }, // 0x66
    Operation {
        name: "",
        code: 0x67,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x67
    Operation {
        name: "PLA",
        code: 0x68,
        length: 1,
        cycles: 4,
        mode: AddressMode::Implied,
        function: CPU::pla,
    }, // 0x68
    Operation {
        name: "ADC",
        code: 0x69,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::adc,
    }, // 0x69
    Operation {
        name: "ROR",
        code: 0x6A,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::ror,
    }, // 0x6A
    Operation {
        name: "",
        code: 0x6B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x6B
    Operation {
        name: "JMP",
        code: 0x6C,
        length: 3,
        cycles: 5,
        mode: AddressMode::Indirect,
        function: CPU::jmp,
    }, // 0x6C
    Operation {
        name: "ADC",
        code: 0x6D,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::adc,
    }, // 0x6D
    Operation {
        name: "ROR",
        code: 0x6E,
        length: 3,
        cycles: 6,
        mode: AddressMode::Absolute,
        function: CPU::ror,
    }, // 0x6E
    Operation {
        name: "",
        code: 0x6F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x6F
    Operation {
        name: "BVS",
        code: 0x70,
        length: 2,
        cycles: 2, /* maybe + 1 or + 2 */
        mode: AddressMode::Implied,
        function: CPU::bvs,
    }, // 0x70
    Operation {
        name: "ADC",
        code: 0x71,
        length: 2,
        cycles: 5, /* maybe + 1 */
        mode: AddressMode::IndirectY,
        function: CPU::adc,
    }, // 0x71
    Operation {
        name: "",
        code: 0x72,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x72
    Operation {
        name: "",
        code: 0x73,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x73
    Operation {
        name: "",
        code: 0x74,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x74
    Operation {
        name: "ADC",
        code: 0x75,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::adc,
    }, // 0x75
    Operation {
        name: "ROR",
        code: 0x76,
        length: 2,
        cycles: 6,
        mode: AddressMode::ZeroPageX,
        function: CPU::ror,
    }, // 0x76
    Operation {
        name: "",
        code: 0x77,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x77
    Operation {
        name: "SEI",
        code: 0x78,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::sei,
    }, // 0x78
    Operation {
        name: "ADC",
        code: 0x79,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteY,
        function: CPU::adc,
    }, // 0x79
    Operation {
        name: "",
        code: 0x7A,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x7A
    Operation {
        name: "",
        code: 0x7B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x7B
    Operation {
        name: "",
        code: 0x7C,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x7C
    Operation {
        name: "ADC",
        code: 0x7D,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteX,
        function: CPU::adc,
    }, // 0x7D
    Operation {
        name: "ROR",
        code: 0x7E,
        length: 3,
        cycles: 7,
        mode: AddressMode::AbsoluteX,
        function: CPU::ror,
    }, // 0x7E
    Operation {
        name: "",
        code: 0x7F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x7F
    Operation {
        name: "",
        code: 0x80,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x80
    Operation {
        name: "STA",
        code: 0x81,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectX,
        function: CPU::sta,
    }, // 0x81
    Operation {
        name: "",
        code: 0x82,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x82
    Operation {
        name: "",
        code: 0x83,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x83
    Operation {
        name: "STY",
        code: 0x84,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::sty,
    }, // 0x84
    Operation {
        name: "STA",
        code: 0x85,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::sta,
    }, // 0x85
    Operation {
        name: "STX",
        code: 0x86,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::stx,
    }, // 0x86
    Operation {
        name: "",
        code: 0x87,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x87
    Operation {
        name: "DEY",
        code: 0x88,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::dey,
    }, // 0x88
    Operation {
        name: "",
        code: 0x89,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x89
    Operation {
        name: "TXA",
        code: 0x8A,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::txa,
    }, // 0x8A
    Operation {
        name: "",
        code: 0x8B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x8B
    Operation {
        name: "STY",
        code: 0x8C,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::sty,
    }, // 0x8C
    Operation {
        name: "STA",
        code: 0x8D,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::sta,
    }, // 0x8D
    Operation {
        name: "STX",
        code: 0x8E,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::stx,
    }, // 0x8E
    Operation {
        name: "",
        code: 0x8F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x8F
    Operation {
        name: "BCC",
        code: 0x90,
        length: 2,
        cycles: 2, /* maybe + 1 or + 2 */
        mode: AddressMode::Implied,
        function: CPU::bcc,
    }, // 0x90
    Operation {
        name: "STA",
        code: 0x91,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectY,
        function: CPU::sta,
    }, // 0x91
    Operation {
        name: "",
        code: 0x92,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x92
    Operation {
        name: "",
        code: 0x93,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x93
    Operation {
        name: "STY",
        code: 0x94,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::sty,
    }, // 0x94
    Operation {
        name: "STA",
        code: 0x95,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::sta,
    }, // 0x95
    Operation {
        name: "STX",
        code: 0x96,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageY,
        function: CPU::stx,
    }, // 0x96
    Operation {
        name: "",
        code: 0x97,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x97
    Operation {
        name: "TYA",
        code: 0x98,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::tya,
    }, // 0x98
    Operation {
        name: "STA",
        code: 0x99,
        length: 3,
        cycles: 5,
        mode: AddressMode::AbsoluteX,
        function: CPU::sta,
    }, // 0x99
    Operation {
        name: "TXS",
        code: 0x9A,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::txs,
    }, // 0x9A
    Operation {
        name: "",
        code: 0x9B,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x9B
    Operation {
        name: "",
        code: 0x9C,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x9C
    Operation {
        name: "STA",
        code: 0x9D,
        length: 3,
        cycles: 5,
        mode: AddressMode::AbsoluteX,
        function: CPU::sta,
    }, // 0x9D
    Operation {
        name: "",
        code: 0x9E,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x9E
    Operation {
        name: "",
        code: 0x9F,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0x9F
    Operation {
        name: "LDY",
        code: 0xA0,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::ldy,
    }, // 0xA0
    Operation {
        name: "LDA",
        code: 0xA1,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectX,
        function: CPU::lda,
    }, // 0xA1
    Operation {
        name: "LDX",
        code: 0xA2,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::ldx,
    }, // 0xA2
    Operation {
        name: "",
        code: 0xA3,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xA3
    Operation {
        name: "LDY",
        code: 0xA4,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::ldy,
    }, // 0xA4
    Operation {
        name: "LDA",
        code: 0xA5,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::lda,
    }, // 0xA5
    Operation {
        name: "LDX",
        code: 0xA6,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::ldx,
    }, // 0xA6
    Operation {
        name: "",
        code: 0xA7,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xA7
    Operation {
        name: "TAY",
        code: 0xA8,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::tay,
    }, // 0xA8
    Operation {
        name: "LDA",
        code: 0xA9,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::lda,
    }, // 0xA9
    Operation {
        name: "TAX",
        code: 0xAA,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::tax,
    }, // 0xAA
    Operation {
        name: "",
        code: 0xAB,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xAB
    Operation {
        name: "LDY",
        code: 0xAC,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::ldy,
    }, // 0xAC
    Operation {
        name: "LDA",
        code: 0xAD,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::lda,
    }, // 0xAD
    Operation {
        name: "LDX",
        code: 0xAE,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::ldx,
    }, // 0xAE
    Operation {
        name: "",
        code: 0xAF,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xAF
    Operation {
        name: "BCS",
        code: 0xB0,
        length: 2,
        cycles: 2, /* maybe + 1 or + 2 */
        mode: AddressMode::Implied,
        function: CPU::bcs,
    }, // 0xB0
    Operation {
        name: "LDA",
        code: 0xB1,
        length: 2,
        cycles: 5, /* maybe + 1 */
        mode: AddressMode::IndirectY,
        function: CPU::lda,
    }, // 0xB1
    Operation {
        name: "",
        code: 0xB2,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xB2
    Operation {
        name: "",
        code: 0xB3,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xB3
    Operation {
        name: "LDY",
        code: 0xB4,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::ldy,
    }, // 0xB4
    Operation {
        name: "LDA",
        code: 0xB5,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::lda,
    }, // 0xB5
    Operation {
        name: "LDX",
        code: 0xB6,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageY,
        function: CPU::ldx,
    }, // 0xB6
    Operation {
        name: "",
        code: 0xB7,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xB7
    Operation {
        name: "CLV",
        code: 0xB8,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::clv,
    }, // 0xB8
    Operation {
        name: "LDA",
        code: 0xB9,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteY,
        function: CPU::lda,
    }, // 0xB9
    Operation {
        name: "TSX",
        code: 0xBA,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::tsx,
    }, // 0xBA
    Operation {
        name: "",
        code: 0xBB,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xBB
    Operation {
        name: "LDY",
        code: 0xBC,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteX,
        function: CPU::ldy,
    }, // 0xBC
    Operation {
        name: "LDA",
        code: 0xBD,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteX,
        function: CPU::lda,
    }, // 0xBD
    Operation {
        name: "LDX",
        code: 0xBE,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteY,
        function: CPU::ldx,
    }, // 0xBE
    Operation {
        name: "",
        code: 0xBF,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xBF
    Operation {
        name: "CPY",
        code: 0xC0,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::cpy,
    }, // 0xC0
    Operation {
        name: "CMP",
        code: 0xC1,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectX,
        function: CPU::cmp,
    }, // 0xC1
    Operation {
        name: "",
        code: 0xC2,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xC2
    Operation {
        name: "",
        code: 0xC3,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xC3
    Operation {
        name: "CPY",
        code: 0xC4,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::cpy,
    }, // 0xC4
    Operation {
        name: "CMP",
        code: 0xC5,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::cmp,
    }, // 0xC5
    Operation {
        name: "DEC",
        code: 0xC6,
        length: 2,
        cycles: 5,
        mode: AddressMode::ZeroPage,
        function: CPU::dec,
    }, // 0xC6
    Operation {
        name: "",
        code: 0xC7,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xC7
    Operation {
        name: "INY",
        code: 0xC8,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::iny,
    }, // 0xC8
    Operation {
        name: "CMP",
        code: 0xC9,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::nop,
    }, // 0xC9
    Operation {
        name: "DEX",
        code: 0xCA,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::dex,
    }, // 0xCA
    Operation {
        name: "",
        code: 0xCB,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xCB
    Operation {
        name: "CPY",
        code: 0xCC,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::cpy,
    }, // 0xCC
    Operation {
        name: "CMP",
        code: 0xCD,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::cmp,
    }, // 0xCD
    Operation {
        name: "DEC",
        code: 0xCE,
        length: 3,
        cycles: 6,
        mode: AddressMode::Absolute,
        function: CPU::dec,
    }, // 0xCE
    Operation {
        name: "",
        code: 0xCF,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xCF
    Operation {
        name: "BNE",
        code: 0xD0,
        length: 2,
        cycles: 2, /* maybe + 1 or + 2 */
        mode: AddressMode::Implied,
        function: CPU::bne,
    }, // 0xD0
    Operation {
        name: "CMP",
        code: 0xD1,
        length: 2,
        cycles: 5, /* maybe + 1 */
        mode: AddressMode::IndirectY,
        function: CPU::cmp,
    }, // 0xD1
    Operation {
        name: "",
        code: 0xD2,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xD2
    Operation {
        name: "",
        code: 0xD3,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xD3
    Operation {
        name: "",
        code: 0xD4,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xD4
    Operation {
        name: "CMP",
        code: 0xD5,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::cmp,
    }, // 0xD5
    Operation {
        name: "DEC",
        code: 0xD6,
        length: 2,
        cycles: 6,
        mode: AddressMode::ZeroPageX,
        function: CPU::dec,
    }, // 0xD6
    Operation {
        name: "",
        code: 0xD7,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xD7
    Operation {
        name: "CLD",
        code: 0xD8,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::cld,
    }, // 0xD8
    Operation {
        name: "CMP",
        code: 0xD9,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteY,
        function: CPU::cmp,
    }, // 0xD9
    Operation {
        name: "",
        code: 0xDA,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xDA
    Operation {
        name: "",
        code: 0xDB,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xDB
    Operation {
        name: "",
        code: 0xDC,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xDC
    Operation {
        name: "CMP",
        code: 0xDD,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteX,
        function: CPU::nop,
    }, // 0xDD
    Operation {
        name: "DEC",
        code: 0xDE,
        length: 3,
        cycles: 7,
        mode: AddressMode::AbsoluteX,
        function: CPU::dec,
    }, // 0xDE
    Operation {
        name: "",
        code: 0xDF,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xDF
    Operation {
        name: "CPX",
        code: 0xE0,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::cpx,
    }, // 0xE0
    Operation {
        name: "SBC",
        code: 0xE1,
        length: 2,
        cycles: 6,
        mode: AddressMode::IndirectX,
        function: CPU::sbc,
    }, // 0xE1
    Operation {
        name: "",
        code: 0xE2,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xE2
    Operation {
        name: "",
        code: 0xE3,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xE3
    Operation {
        name: "CPX",
        code: 0xE4,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::cpx,
    }, // 0xE4
    Operation {
        name: "SBC",
        code: 0xE5,
        length: 2,
        cycles: 3,
        mode: AddressMode::ZeroPage,
        function: CPU::sbc,
    }, // 0xE5
    Operation {
        name: "INC",
        code: 0xE6,
        length: 2,
        cycles: 5,
        mode: AddressMode::ZeroPage,
        function: CPU::inc,
    }, // 0xE6
    Operation {
        name: "",
        code: 0xE7,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xE7
    Operation {
        name: "INX",
        code: 0xE8,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::inx,
    }, // 0xE8
    Operation {
        name: "SBC",
        code: 0xE9,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::sbc,
    }, // 0xE9
    Operation {
        name: "NOP",
        code: 0xEA,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xEA
    Operation {
        name: "",
        code: 0xEB,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xEB
    Operation {
        name: "CPX",
        code: 0xEC,
        length: 2,
        cycles: 2,
        mode: AddressMode::Immediate,
        function: CPU::cpx,
    }, // 0xEC
    Operation {
        name: "SBC",
        code: 0xED,
        length: 3,
        cycles: 4,
        mode: AddressMode::Absolute,
        function: CPU::sbc,
    }, // 0xED
    Operation {
        name: "INC",
        code: 0xEE,
        length: 3,
        cycles: 6,
        mode: AddressMode::Absolute,
        function: CPU::inc,
    }, // 0xEE
    Operation {
        name: "",
        code: 0xEF,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xEF
    Operation {
        name: "BEQ",
        code: 0xF0,
        length: 2,
        cycles: 2, /* maybe + 1 or + 2 */
        mode: AddressMode::Implied,
        function: CPU::beq,
    }, // 0xF0
    Operation {
        name: "SBC",
        code: 0xF1,
        length: 2,
        cycles: 5, /* maybe + 1 */
        mode: AddressMode::IndirectY,
        function: CPU::sbc,
    }, // 0xF1
    Operation {
        name: "",
        code: 0xF2,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xF2
    Operation {
        name: "",
        code: 0xF3,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xF3
    Operation {
        name: "",
        code: 0xF4,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xF4
    Operation {
        name: "SBC",
        code: 0xF5,
        length: 2,
        cycles: 4,
        mode: AddressMode::ZeroPageX,
        function: CPU::sbc,
    }, // 0xF5
    Operation {
        name: "INC",
        code: 0xF6,
        length: 2,
        cycles: 6,
        mode: AddressMode::ZeroPageX,
        function: CPU::inc,
    }, // 0xF6
    Operation {
        name: "",
        code: 0xF7,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xF7
    Operation {
        name: "SED",
        code: 0xF8,
        length: 1,
        cycles: 2,
        mode: AddressMode::Implied,
        function: CPU::sed,
    }, // 0xF8
    Operation {
        name: "SBC",
        code: 0xF9,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteY,
        function: CPU::sbc,
    }, // 0xF9
    Operation {
        name: "",
        code: 0xFA,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xFA
    Operation {
        name: "",
        code: 0xFB,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xFB
    Operation {
        name: "",
        code: 0xFC,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xFC
    Operation {
        name: "SBC",
        code: 0xFD,
        length: 3,
        cycles: 4, /* maybe + 1 */
        mode: AddressMode::AbsoluteX,
        function: CPU::sbc,
    }, // 0xFD
    Operation {
        name: "INC",
        code: 0xFE,
        length: 3,
        cycles: 7,
        mode: AddressMode::AbsoluteX,
        function: CPU::inc,
    }, // 0xFE
    Operation {
        name: "",
        code: 0xFF,
        length: 1,
        cycles: 7,
        mode: AddressMode::Implied,
        function: CPU::nop,
    }, // 0xFF
];
