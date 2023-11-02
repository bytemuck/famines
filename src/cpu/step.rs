use crate::memory::{Memory, addressing::{Relative, Implied, Indirect}};

use super::{
    instructions::{
        ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BVC, BVS, CLC, CLD, CLI, CLV, CMP, CPX,
        CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JMPI, JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA,
        PHP, PLA, PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA,
        TXS, TYA,
    },
    CPU,
};
use crate::memory::addressing::{
    Absolute, AbsoluteX, AbsoluteY, Accumulator, Immediate, IndexedIndirectX, IndirectIndexedY,
    ZeroPage, ZeroPageX, ZeroPageY,
};

impl<M: Memory> CPU<M> {
    pub fn trace(&mut self) {
        println!(
            "{:04X} OP:{:02X} A:{:02X} X:{:02X} Y:{:02X} FLAGS:{:02X} SP:{:02X} CYC:{}",
            self.registers.pc as usize,
            self.read_byte(self.registers.pc) as usize,
            self.registers.a as usize,
            self.registers.x as usize,
            self.registers.y as usize,
            self.registers.flags as usize,
            self.registers.sp as usize,
            self.cycles as usize,
        );
    }

    pub fn step(&mut self) -> bool {
        self.trace();
        let opcode = self.read_next_byte();
        let mut success = true;

        match opcode {
            // ADC
            0x69 => self.execute_read::<ADC, Immediate>(),
            0x65 => self.execute_read::<ADC, ZeroPage>(),
            0x75 => self.execute_read::<ADC, ZeroPageX>(),
            0x6d => self.execute_read::<ADC, Absolute>(),
            0x7d => self.execute_read::<ADC, AbsoluteX>(),
            0x79 => self.execute_read::<ADC, AbsoluteY>(),
            0x61 => self.execute_read::<ADC, IndexedIndirectX>(),
            0x71 => self.execute_read::<ADC, IndirectIndexedY>(),

            // AND
            0x29 => self.execute_read::<AND, Immediate>(),
            0x25 => self.execute_read::<AND, ZeroPage>(),
            0x35 => self.execute_read::<AND, ZeroPageX>(),
            0x2d => self.execute_read::<AND, Absolute>(),
            0x3d => self.execute_read::<AND, AbsoluteX>(),
            0x39 => self.execute_read::<AND, AbsoluteY>(),
            0x21 => self.execute_read::<AND, IndexedIndirectX>(),
            0x31 => self.execute_read::<AND, IndirectIndexedY>(),

            // ASL
            0x0a => self.execute_read_write::<ASL, Accumulator>(),
            0x06 => self.execute_read_write::<ASL, ZeroPage>(),
            0x16 => self.execute_read_write::<ASL, ZeroPageX>(),
            0x0e => self.execute_read_write::<ASL, Absolute>(),
            0x1e => self.execute_read_write::<ASL, AbsoluteX>(),

            // BCC
            0x90 => self.execute::<BCC, Relative>(),

            // BCS
            0xb0 => self.execute::<BCS, Relative>(),

            // BEQ
            0xf0 => self.execute::<BEQ, Relative>(),

            // BIT
            0x24 => self.execute_read::<BIT, ZeroPage>(),
            0x2c => self.execute_read::<BIT, Absolute>(),

            // BMI
            0x30 => self.execute::<BMI, Relative>(),

            // BNE
            0xd0 => self.execute::<BNE, Relative>(),

            // BPL
            0x10 => self.execute::<BPL, Relative>(),

            // BVC
            0x50 => self.execute::<BVC, Relative>(),

            // BVS
            0x70 => self.execute::<BVS, Relative>(),

            // CLC
            0x18 => self.execute::<CLC, Implied>(),

            // CLD
            0xd8 => self.execute::<CLD, Implied>(),

            // CLI
            0x58 => self.execute::<CLI, Implied>(),

            // CLV
            0xb8 => self.execute::<CLV, Implied>(),

            // CMP
            0xc9 => self.execute_read::<CMP, Immediate>(),
            0xc5 => self.execute_read::<CMP, ZeroPage>(),
            0xd5 => self.execute_read::<CMP, ZeroPageX>(),
            0xcd => self.execute_read::<CMP, Absolute>(),
            0xdd => self.execute_read::<CMP, AbsoluteX>(),
            0xd9 => self.execute_read::<CMP, AbsoluteY>(),
            0xc1 => self.execute_read::<CMP, IndexedIndirectX>(),
            0xd1 => self.execute_read::<CMP, IndirectIndexedY>(),

            // CMX
            0xe0 => self.execute_read::<CPX, Immediate>(),
            0xe4 => self.execute_read::<CPX, ZeroPage>(),
            0xec => self.execute_read::<CPX, Absolute>(),

            // CMY
            0xc0 => self.execute_read::<CPY, Immediate>(),
            0xc4 => self.execute_read::<CPY, ZeroPage>(),
            0xcc => self.execute_read::<CPY, Absolute>(),

            // DEC
            0xc6 => self.execute_read_write::<DEC, ZeroPage>(),
            0xd6 => self.execute_read_write::<DEC, ZeroPageX>(),
            0xce => self.execute_read_write::<DEC, Absolute>(),
            0xde => self.execute_read_write::<DEC, AbsoluteX>(),

            // DEX
            0xca => self.execute::<DEX, Implied>(),

            // DEY
            0x88 => self.execute::<DEY, Implied>(),

            // EOR
            0x49 => self.execute_read::<EOR, Immediate>(),
            0x45 => self.execute_read::<EOR, ZeroPage>(),
            0x55 => self.execute_read::<EOR, ZeroPageX>(),
            0x4d => self.execute_read::<EOR, Absolute>(),
            0x5d => self.execute_read::<EOR, AbsoluteX>(),
            0x59 => self.execute_read::<EOR, AbsoluteY>(),
            0x41 => self.execute_read::<EOR, IndexedIndirectX>(),
            0x51 => self.execute_read::<EOR, IndirectIndexedY>(),

            // DEC
            0xe6 => self.execute_read_write::<INC, ZeroPage>(),
            0xf6 => self.execute_read_write::<INC, ZeroPageX>(),
            0xee => self.execute_read_write::<INC, Absolute>(),
            0xfe => self.execute_read_write::<INC, AbsoluteX>(),

            // INX
            0xe8 => self.execute::<INX, Implied>(),

            // INY
            0xc8 => self.execute::<INY, Implied>(),

            // JMP
            0x4c => self.execute::<JMP, Absolute>(),
            0x6c => self.execute::<JMPI, Indirect>(),

            // JSR
            0x20 => self.execute::<JSR, Absolute>(),

            // LDA
            0xa9 => self.execute_read::<LDA, Immediate>(),
            0xa5 => self.execute_read::<LDA, ZeroPage>(),
            0xb5 => self.execute_read::<LDA, ZeroPageX>(),
            0xad => self.execute_read::<LDA, Absolute>(),
            0xbd => self.execute_read::<LDA, AbsoluteX>(),
            0xb9 => self.execute_read::<LDA, AbsoluteY>(),
            0xa1 => self.execute_read::<LDA, IndexedIndirectX>(),
            0xb1 => self.execute_read::<LDA, IndirectIndexedY>(),

            // LDX
            0xa2 => self.execute_read::<LDX, Immediate>(),
            0xa6 => self.execute_read::<LDX, ZeroPage>(),
            0xb6 => self.execute_read::<LDX, ZeroPageY>(),
            0xae => self.execute_read::<LDX, Absolute>(),
            0xbe => self.execute_read::<LDX, AbsoluteY>(),

            // LDY
            0xa0 => self.execute_read::<LDY, Immediate>(),
            0xa4 => self.execute_read::<LDY, ZeroPage>(),
            0xb4 => self.execute_read::<LDY, ZeroPageX>(),
            0xac => self.execute_read::<LDY, Absolute>(),
            0xbc => self.execute_read::<LDY, AbsoluteX>(),

            // LSR
            0x4a => self.execute_read_write::<LSR, Accumulator>(),
            0x46 => self.execute_read_write::<LSR, ZeroPage>(),
            0x56 => self.execute_read_write::<LSR, ZeroPageX>(),
            0x4e => self.execute_read_write::<LSR, Absolute>(),
            0x5e => self.execute_read_write::<LSR, AbsoluteX>(),

            // NOP
            0xea => self.execute::<NOP, Implied>(),

            // ORA
            0x09 => self.execute_read::<ORA, Immediate>(),
            0x05 => self.execute_read::<ORA, ZeroPage>(),
            0x15 => self.execute_read::<ORA, ZeroPageX>(),
            0x0d => self.execute_read::<ORA, Absolute>(),
            0x1d => self.execute_read::<ORA, AbsoluteX>(),
            0x19 => self.execute_read::<ORA, AbsoluteY>(),
            0x01 => self.execute_read::<ORA, IndexedIndirectX>(),
            0x11 => self.execute_read::<ORA, IndirectIndexedY>(),

            // PHA
            0x48 => self.execute::<PHA, Implied>(),

            // PHP
            0x08 => self.execute::<PHP, Implied>(),

            // PLA
            0x68 => self.execute::<PLA, Implied>(),

            // PLP
            0x28 => self.execute::<PLP, Implied>(),

            // ROL
            0x2a => self.execute_read_write::<ROL, Accumulator>(),
            0x26 => self.execute_read_write::<ROL, ZeroPage>(),
            0x36 => self.execute_read_write::<ROL, ZeroPageX>(),
            0x2e => self.execute_read_write::<ROL, Absolute>(),
            0x3e => self.execute_read_write::<ROL, AbsoluteX>(),

            // ROR
            0x6a => self.execute_read_write::<ROR, Accumulator>(),
            0x66 => self.execute_read_write::<ROR, ZeroPage>(),
            0x76 => self.execute_read_write::<ROR, ZeroPageX>(),
            0x6e => self.execute_read_write::<ROR, Absolute>(),
            0x7e => self.execute_read_write::<ROR, AbsoluteX>(),

            // RTI
            0x40 => self.execute::<RTI, Implied>(),

            // RTS
            0x60 => self.execute::<RTS, Implied>(),

            // SBC
            0xe9 => self.execute_read::<SBC, Immediate>(),
            0xe5 => self.execute_read::<SBC, ZeroPage>(),
            0xf5 => self.execute_read::<SBC, ZeroPageX>(),
            0xed => self.execute_read::<SBC, Absolute>(),
            0xfd => self.execute_read::<SBC, AbsoluteX>(),
            0xf9 => self.execute_read::<SBC, AbsoluteY>(),
            0xe1 => self.execute_read::<SBC, IndexedIndirectX>(),
            0xf1 => self.execute_read::<SBC, IndirectIndexedY>(),

            // SEC
            0x38 => self.execute::<SEC, Implied>(),

            // SED
            0xf8 => self.execute::<SED, Implied>(),

            // SEI
            0x78 => self.execute::<SEI, Implied>(),

            // STA
            0x85 => self.execute_write::<STA, ZeroPage>(),
            0x95 => self.execute_write::<STA, ZeroPageX>(),
            0x8d => self.execute_write::<STA, Absolute>(),
            0x9d => self.execute_write::<STA, AbsoluteX>(),
            0x99 => self.execute_write::<STA, AbsoluteY>(),
            0x81 => self.execute_write::<STA, IndexedIndirectX>(),
            0x91 => self.execute_write::<STA, IndirectIndexedY>(),

            // STX
            0x86 => self.execute_write::<STX, ZeroPage>(),
            0x96 => self.execute_write::<STX, ZeroPageY>(),
            0x8e => self.execute_write::<STX, Absolute>(),

            // STY
            0x84 => self.execute_write::<STY, ZeroPage>(),
            0x94 => self.execute_write::<STY, ZeroPageX>(),
            0x8c => self.execute_write::<STY, Absolute>(),

            // TAX
            0xaa => self.execute::<TAX, Implied>(),

            // TAY
            0xa8 => self.execute::<TAY, Implied>(),

            // TSX
            0xba => self.execute::<TSX, Implied>(),

            // TXA
            0x8a => self.execute::<TXA, Implied>(),

            // TXS
            0x9a => self.execute::<TXS, Implied>(),

            // TYA
            0x98 => self.execute::<TYA, Implied>(),

            _ => {
                success = false;
            }
        }

        success
    }
}
