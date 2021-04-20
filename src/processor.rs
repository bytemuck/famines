use std::ops::{Add, AddAssign};

use crate::*;

#[derive(Copy, Clone)]
pub struct Processor {
    pub registers: Registers,
    pub memory: Memory,
    pub cycles: u32,
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn execute(&mut self, cycles_needed: u32) -> u32 {
        while self.cycles < cycles_needed {
            self.run_instruction()
        }

        self.cycles
    }

    fn run_instruction(&mut self) {
        let code = self.fetch_byte();
        if let Some((exec_func, addr_func)) = INSTRUCTION_CODE[code as usize] {
            exec_func(addr_func(self), self);
        }
    }

    pub fn fetch_byte(&mut self) -> Byte {
        let data = self.read_byte(self.registers.pc);

        self.registers.pc += RelativeAddress(0x01);
        data
    }

    pub fn fetch_word(&mut self) -> Word {
        // 6502 is little endian
        let mut data = self.read_byte(self.registers.pc) as Word;
        self.registers.pc += RelativeAddress(0x01);

        data |= (self.read_byte(self.registers.pc) as Word) << 8;
        self.registers.pc += RelativeAddress(0x01);

        data
    }

    pub fn read_byte(&mut self, Address(address): Address) -> Byte {
        self.cycles += 1;
        self.memory[address]
    }

    pub fn read_word(&mut self, address: Address) -> Word {
        let low = self.read_byte(address);
        let high = self.read_byte(address + RelativeAddress(0x01));

        low as Word | (high as Word) << 8
    }

    pub fn write_byte(&mut self, value: u8, address: Address) {
        self.memory[address.to_word()] = value;
        self.cycles += 1;
    }

    pub fn write_word(&mut self, value: u16, address: Address) {
        self.write_byte((value & 0xFF) as u8, address);
        self.write_byte((value >> 8) as u8, address + RelativeAddress(1));
    }

    pub fn branch_if(&mut self, switch: bool, offset: RelativeAddress) {
        if switch {
            let pc_old = self.registers.pc;
            self.registers.pc += offset;
            self.cycles += 1;

            let page_changed = (self.registers.pc.to_word() >> 8) != (pc_old.to_word() >> 8);
            if page_changed {
                self.cycles += 1;
            }
        }
    }

    pub fn stack_pop(&mut self) -> Byte {
        if self.registers.sp == 0xFF {
            self.registers.sp = 0x00;
        } else {
            self.registers.sp += 1;
        }
        self.read_byte(self.sp_to_address())
    }

    pub fn stack_push(&mut self, value: Byte) {
        self.write_byte(value, self.sp_to_address());
        if self.registers.sp == 0x00 {
            self.registers.sp = 0xFF;
        } else {
            self.registers.sp -= 1;
        }
    }

    pub fn sp_to_address(&self) -> Address {
        Address(STACK_BOTTOM | self.registers.sp as Word)
    }

    pub fn push_word_to_stack(&mut self, value: Word) {
        self.write_byte((value >> 8) as u8, self.sp_to_address());
        self.registers.sp -= 1;
        self.write_byte((value & 0xFF) as u8, self.sp_to_address());
        self.registers.sp -= 1;
    }

    pub fn push_pc_plus_one_to_stack(&mut self) {
        self.push_word_to_stack(self.registers.pc.to_word().wrapping_add(1));
    }

    pub fn push_pc_minus_one_to_stack(&mut self) {
        self.push_word_to_stack(self.registers.pc.to_word().wrapping_sub(1));
    }

    pub fn push_status_to_stack(&mut self) {
        let status_stack = self.registers.status | FLAG_BREAK;
        self.push_byte_onto_stack(status_stack);
    }

    pub fn push_byte_onto_stack(&mut self, value: Byte) {
        let sp_word = self.sp_to_address();
        self.memory[sp_word.to_word()] = value;
        self.cycles += 1;
        self.registers.sp -= 1;
        self.cycles += 1;
    }
}
