use super::registers::Registers;
use super::CPU;
use crate::memory::addressing::Addressing;
use crate::memory::addressing::Indirect;
use crate::memory::addressing::ReadMode;
use crate::memory::addressing::Relative;
use crate::memory::addressing::WriteMode;
use crate::memory::addressing::{
    Absolute, AbsoluteX, AbsoluteY, Accumulator, Immediate, IndexedIndirectX, IndirectIndexedY,
    ZeroPage, ZeroPageX, ZeroPageY, Implied
};
use crate::memory::Byte;
use crate::memory::DWord;
use crate::memory::Memory;
use crate::memory::Word;

pub trait InstructionDefinition<M: Memory, A> {
    fn cycles() -> usize;
    fn page_boundary() -> bool {
        false
    }
}

pub trait ImpliedInstruction<M: Memory> {
    fn execute(cpu: &mut CPU<M>);
}

pub trait ReadInstruction<M: Memory> {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>;
}

pub trait WriteInstruction<M: Memory> {
    fn execute<WM: WriteMode<M>, ID: InstructionDefinition<M, WM>>(cpu: &mut CPU<M>)
    where
        WM: Addressing<M>;
}

pub trait ReadWriteInstruction<M: Memory> {
    fn execute<RWM: ReadMode<M> + WriteMode<M>, ID: InstructionDefinition<M, RWM>>(cpu: &mut CPU<M>)
    where
        RWM: Addressing<M>;
}

macro_rules! instruction {
    ($instruction:ident, $mode:ident, $cycles:expr, $page_boundary:expr) => {
        impl<M: Memory> InstructionDefinition<M, $mode> for $instruction {
            fn cycles() -> usize {
                $cycles
            }

            fn page_boundary() -> bool {
                $page_boundary
            }
        }
    };
}

instruction!(ADC, Immediate, 2, false);
instruction!(ADC, ZeroPage, 3, false);
instruction!(ADC, ZeroPageX, 4, false);
instruction!(ADC, Absolute, 4, false);
instruction!(ADC, AbsoluteX, 4, true);
instruction!(ADC, AbsoluteY, 4, true);
instruction!(ADC, IndexedIndirectX, 6, false);
instruction!(ADC, IndirectIndexedY, 5, true);
pub struct ADC;
impl<M: Memory> ReadInstruction<M> for ADC {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value = rm.read(cpu);
        let result = cpu.registers.a as Word
            + value as Word
            + (cpu.registers.flags & Registers::CARRY_FLAG as Byte) as Word;

        cpu.registers.set_flag(Registers::CARRY_FLAG, result > 0xFF);

        let result = result as Byte;
        cpu.registers.set_flag(
            Registers::OVERFLOW_FLAG,
            (value ^ result) & (result ^ cpu.registers.a) & 0x80 != 0,
        );
        cpu.registers.set_a(result);
    }
}

instruction!(AND, Immediate, 2, false);
instruction!(AND, ZeroPage, 3, false);
instruction!(AND, ZeroPageX, 4, false);
instruction!(AND, Absolute, 4, false);
instruction!(AND, AbsoluteX, 4, true);
instruction!(AND, AbsoluteY, 4, true);
instruction!(AND, IndexedIndirectX, 6, false);
instruction!(AND, IndirectIndexedY, 5, true);
pub struct AND;
impl<M: Memory> ReadInstruction<M> for AND {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value = rm.read(cpu) & cpu.registers.a;
        cpu.registers.set_a(value);
    }
}

instruction!(ASL, Accumulator, 2, false);
instruction!(ASL, ZeroPage, 5, false);
instruction!(ASL, ZeroPageX, 6, false);
instruction!(ASL, Absolute, 6, false);
instruction!(ASL, AbsoluteX, 7, true);
pub struct ASL;
impl<M: Memory> ReadWriteInstruction<M> for ASL {
    fn execute<RWM: ReadMode<M> + WriteMode<M>, ID: InstructionDefinition<M, RWM>>(cpu: &mut CPU<M>)
    where
        RWM: Addressing<M>,
    {
        cpu.shift_left::<RWM, ID>(false);
    }
}

instruction!(BCC, Relative, 2, false);
pub struct BCC;
impl<M: Memory> ImpliedInstruction<M> for BCC {
    fn execute(cpu: &mut CPU<M>) {
        cpu.branch(!cpu.registers.get_flag(Registers::CARRY_FLAG));
    }
}

instruction!(BCS, Relative, 2, false);
pub struct BCS;
impl<M: Memory> ImpliedInstruction<M> for BCS {
    fn execute(cpu: &mut CPU<M>) {
        cpu.branch(cpu.registers.get_flag(Registers::CARRY_FLAG));
    }
}

instruction!(BEQ, Relative, 2, false);
pub struct BEQ;
impl<M: Memory> ImpliedInstruction<M> for BEQ {
    fn execute(cpu: &mut CPU<M>) {
        cpu.branch(cpu.registers.get_flag(Registers::ZERO_FLAG));
    }
}

instruction!(BIT, ZeroPage, 3, false);
instruction!(BIT, Absolute, 4, false);
pub struct BIT;
impl<M: Memory> ReadInstruction<M> for BIT {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value = rm.read(cpu);
        cpu.registers
            .set_flag(Registers::ZERO_FLAG, (value & cpu.registers.a) == 0);
        cpu.registers
            .set_flag(Registers::NEGATIVE_FLAG, (value & 0x80) != 0);
        cpu.registers
            .set_flag(Registers::OVERFLOW_FLAG, (value & 0x40) != 0);
    }
}

instruction!(BMI, Relative, 2, false);
pub struct BMI;
impl<M: Memory> ImpliedInstruction<M> for BMI {
    fn execute(cpu: &mut CPU<M>) {
        cpu.branch(cpu.registers.get_flag(Registers::NEGATIVE_FLAG));
    }
}

instruction!(BNE, Relative, 2, false);
pub struct BNE;
impl<M: Memory> ImpliedInstruction<M> for BNE {
    fn execute(cpu: &mut CPU<M>) {
        cpu.branch(!cpu.registers.get_flag(Registers::ZERO_FLAG));
    }
}

instruction!(BPL, Relative, 2, false);
pub struct BPL;
impl<M: Memory> ImpliedInstruction<M> for BPL {
    fn execute(cpu: &mut CPU<M>) {
        cpu.branch(!cpu.registers.get_flag(Registers::NEGATIVE_FLAG));
    }
}

// BRK

instruction!(BVC, Relative, 2, false);
pub struct BVC;
impl<M: Memory> ImpliedInstruction<M> for BVC {
    fn execute(cpu: &mut CPU<M>) {
        cpu.branch(!cpu.registers.get_flag(Registers::OVERFLOW_FLAG));
    }
}

instruction!(BVS, Relative, 2, false);
pub struct BVS;
impl<M: Memory> ImpliedInstruction<M> for BVS {
    fn execute(cpu: &mut CPU<M>) {
        cpu.branch(cpu.registers.get_flag(Registers::OVERFLOW_FLAG));
    }
}

instruction!(CLC, Implied, 2, false);
pub struct CLC;
impl<M: Memory> ImpliedInstruction<M> for CLC {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_flag(Registers::CARRY_FLAG, false);
    }
}

instruction!(CLD, Implied, 2, false);
pub struct CLD;
impl<M: Memory> ImpliedInstruction<M> for CLD {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_flag(Registers::DECIMAL_FLAG, false);
    }
}

instruction!(CLI, Implied, 2, false);
pub struct CLI;
impl<M: Memory> ImpliedInstruction<M> for CLI {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_flag(Registers::IRQ_FLAG, false);
    }
}

instruction!(CLV, Implied, 2, false);
pub struct CLV;
impl<M: Memory> ImpliedInstruction<M> for CLV {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_flag(Registers::OVERFLOW_FLAG, false);
    }
}

instruction!(CMP, Immediate, 2, false);
instruction!(CMP, ZeroPage, 3, false);
instruction!(CMP, ZeroPageX, 4, false);
instruction!(CMP, Absolute, 4, false);
instruction!(CMP, AbsoluteX, 4, true);
instruction!(CMP, AbsoluteY, 4, true);
instruction!(CMP, IndexedIndirectX, 6, false);
instruction!(CMP, IndirectIndexedY, 5, true);
pub struct CMP;
impl<M: Memory> ReadInstruction<M> for CMP {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        cpu.compare::<RM, ID>(cpu.registers.a);
    }
}

instruction!(CPX, Immediate, 2, false);
instruction!(CPX, ZeroPage, 3, false);
instruction!(CPX, Absolute, 4, false);
pub struct CPX;
impl<M: Memory> ReadInstruction<M> for CPX {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        cpu.compare::<RM, ID>(cpu.registers.x);
    }
}

instruction!(CPY, Immediate, 2, false);
instruction!(CPY, ZeroPage, 3, false);
instruction!(CPY, Absolute, 4, false);
pub struct CPY;
impl<M: Memory> ReadInstruction<M> for CPY {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        cpu.compare::<RM, ID>(cpu.registers.y);
    }
}

instruction!(DEC, ZeroPage, 5, false);
instruction!(DEC, ZeroPageX, 6, false);
instruction!(DEC, Absolute, 6, false);
instruction!(DEC, AbsoluteX, 7, false);
pub struct DEC;
impl<M: Memory> ReadWriteInstruction<M> for DEC {
    fn execute<RWM: ReadMode<M> + WriteMode<M>, ID: InstructionDefinition<M, RWM>>(cpu: &mut CPU<M>)
    where
        RWM: Addressing<M>,
    {
        let rwm = RWM::create_addressing(cpu, ID::page_boundary());
        let value = rwm.read(cpu);
        let value = cpu.registers.set_zn(value.wrapping_sub(0x01));
        rwm.write(cpu, value);
    }
}

instruction!(DEX, Implied, 2, false);
pub struct DEX;
impl<M: Memory> ImpliedInstruction<M> for DEX {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_x(cpu.registers.x.wrapping_sub(0x01));
    }
}

instruction!(DEY, Implied, 2, false);
pub struct DEY;
impl<M: Memory> ImpliedInstruction<M> for DEY {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_y(cpu.registers.y.wrapping_sub(0x01));
    }
}

instruction!(EOR, Immediate, 2, false);
instruction!(EOR, ZeroPage, 3, false);
instruction!(EOR, ZeroPageX, 4, false);
instruction!(EOR, Absolute, 4, false);
instruction!(EOR, AbsoluteX, 4, true);
instruction!(EOR, AbsoluteY, 4, true);
instruction!(EOR, IndexedIndirectX, 6, false);
instruction!(EOR, IndirectIndexedY, 5, true);
pub struct EOR;
impl<M: Memory> ReadInstruction<M> for EOR {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value = rm.read(cpu);
        cpu.registers.set_a(value ^ cpu.registers.a);
    }
}

instruction!(INC, ZeroPage, 5, false);
instruction!(INC, ZeroPageX, 6, false);
instruction!(INC, Absolute, 6, false);
instruction!(INC, AbsoluteX, 7, true);
pub struct INC;
impl<M: Memory> ReadWriteInstruction<M> for INC {
    fn execute<RWM: ReadMode<M> + WriteMode<M>, ID: InstructionDefinition<M, RWM>>(cpu: &mut CPU<M>)
    where
        RWM: Addressing<M>,
    {
        let rwm = RWM::create_addressing(cpu, ID::page_boundary());
        let value = rwm.read(cpu);
        let value = cpu.registers.set_zn(value.wrapping_add(0x01));
        rwm.write(cpu, value);
    }
}

instruction!(INX, Implied, 2, false);
pub struct INX;
impl<M: Memory> ImpliedInstruction<M> for INX {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_x(cpu.registers.x.wrapping_add(0x01));
    }
}

instruction!(INY, Implied, 2, false);
pub struct INY;
impl<M: Memory> ImpliedInstruction<M> for INY {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_y(cpu.registers.y.wrapping_add(0x01));
    }
}

instruction!(JMP, Absolute, 3, false);
pub struct JMP;
impl<M: Memory> ImpliedInstruction<M> for JMP {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.pc = cpu.read_next_word();
    }
}

instruction!(JMPI, Indirect, 5, false);
pub struct JMPI;
impl<M: Memory> ImpliedInstruction<M> for JMPI {
    fn execute(cpu: &mut CPU<M>) {
        let address = cpu.read_next_word();

        let low = cpu.read_byte(address);
        let high = cpu.read_byte((address & 0xff00) | ((address + 0x0001) & 0x00ff));

        cpu.registers.pc = (high as Word) << 8 | low as Word
    }
}

instruction!(JSR, Absolute, 6, false);
pub struct JSR;
impl<M: Memory> ImpliedInstruction<M> for JSR {
    fn execute(cpu: &mut CPU<M>) {
        cpu.push_word(cpu.registers.pc + 0x0001);
        cpu.registers.pc = cpu.read_word(cpu.registers.pc);
    }
}

instruction!(LDA, Immediate, 2, false);
instruction!(LDA, ZeroPage, 3, false);
instruction!(LDA, ZeroPageX, 4, false);
instruction!(LDA, Absolute, 4, false);
instruction!(LDA, AbsoluteX, 4, true);
instruction!(LDA, AbsoluteY, 4, true);
instruction!(LDA, IndexedIndirectX, 6, false);
instruction!(LDA, IndirectIndexedY, 5, true);
pub struct LDA;
impl<M: Memory> ReadInstruction<M> for LDA {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value: Byte = rm.read(cpu);
        cpu.registers.set_a(value);
    }
}

instruction!(LDX, Immediate, 2, false);
instruction!(LDX, ZeroPage, 3, false);
instruction!(LDX, ZeroPageY, 4, false);
instruction!(LDX, Absolute, 4, false);
instruction!(LDX, AbsoluteY, 4, true);
pub struct LDX;
impl<M: Memory> ReadInstruction<M> for LDX {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value: Byte = rm.read(cpu);
        cpu.registers.set_x(value);
    }
}

instruction!(LDY, Immediate, 2, false);
instruction!(LDY, ZeroPage, 3, false);
instruction!(LDY, ZeroPageX, 4, false);
instruction!(LDY, Absolute, 4, false);
instruction!(LDY, AbsoluteX, 4, true);
pub struct LDY;
impl<M: Memory> ReadInstruction<M> for LDY {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value: Byte = rm.read(cpu);
        cpu.registers.set_y(value);
    }
}

instruction!(LSR, Accumulator, 2, false);
instruction!(LSR, ZeroPage, 5, false);
instruction!(LSR, ZeroPageX, 6, false);
instruction!(LSR, Absolute, 6, false);
instruction!(LSR, AbsoluteX, 7, true);
pub struct LSR;
impl<M: Memory> ReadWriteInstruction<M> for LSR {
    fn execute<RWM: ReadMode<M> + WriteMode<M>, ID:  InstructionDefinition<M, RWM>>(cpu: &mut CPU<M>)
    where
        RWM: Addressing<M>,
    {
        cpu.shift_right::<RWM, ID>(false);
    }
}

instruction!(NOP, Implied, 2, false);
pub struct NOP;
impl<M: Memory> ImpliedInstruction<M> for NOP {
    fn execute(_cpu: &mut CPU<M>) {}
}

instruction!(ORA, Immediate, 2, false);
instruction!(ORA, ZeroPage, 3, false);
instruction!(ORA, ZeroPageX, 4, false);
instruction!(ORA, Absolute, 4, false);
instruction!(ORA, AbsoluteX, 4, true);
instruction!(ORA, AbsoluteY, 4, true);
instruction!(ORA, IndexedIndirectX, 6, false);
instruction!(ORA, IndirectIndexedY, 5, true);
pub struct ORA;
impl<M: Memory> ReadInstruction<M> for ORA {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M> + ,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value: Byte = rm.read(cpu) | cpu.registers.a;
        cpu.registers.set_a(value);
    }
}

instruction!(PHA, Implied, 3, false);
pub struct PHA;
impl<M: Memory> ImpliedInstruction<M> for PHA {
    fn execute(cpu: &mut CPU<M>) {
        cpu.push_byte(cpu.registers.a);
    }
}

instruction!(PHP, Implied, 3, false);
pub struct PHP;
impl<M: Memory> ImpliedInstruction<M> for PHP {
    fn execute(cpu: &mut CPU<M>) {
        cpu.push_byte(cpu.registers.flags | Registers::BREAK_FLAG);
    }
}

instruction!(PLA, Implied, 4, false);
pub struct PLA;
impl<M: Memory> ImpliedInstruction<M> for PLA {
    fn execute(cpu: &mut CPU<M>) {
        let value = cpu.pop_byte();
        cpu.registers.set_a(value);
    }
}

instruction!(PLP, Implied, 4, false);
pub struct PLP;
impl<M: Memory> ImpliedInstruction<M> for PLP {
    fn execute(cpu: &mut CPU<M>) {
        let flags = cpu.pop_byte();
        cpu.registers.set_flags(flags);
    }
}

instruction!(ROL, Accumulator, 2, false);
instruction!(ROL, ZeroPage, 5, false);
instruction!(ROL, ZeroPageX, 6, false);
instruction!(ROL, Absolute, 6, false);
instruction!(ROL, AbsoluteX, 7, true);
pub struct ROL;
impl<M: Memory> ReadWriteInstruction<M> for ROL {
    fn execute<RWM: ReadMode<M> + WriteMode<M>, ID: InstructionDefinition<M, RWM>>(cpu: &mut CPU<M>)
    where
        RWM: Addressing<M>,
    {
        cpu.shift_left::<RWM, ID>(cpu.registers.get_flag(Registers::CARRY_FLAG));
    }
}

instruction!(ROR, Accumulator, 2, false);
instruction!(ROR, ZeroPage, 5, false);
instruction!(ROR, ZeroPageX, 6, false);
instruction!(ROR, Absolute, 6, false);
instruction!(ROR, AbsoluteX, 7, true);
pub struct ROR;
impl<M: Memory> ReadWriteInstruction<M> for ROR {
    fn execute<RWM: ReadMode<M> + WriteMode<M>, ID: InstructionDefinition<M, RWM>>(cpu: &mut CPU<M>)
    where
        RWM: Addressing<M>,
    {
        cpu.shift_right::<RWM, ID>(cpu.registers.get_flag(Registers::CARRY_FLAG));
    }
}

instruction!(RTI, Implied, 6, false);
pub struct RTI;
impl<M: Memory> ImpliedInstruction<M> for RTI {
    fn execute(cpu: &mut CPU<M>) {
        let flags = cpu.pop_byte();
        cpu.registers.set_flags(flags);
        cpu.registers.pc = cpu.pop_word();
    }
}

instruction!(RTS, Implied, 6, false);
pub struct RTS;
impl<M: Memory> ImpliedInstruction<M> for RTS {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.pc = cpu.pop_word() + 0x0001;
    }
}

instruction!(SBC, Immediate, 2, false);
instruction!(SBC, ZeroPage, 3, false);
instruction!(SBC, ZeroPageX, 4, false);
instruction!(SBC, Absolute, 4, false);
instruction!(SBC, AbsoluteX, 4, true);
instruction!(SBC, AbsoluteY, 4, true);
instruction!(SBC, IndexedIndirectX, 6, false);
instruction!(SBC, IndirectIndexedY, 5, true);
pub struct SBC;
impl<M: Memory> ReadInstruction<M> for SBC {
    fn execute<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(cpu: &mut CPU<M>)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(cpu, ID::page_boundary());
        let value = rm.read(cpu);
        let mut result = (cpu.registers.a as DWord).wrapping_sub(value as DWord);
        if !cpu.registers.get_flag(Registers::CARRY_FLAG) {
            result = result.wrapping_sub(0x0000_0001);
        }

        cpu.registers
            .set_flag(Registers::CARRY_FLAG, (result & 0x0100) == 0);

        let result = result as Byte;
        cpu.registers.set_flag(
            Registers::OVERFLOW_FLAG,
            (cpu.registers.a ^ result) & 0x80 != 0 && (cpu.registers.a ^ value) & 0x80 == 0x80,
        );
        cpu.registers.set_a(result);
    }
}

instruction!(SEC, Implied, 2, false);
pub struct SEC;
impl<M: Memory> ImpliedInstruction<M> for SEC {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_flag(Registers::CARRY_FLAG, true);
    }
}

instruction!(SED, Implied, 2, false);
pub struct SED;
impl<M: Memory> ImpliedInstruction<M> for SED {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_flag(Registers::DECIMAL_FLAG, true);
    }
}

instruction!(SEI, Implied, 2, false);
pub struct SEI;
impl<M: Memory> ImpliedInstruction<M> for SEI {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_flag(Registers::IRQ_FLAG, true);
    }
}

instruction!(STA, ZeroPage, 3, false);
instruction!(STA, ZeroPageX, 4, false);
instruction!(STA, Absolute, 4, false);
instruction!(STA, AbsoluteX, 5, false);
instruction!(STA, AbsoluteY, 5, false);
instruction!(STA, IndexedIndirectX, 6, false);
instruction!(STA, IndirectIndexedY, 6, false);
pub struct STA;
impl<M: Memory> WriteInstruction<M> for STA {
    fn execute<WM: WriteMode<M>, ID: InstructionDefinition<M, WM>>(cpu: &mut CPU<M>)
    where
        WM: Addressing<M>,
    {
        let wm = WM::create_addressing(cpu, ID::page_boundary());
        wm.write(cpu, cpu.registers.a);
    }
}

instruction!(STX, ZeroPage, 3, false);
instruction!(STX, ZeroPageY, 4, false);
instruction!(STX, Absolute, 4, false);
pub struct STX;
impl<M: Memory> WriteInstruction<M> for STX {
    fn execute<WM: WriteMode<M>, ID: InstructionDefinition<M, WM>>(cpu: &mut CPU<M>)
    where
        WM: Addressing<M>,
    {
        let wm = WM::create_addressing(cpu, ID::page_boundary());
        wm.write(cpu, cpu.registers.x);
    }
}

instruction!(STY, ZeroPage, 3, false);
instruction!(STY, ZeroPageX, 4, false);
instruction!(STY, Absolute, 4, false);
pub struct STY;
impl<M: Memory> WriteInstruction<M> for STY {
    fn execute<WM: WriteMode<M>, ID: InstructionDefinition<M, WM>>(cpu: &mut CPU<M>)
    where
        WM: Addressing<M> + ,
    {
        let wm = WM::create_addressing(cpu, ID::page_boundary());
        wm.write(cpu, cpu.registers.y);
    }
}

instruction!(TAX, Implied, 2, false);
pub struct TAX;
impl<M: Memory> ImpliedInstruction<M> for TAX {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_x(cpu.registers.a);
    }
}

instruction!(TAY, Implied, 2, false);
pub struct TAY;
impl<M: Memory> ImpliedInstruction<M> for TAY {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_y(cpu.registers.a);
    }
}

instruction!(TSX, Implied, 2, false);
pub struct TSX;
impl<M: Memory> ImpliedInstruction<M> for TSX {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_x(cpu.registers.sp);
    }
}

instruction!(TXA, Implied, 2, false);
pub struct TXA;
impl<M: Memory> ImpliedInstruction<M> for TXA {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_a(cpu.registers.x);
    }
}

instruction!(TXS, Implied, 2, false);
pub struct TXS;
impl<M: Memory> ImpliedInstruction<M> for TXS {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.sp = cpu.registers.x;
    }
}

instruction!(TYA, Implied, 2, false);
pub struct TYA;
impl<M: Memory> ImpliedInstruction<M> for TYA {
    fn execute(cpu: &mut CPU<M>) {
        cpu.registers.set_a(cpu.registers.y);
    }
}
