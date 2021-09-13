use std::{
    io::Write,
    ops::{Add, AddAssign},
};

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

    pub fn execute_cycles(&mut self, cycles_needed: u32) -> u32 {
        while self.cycles < cycles_needed {
            self.run_instruction();
        }

        self.cycles
    }

    pub fn execute(&mut self) {
        loop {
            self.run_instruction();
        }
    }

    fn run_instruction(&mut self) {
        if let Some((exec_func, addr_func)) = INSTRUCTION_CODE[self.fetch_byte() as usize] {
            exec_func(addr_func(self), self);
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let data = self.read_byte(self.registers.pc);

        self.registers.pc = u16::wrapping_add(self.registers.pc, 0x0001);
        data
    }

    pub fn fetch_word(&mut self) -> u16 {
        // 6502 is little endian
        let mut data = self.read_byte(self.registers.pc) as u16;
        self.registers.pc = u16::wrapping_add(self.registers.pc, 0x0001);

        data |= (self.read_byte(self.registers.pc) as u16) << 8;
        self.registers.pc = u16::wrapping_add(self.registers.pc, 0x0001);

        data
    }

    pub fn read_byte(&mut self, u16: u16) -> u8 {
        self.cycles += 1;
        self.memory[u16]
    }

    pub fn read_word(&mut self, u16: u16) -> u16 {
        let low = self.read_byte(u16);
        let high = self.read_byte(u16 + 0x0001);

        low as u16 | (high as u16) << 8
    }

    pub fn write_byte(&mut self, value: u8, addr: u16) {
        self.memory[addr] = value;
        self.cycles += 1;
    }

    pub fn write_word(&mut self, value: u16, u16: u16) {
        self.write_byte((value & 0xFF) as u8, u16);
        self.write_byte((value >> 8) as u8, u16 + 0x0001);
    }

    pub fn branch_if(&mut self, switch: bool, offset: i8) {
        if switch {
            let pc_old = self.registers.pc;
            self.registers.pc = (self.registers.pc as i32 + offset as i32) as u16;

            self.cycles += 1;

            let page_changed = (self.registers.pc >> 8) != (pc_old >> 8);
            if page_changed {
                self.cycles += 1;
            }
        }
    }

    pub fn stack_pop_byte(&mut self) -> u8 {
        self.registers.sp += 1;

        self.read_byte(self.sp_to_address())
    }

    pub fn stack_pop_word(&mut self) -> u16 {
        self.registers.sp += 1;
        let value = self.read_word(self.sp_to_address());
        self.registers.sp += 1;
        value
    }

    pub fn stack_push_byte(&mut self, value: u8) {
        self.write_byte(value, self.sp_to_address());
        self.registers.sp -= 1;
    }

    pub fn stack_push_word(&mut self, value: u16) {
        self.write_word(value, self.sp_to_address());
        self.registers.sp -= 2;
    }

    // returns the address at which the stack pointer points as a full 16-bits address
    pub fn sp_to_address(&self) -> u16 {
        STACK_BOTTOM | self.registers.sp as u16 // 0x01 | sp -> 0x01[sp]
    }

    pub fn push_word_to_stack(&mut self, value: u16) {
        self.write_byte((value >> 8) as u8, self.sp_to_address());
        self.registers.sp -= 1;
        self.write_byte((value & 0xFF) as u8, self.sp_to_address());
        self.registers.sp -= 1;
    }

    pub fn push_pc_plus_one_to_stack(&mut self) {
        self.push_word_to_stack(self.registers.pc + 0x0001);
    }

    pub fn push_pc_minus_one_to_stack(&mut self) {
        self.push_word_to_stack(self.registers.pc - 0x0001);
    }

    pub fn push_status_to_stack(&mut self) {
        let status = self.registers.to_byte() | FLAG_BREAK | FLAG_UNUSED;
        self.push_byte_onto_stack(status);
    }

    pub fn stack_pop_status(&mut self) {
        let pop = self.stack_pop_byte();
        self.registers.from_byte(pop);
        self.registers.b = false;
        self.registers.unused = false;
    }

    pub fn push_byte_onto_stack(&mut self, value: u8) {
        let sp_word = self.sp_to_address();
        self.memory[sp_word] = value;
        self.cycles += 1;
        self.registers.sp -= 1;
        self.cycles += 1;
    }
}
