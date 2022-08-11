use std::{
    io::Write,
    ops::{Add, AddAssign},
};

use crate::{
    operation::{self, AddressMode},
    CPUFlag, Cartridge, Memory, BUS, OPERATIONS,
};

pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub status: u8, // bit 0: C, bit 1: Z, bit 2: I, bit 3: D, bit 4: B, bit 5: U, bit 6: V, bit 7: N

    pub sp: u8,
    pub pc: u16,

    pub cycles: u32,

    pub bus: BUS,
}

impl CPU {
    const STACK: u16 = 0x0100;
    const STACK_RESET: u8 = 0xfd;
    const RESET_VECTOR: u16 = 0xFFFC;

    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: Self::STACK_RESET,
            status: CPUFlag::Interrupt | CPUFlag::Unused,
            cycles: 0,
            bus: BUS::new(cartridge),
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.sp = Self::STACK_RESET;
        self.pc = self.bus.read_word(Self::RESET_VECTOR);

        self.status = CPUFlag::Interrupt | CPUFlag::Unused;
    }

    pub fn operand(&self, mode: &AddressMode) -> u16 {
        match mode {
            AddressMode::Immediate => self.pc,
            _ => self.get_absolute_address(mode, self.pc),
        }
    }

    pub fn get_absolute_address(&self, mode: &AddressMode, addr: u16) -> u16 {
        match mode {
            AddressMode::ZeroPage => self.bus.read_byte(addr) as u16,
            AddressMode::ZeroPageX => {
                let position = self.bus.read_byte(addr);
                
                position.wrapping_add(self.x) as u16
            }
            AddressMode::ZeroPageY => {
                let position = self.bus.read_byte(addr);
                
                position.wrapping_add(self.y) as u16
            }
            AddressMode::Absolute => self.bus.read_word(addr),
            AddressMode::AbsoluteX => {
                let base = self.bus.read_word(addr);
                
                base.wrapping_add(self.x as u16)
            }
            AddressMode::AbsoluteY => {
                let base = self.bus.read_word(addr);
                
                base.wrapping_add(self.y as u16)
            }
            AddressMode::IndirectX => {
                let base = self.bus.read_byte(addr);
                let pointer: u8 = (base as u8).wrapping_add(self.x);

                let low = self.bus.read_byte(pointer as u16);
                let high = self.bus.read_byte(pointer.wrapping_add(1) as u16);

                (high as u16) << 8 | (low as u16)
            }
            AddressMode::IndirectY => {
                let base = self.bus.read_byte(addr);

                let low = self.bus.read_byte(base as u16);
                let high = self.bus.read_byte(base.wrapping_add(1) as u16);

                let deref_base = (high as u16) << 8 | (low as u16);
                
                deref_base.wrapping_add(self.y as u16)
            }
            _ => panic!("Mode {:?} is not supported", mode),
        }
    }

    pub fn run(&mut self) {
        self.run_callback(|_| {});
    }

    pub fn run_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);
            let code = self.bus.read_byte(self.pc);
            self.pc += 1;
            let pc_state = self.pc;

            // 0xAD
            let op = &OPERATIONS[code as usize];

            (op.function)(self, &op.mode);
            self.cycles += op.cycles as u32;

            if pc_state == self.pc {
                self.pc += (op.length - 1) as u16;
            }
        }
    }

    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            Self::set_bit(&mut self.status, CPUFlag::Zero);
        } else {
            Self::unset_bit(&mut self.status, CPUFlag::Zero);
        }

        self.update_negative_flag(result);
    }

    pub fn update_negative_flag(&mut self, result: u8) {
        if result >> 7 == 1 {
            Self::set_bit(&mut self.status, CPUFlag::Negative);
        } else {
            Self::unset_bit(&mut self.status, CPUFlag::Negative);
        }
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
    }

    pub fn add_a(&mut self, data: u8) {
        let sum = self.a as u16 + data as u16 + (self.status & CPUFlag::Carry as u8) as u16;

        let carry = sum > 0xFF;
        if carry {
            Self::set_bit(&mut self.status, CPUFlag::Carry);
        } else {
            Self::unset_bit(&mut self.status, CPUFlag::Carry);
        }

        let result = sum as u8;
        if (data ^ result) & (result ^ self.a) & 0x80 != 0 {
            self.status |= CPUFlag::Overflow as u8;
        } else {
            self.status &= !(CPUFlag::Overflow as u8);
        }

        self.set_a(result);
    }

    pub fn branch(&mut self, condition: bool) {
        if condition {
            let offset = self.bus.read_byte(self.pc) as i8;
            let address = self.pc.wrapping_add(1).wrapping_add(offset as u16);

            self.pc = address;
        }
    }

    pub fn set_bit(target: &mut u8, flag: CPUFlag) {
        *target |= flag as u8;
    }

    pub fn unset_bit(target: &mut u8, flag: CPUFlag) {
        *target &= !(flag as u8);
    }

    pub fn is_bit_set(target: &u8, flag: CPUFlag) -> bool {
        *target & flag as u8 != 0
    }

    pub fn compare(&mut self, mode: &AddressMode, what: u8) {
        let address = self.operand(mode);
        let data = self.bus.read_byte(address);
        if data <= what {
            Self::set_bit(&mut self.status, CPUFlag::Carry);
        } else {
            Self::unset_bit(&mut self.status, CPUFlag::Carry);
        }

        self.update_zero_and_negative_flags(self.a.wrapping_sub(data));
    }

    fn stack_push_byte(&mut self, data: u8) {
        self.bus.write_byte((Self::STACK as u16) + self.sp as u16, data);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn stack_push_word(&mut self, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xFF) as u8;
        self.stack_push_byte(high);
        self.stack_push_byte(low);
    }

    fn stack_pop_byte(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.bus.read_byte((Self::STACK as u16) + self.sp as u16)
    }

    fn stack_pop_word(&mut self) -> u16 {
        let low = self.stack_pop_byte() as u16;
        let high = self.stack_pop_byte() as u16;
        high << 8 | low
    }
}

impl CPU {
    pub fn adc(&mut self, mode: &AddressMode) {
        let operand = self.operand(mode);
        let value = self.bus.read_byte(operand);
        self.add_a(value);
    }

    pub fn and(&mut self, mode: &AddressMode) {
        let operand = self.operand(mode);
        let data = self.bus.read_byte(operand);
        self.set_a(data & self.a);
    }

    pub fn asl(&mut self, mode: &AddressMode) {
        match mode {
            AddressMode::Implied => {
                let mut data = self.a;
                
                if data >> 7 == 1 {
                    Self::set_bit(&mut self.status, CPUFlag::Carry);
                } else {
                    Self::unset_bit(&mut self.status, CPUFlag::Carry);
                }

                data <<= 1;
                self.set_a(data);
            }
            x => {
                let address = self.operand(x);
                let mut data = self.bus.read_byte(address);

                if data >> 1 == 1 {
                    Self::set_bit(&mut self.status, CPUFlag::Carry);
                } else {
                    Self::unset_bit(&mut self.status, CPUFlag::Carry);
                }


                data <<= 1;
                self.bus.write_byte(address, data);
                self.update_zero_and_negative_flags(data);
            }
        }
    }

    pub fn bcc(&mut self, _: &AddressMode) {
        let cond = !Self::is_bit_set(&self.status, CPUFlag::Carry);
        self.branch(cond); // carry not set
    }

    pub fn bcs(&mut self, _: &AddressMode) {
        let cond = Self::is_bit_set(&self.status, CPUFlag::Carry);
        self.branch(cond); // carry set
    }

    pub fn beq(&mut self, _: &AddressMode) {
        let cond = Self::is_bit_set(&self.status, CPUFlag::Zero);
        self.branch(cond); // zero set
    }

    pub fn bit(&mut self, mode: &AddressMode) {
        let address = self.operand(mode);
        let data = self.bus.read_byte(address);
        let and = self.a & data;
        if and == 0  {
            Self::set_bit(&mut self.status, CPUFlag::Zero);
        } else {
            Self::unset_bit(&mut self.status, CPUFlag::Zero);
        }


        if (data & CPUFlag::Negative as u8) > 0 { Self::set_bit(&mut self.status, CPUFlag::Negative) }
        if (data & CPUFlag::Overflow as u8) > 0 { Self::set_bit(&mut self.status, CPUFlag::Overflow) }
    }

    pub fn bmi(&mut self, _: &AddressMode) {
        let cond = Self::is_bit_set(&self.status, CPUFlag::Negative);
        self.branch(cond);
    }

    pub fn bne(&mut self, _: &AddressMode) {
        let cond = !Self::is_bit_set(&self.status, CPUFlag::Zero);
        self.branch(cond);
    }

    pub fn bpl(&mut self, _: &AddressMode) {
        let cond = !Self::is_bit_set(&self.status, CPUFlag::Negative);
        self.branch(cond);
    }

    pub fn brk(&mut self, _: &AddressMode) {
        // Should break.
    }

    pub fn bvc(&mut self, _: &AddressMode) {
        let cond = !Self::is_bit_set(&self.status, CPUFlag::Overflow);
        self.branch(cond); 
    }

    pub fn bvs(&mut self, _: &AddressMode) {
        let cond = Self::is_bit_set(&self.status, CPUFlag::Overflow);
        self.branch(cond);
    }

    pub fn clc(&mut self, _: &AddressMode) {
        Self::unset_bit(&mut self.status, CPUFlag::Carry);
    }

    pub fn cld(&mut self, _: &AddressMode) {
        Self::unset_bit(&mut self.status, CPUFlag::Decimal);
    }

    pub fn cli(&mut self, _: &AddressMode) {
        Self::unset_bit(&mut self.status, CPUFlag::Interrupt);
    }

    pub fn clv(&mut self, _: &AddressMode) {
        Self::unset_bit(&mut self.status, CPUFlag::Overflow);
    }

    pub fn cmp(&mut self, mode: &AddressMode) {
        self.compare(mode, self.a);
    }

    pub fn cpx(&mut self, mode: &AddressMode) {
        self.compare(mode, self.x);
    }

    pub fn cpy(&mut self, mode: &AddressMode) {
        self.compare(mode, self.y);
    }

    pub fn dec(&mut self, mode: &AddressMode) {
        let address = self.operand(mode);
        let mut data = self.bus.read_byte(address);
        data = data.wrapping_sub(1);
        self.bus.write_byte(address, data);
        self.update_zero_and_negative_flags(data);
    }

    pub fn dex(&mut self, _: &AddressMode) {
        self.x = self.x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.x);
    }

    pub fn dey(&mut self, _: &AddressMode) {
        self.y = self.y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.y);
    }

    pub fn eor(&mut self, mode: &AddressMode) {
        let address = self.operand(mode);
        let data = self.bus.read_byte(address);
        self.set_a(data ^ self.a);
    }

    pub fn inc(&mut self, mode: &AddressMode) {
        let address = self.operand(mode);
        let mut data = self.bus.read_byte(address);
        data = data.wrapping_add(1);
        self.bus.write_byte(address, data);
        self.update_zero_and_negative_flags(data);
    }

    pub fn inx(&mut self, _: &AddressMode) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.x);
    }

    pub fn iny(&mut self, _: &AddressMode) {
        self.y = self.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.y);
    }

    pub fn jmp(&mut self, mode: &AddressMode) {
        let address = self.bus.read_word(self.pc);

        self.pc = match mode {
            AddressMode::Absolute => {
                address
            }
            AddressMode::Indirect => {
                if address & 0x00FF == 0x00FF {
                    let low = self.bus.read_byte(address);
                    let high = self.bus.read_byte(address & 0xFF00);
                    (high as u16) << 8 | (low as u16)
                } else {
                    self.bus.read_word(address)
                }
            }
            _ => { panic!("Invalid address mode for JMP.") }
        }
    }

    pub fn jsr(&mut self, _: &AddressMode) {
        self.stack_push_word(self.pc + 1);
        let address = self.bus.read_word(self.pc);
        self.pc = address;
    }

    pub fn lda(&mut self, mode: &AddressMode) {
        let operand = self.operand(mode);
        let value = self.bus.read_byte(operand);
        self.set_a(value);
    }

    pub fn ldx(&mut self, mode: &AddressMode) {
        let operand = self.operand(mode);
        let value = self.bus.read_byte(operand);
        self.x = value;
        self.update_zero_and_negative_flags(self.x);
    }

    pub fn ldy(&mut self, mode: &AddressMode) {
        let operand = self.operand(mode);
        let value = self.bus.read_byte(operand);
        self.y = value;
        self.update_zero_and_negative_flags(self.y);
    }

    pub fn lsr(&mut self, mode: &AddressMode) {
        match mode {
            AddressMode::Implied => {
                let mut data = self.a;

                if data & 1 == 1 {
                    Self::set_bit(&mut self.status, CPUFlag::Carry);
                } else {
                    Self::unset_bit(&mut self.status, CPUFlag::Carry);
                }

                data >>= 1;
                self.set_a(data);
            }
            x => {
                let address = self.operand(x);
                let mut data = self.bus.read_byte(address);

                if data & 1 == 1 {
                    Self::set_bit(&mut self.status, CPUFlag::Carry);
                } else {
                    Self::unset_bit(&mut self.status, CPUFlag::Carry);
                }

                data >>= 1;
                self.bus.write_byte(address, data);
                self.update_zero_and_negative_flags(data);
            }
        }
    }

    pub fn nop(&mut self, _: &AddressMode) {}

    pub fn ora(&mut self, mode: &AddressMode) {
        let operand = self.operand(mode);
        let value = self.bus.read_byte(operand);
        self.set_a(self.a | value);
    }

    pub fn pha(&mut self, _: &AddressMode) {
        self.stack_push_byte(self.a);
    }

    pub fn php(&mut self, _: &AddressMode) {
        let mut status = self.status;
        status |= CPUFlag::Break;
        status |= CPUFlag::Unused;
        self.stack_push_byte(status);
    }

    pub fn pla(&mut self, _: &AddressMode) {
        let data = self.stack_pop_byte();
        self.set_a(data);
    }

    pub fn plp(&mut self, _: &AddressMode) {
        self.status = self.stack_pop_byte();
        Self::set_bit(&mut self.status, CPUFlag::Break);
        Self::set_bit(&mut self.status, CPUFlag::Unused);
    }

    pub fn rol(&mut self, mode: &AddressMode) {
        match mode {
            AddressMode::Implied => {
                let mut data = self.a;
                let carry = Self::is_bit_set(&self.status, CPUFlag::Carry);

                if data >> 7 == 1 {
                    Self::set_bit(&mut self.status, CPUFlag::Carry);
                } else {
                    Self::unset_bit(&mut self.status, CPUFlag::Carry);
                }

                data <<= 1;

                if carry {
                    data |= 1;
                }

                self.set_a(data);
            }
            x => {
                let address = self.operand(x);
                let mut data = self.bus.read_byte(address);
                let carry = Self::is_bit_set(&self.status, CPUFlag::Carry);

                if data >> 7 == 1 {
                    Self::set_bit(&mut self.status, CPUFlag::Carry);
                } else {
                    Self::unset_bit(&mut self.status, CPUFlag::Carry);
                }

                data <<= 1;

                if carry {
                    data |= 1;
                }

                self.bus.write_byte(address, data);
                self.update_negative_flag(data);
            }
        }
    }

    pub fn ror(&mut self, mode: &AddressMode) {
        match mode {
            AddressMode::Implied => {
                let mut data = self.a;
                let carry = Self::is_bit_set(&self.status, CPUFlag::Carry);

                if data & 1 == 1 {
                    Self::set_bit(&mut self.status, CPUFlag::Carry);
                } else {
                    Self::unset_bit(&mut self.status, CPUFlag::Carry);
                }

                data >>= 1;

                if carry {
                    data |= 0x80;
                }

                self.set_a(data);
            }
            x => {
                let address = self.operand(x);
                let mut data = self.bus.read_byte(address);
                let carry = Self::is_bit_set(&self.status, CPUFlag::Carry);

                if data & 1 == 1 {
                    Self::set_bit(&mut self.status, CPUFlag::Carry);
                } else {
                    Self::unset_bit(&mut self.status, CPUFlag::Carry);
                }

                data >>= 1;

                if carry {
                    data |= 0x80;
                }

                self.bus.write_byte(address, data);
                self.update_negative_flag(data);
            }
        }
    }

    pub fn rti(&mut self, _: &AddressMode) {
        self.status = self.stack_pop_byte();
        
        Self::set_bit(&mut self.status, CPUFlag::Break);
        Self::set_bit(&mut self.status, CPUFlag::Unused);

        self.pc = self.stack_pop_word();
    }

    pub fn rts(&mut self, _: &AddressMode) {
        self.pc = self.stack_pop_word() + 1;
    }

    pub fn sbc(&mut self, mode: &AddressMode) {
        let addr = self.operand(mode);
        let data = self.bus.read_byte(addr);
        self.add_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8)
    }

    pub fn sec(&mut self, _: &AddressMode) {
        Self::set_bit(&mut self.status, CPUFlag::Carry);
    }

    pub fn sed(&mut self, _: &AddressMode) {
        Self::set_bit(&mut self.status, CPUFlag::Decimal);
    }

    pub fn sei(&mut self, _: &AddressMode) {
        Self::set_bit(&mut self.status, CPUFlag::Interrupt);
    }

    pub fn sta(&mut self, mode: &AddressMode) {
        let addr = self.operand(mode);
        self.bus.write_byte(addr, self.a);
    }

    pub fn stx(&mut self, mode: &AddressMode) {
        let addr = self.operand(mode);
        self.bus.write_byte(addr, self.x);
    }

    pub fn sty(&mut self, mode: &AddressMode) {
        let addr = self.operand(mode);
        self.bus.write_byte(addr, self.y);
    }

    pub fn tax(&mut self, _: &AddressMode) {
        self.x = self.a;
        self.update_zero_and_negative_flags(self.x);
    }

    pub fn tay(&mut self, _: &AddressMode) {
        self.y = self.a;
        self.update_zero_and_negative_flags(self.y);
    }

    pub fn tsx(&mut self, _: &AddressMode) {
        self.x = self.sp;
        self.update_zero_and_negative_flags(self.x);
    }

    pub fn txa(&mut self, _: &AddressMode) {
        self.a = self.x;
        self.update_zero_and_negative_flags(self.a);
    }

    pub fn txs(&mut self, _: &AddressMode) {
        self.sp = self.x;
    }

    pub fn tya(&mut self, _: &AddressMode) {
        self.a = self.y;
        self.update_zero_and_negative_flags(self.a);
    }
}
