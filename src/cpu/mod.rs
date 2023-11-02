use crate::memory::addressing::Addressing;
use crate::memory::addressing::ReadMode;
use crate::memory::addressing::WriteMode;
use crate::{
    cartridge::Cartridge,
    memory::{Address, Byte, DWord, Memory, Offset, Word, ZeroPageAddress, ZeroPageMemory},
};

use self::instructions::InstructionDefinition;
use self::{
    instructions::{ImpliedInstruction, ReadInstruction, ReadWriteInstruction, WriteInstruction},
    registers::Registers,
};

pub mod instructions;
pub mod registers;
pub mod step;

pub struct CPU<M: Memory> {
    pub registers: Registers,
    pub memory: M,
    pub cartridge: Cartridge,
    pub cycles: usize,
}

impl<M: Memory> Memory for CPU<M> {
    fn read_byte(&mut self, address: Address) -> Byte {
        match address {
            0x0000..=0x1fff => self.memory.read_byte(address),
            0x8000..=0xffff => self.cartridge.read_byte(address - 0x8000),
            _ => {
                println!("Ignoring memory access (read) at {:04X}", address);
                0x000
            }
        }
    }

    fn write_byte(&mut self, address: Address, value: Byte) {
        match address {
            0x0000..=0x1fff => self.memory.write_byte(address, value),
            0x8000..=0xffff => panic!("Cannot write on cartridge."),
            _ => {
                println!("Ignoring memory access (write) at {:04X}", address)
            }
        }
    }
}

impl<M: Memory> ZeroPageMemory for CPU<M> {
    fn read_word_zero_page(&mut self, address: ZeroPageAddress) -> Word {
        let low = self.read_byte(address as Word) as Word;
        let high = self.read_byte((address as Word + 1) as Word) as Word;

        low | high << 8
    }
}

impl<M: Memory> CPU<M> {
    pub fn new(cartridge: Cartridge, memory: M) -> Self {
        Self {
            registers: Registers::new(),
            memory,
            cartridge,
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.registers.a = 0;
        self.registers.x = 0;
        self.registers.y = 0;

        self.registers.sp = Registers::STACK_RESET;
        self.registers.pc = self.read_word(Registers::RESET_VECTOR);

        self.registers.flags = Registers::IRQ_FLAG | Registers::UNUSED_FLAG;
    }

    pub fn push_byte(&mut self, value: u8) {
        self.write_byte(Registers::STACK + self.registers.sp as Word, value);
        self.registers.sp -= 0x0001;
    }

    pub fn push_word(&mut self, value: Word) {
        let high = (value >> 8) as Byte;
        let low = (value & 0xFF) as Byte;
        self.push_byte(high);
        self.push_byte(low);
    }

    pub fn pop_byte(&mut self) -> Byte {
        self.registers.sp += 1;
        self.read_byte(Registers::STACK + self.registers.sp as Word)
    }

    pub fn pop_word(&mut self) -> Word {
        let low = self.pop_byte() as Word;
        let high = self.pop_byte() as Word;
        (high << 8) | low
    }

    pub fn read_next_byte(&mut self) -> Byte {
        let value = self.read_byte(self.registers.pc);
        self.registers.pc += 1;

        value
    }

    pub fn read_next_word(&mut self) -> Word {
        let value = self.read_word(self.registers.pc);
        self.registers.pc += 2;

        value
    }
}

impl<M: Memory> CPU<M> {
    pub fn branch(&mut self, condition: bool) {
        let offset = self.read_next_byte() as Offset;
        let current = self.registers.pc & 0xFF00;

        if condition {
            self.cycles += 1; // +1 if branch succeeds.
            self.registers.pc = (self.registers.pc as DWord + offset as DWord) as Word;
            if self.registers.pc & 0xFF00 != current { 
                self.cycles += 1; // +1 if to a new page
            }
        }
    }

    pub fn compare<RM: ReadMode<M>, ID: InstructionDefinition<M, RM>>(&mut self, x: u8)
    where
        RM: Addressing<M>,
    {
        let rm = RM::create_addressing(self, ID::page_boundary());
        let y = rm.read(self) as DWord;
        let result = (x as DWord).wrapping_sub(y);
        self.registers
            .set_flag(Registers::CARRY_FLAG, (result & 0x0100) == 0);
        self.registers.set_flag(Registers::ZERO_FLAG, result == 0);
        self.registers
            .set_flag(Registers::NEGATIVE_FLAG, (result & 0x80) != 0);
    }

    pub fn shift_left<RWM: ReadMode<M> + WriteMode<M>, ID: InstructionDefinition<M, RWM>>(&mut self, condition: bool)
    where
        RWM: Addressing<M>,
    {
        let rwm = RWM::create_addressing(self, ID::page_boundary());
        let value = rwm.read(self);
        let mut result = value << 0x01;
        if condition {
            result |= 0x01;
        }

        self.registers
            .set_flag(Registers::CARRY_FLAG, (value & 0x80) != 0);
        self.registers.set_zn(result);
        rwm.write(self, result as Byte);
    }

    pub fn shift_right<RWM: ReadMode<M> + WriteMode<M>, ID: InstructionDefinition<M, RWM>>(&mut self, condition: bool)
    where
        RWM: Addressing<M>,
    {
        let rwm = RWM::create_addressing(self, ID::page_boundary());
        let value = rwm.read(self);
        let mut result = value >> 0x01;
        if condition {
            result |= 0x80;
        }

        self.registers
            .set_flag(Registers::CARRY_FLAG, (value & 0x01) != 0);
        self.registers.set_zn(result);
        rwm.write(self, result);
    }
}

impl<M: Memory> CPU<M> {
    pub fn execute<II: ImpliedInstruction<M> + InstructionDefinition<M, S>, S>(&mut self) {
        self.cycles += II::cycles();
        II::execute(self);
    }

    pub fn execute_read<RI: ReadInstruction<M> + InstructionDefinition<M, RM>, RM: ReadMode<M>>(&mut self)
    where
        RM: Addressing<M>,
    {
        self.cycles += RI::cycles();
        RI::execute::<RM, RI>(self);
    }

    pub fn execute_write<WI: WriteInstruction<M> + InstructionDefinition<M, WM>, WM: WriteMode<M>>(&mut self)
    where
        WM: Addressing<M>,
    {
        self.cycles += WI::cycles();
        WI::execute::<WM, WI>(self);
    }

    pub fn execute_read_write<RWI: ReadWriteInstruction<M> + InstructionDefinition<M, RWM>, RWM: ReadMode<M> + WriteMode<M>>(
        &mut self,
    ) where
        RWM: Addressing<M>,
    {
        self.cycles += RWI::cycles();
        RWI::execute::<RWM, RWI>(self);
    }
}
