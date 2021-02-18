use crate::*;

#[derive(Copy, Clone)]
pub struct Processor {
    pub pc: u16, // Program Counter
    pub sp: u16, // Stack Pointer

    pub a: u8, // Accumulator
    pub x: u8, // Register X
    pub y: u8, // Register Y

    pub c: u8, // Carry Flag
    pub z: u8, // Zero Flag
    pub i: u8, // Interrupt Disable
    pub d: u8, // Decimal Mode
    pub b: u8, // Break Command
    pub v: u8, // Overflow Flag
    pub n: u8, // Negative Flag
}

impl Processor {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,

            a: 0,
            x: 0,
            y: 0,

            c: 0,
            z: 0,
            i: 0,
            d: 0,
            b: 0,
            v: 0,
            n: 0,
        }
    }

    pub fn reset(&mut self, memory: &mut Memory) {
        self.pc = 0xFFFC;
        self.sp = 0x0100;

        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.c = 0;
        self.z = 0;
        self.i = 0;
        self.d = 0;
        self.b = 0;
        self.v = 0;
        self.n = 0;

        memory.initialize();
    }

    pub fn execute(&mut self, mut cycles: u32, memory: &mut Memory) -> u32 {
        let cycles_requested = cycles;

        while cycles > 0 {
            match self.fetch_u8(&mut cycles, memory) {
                LDA_IMMEDIATE => {
                    self.a = self.fetch_u8(&mut cycles, memory);
                    self.z = (self.a == 0) as u8;
                    self.n = check_mask(self.a, 0b1000_0000) as u8;
                }
                LDA_ZERO_PAGE => {
                    let address = self.fetch_u8(&mut cycles, memory);
                    self.a = self.read(&mut cycles, address, memory);
                    self.z = (self.a == 0) as u8;
                    self.n = check_mask(self.a, 0b1000_0000) as u8;
                }
                LDA_ZERO_PAGE_X => {
                    let mut address = self.fetch_u8(&mut cycles, memory);
                    address = address.wrapping_add(self.x); // since it may overflow!
                    cycles -= 1;
                    self.a = self.read(&mut cycles, address, memory);
                    self.z = (self.a == 0) as u8;
                    self.n = check_mask(self.a, 0b1000_0000) as u8;
                }
                JSR_ABSOLUTE => {
                    let sub_routine_address = self.fetch_u16(&mut cycles, memory);
                    memory.write_u16((self.pc - 1) as u32, self.sp as u32, &mut cycles);
                    self.pc = sub_routine_address;
                    cycles -= 1;
                }
                _ => { println!("Unknown instruction.") }
            }
        }

        cycles_requested - cycles
    }

    fn fetch_u8(&mut self, cycles: &mut u32, memory: &mut Memory) -> u8 {
        let data = memory[self.pc];
        self.pc += 1;
        *cycles -= 1;
        data
    }

    fn fetch_u16(&mut self, cycles: &mut u32, memory: &mut Memory) -> u16 {
        // 6502 is little endian
        let mut data = memory[self.pc] as u16;
        self.pc += 1;

        data |= (memory[self.pc] as u16).rotate_left(8);
        self.pc += 1;

        *cycles -= 2;
        data
    }

    fn read(&mut self, cycles: &mut u32, address: u8, memory: &mut Memory) -> u8 {
        let data = memory[address as u16];
        *cycles -= 1;
        data
    }
}