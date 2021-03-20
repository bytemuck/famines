use crate::*;

#[derive(Copy, Clone)]
pub struct Processor {
    pub registers: Registers,
    pub memory: Memory,
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
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn execute(&mut self, mut cycles: i32) -> i32 {
        let cycles_requested = cycles;

        while cycles > 0 {
            match self.fetch_instruction(&mut cycles) {
                (Instruction::LDA, InstructionInput::Immediate(value)) => {
                    self.registers.a = value;
                    self.registers.set_zero();
                    self.registers.set_negative();
                }
                (Instruction::LDA, InstructionInput::Address(address)) => {
                    self.registers.a = self.read_byte(&mut cycles, address);
                    self.registers.set_zero();
                    self.registers.set_negative();
                }
                (Instruction::JSR, InstructionInput::Address(address)) => {
                    self.memory.write_word(
                        self.registers.pc - 1,
                        self.registers.sp as Word,
                        &mut cycles,
                    );
                    self.registers.sp += 2;
                    self.registers.pc = address;
                    cycles -= 1;
                }
                _ => {}
            }
        }

        cycles_requested - cycles
    }

    fn fetch_instruction(&mut self, cycles: &mut i32) -> DecodedInstruction {
        let code = self.fetch_byte(cycles);
        let (instruction, mode, register) = INSTRUCTION_CODE[code as usize];

        let input = match mode {
            AddressingMode::Unknown => InstructionInput::Unknown,
            AddressingMode::Accumulator => InstructionInput::Unknown,
            AddressingMode::Immediate => InstructionInput::Immediate(self.fetch_byte(cycles)),
            AddressingMode::Implied => InstructionInput::Unknown,
            AddressingMode::Relative => InstructionInput::Unknown,
            AddressingMode::Absolute => match register {
                ImpliedRegister::None => InstructionInput::Address(self.fetch_word(cycles)),
                ImpliedRegister::X => {
                    let address = self.fetch_word(cycles);
                    let address_x = address + self.registers.x as Word;
                    if address_x - address >= 0xFF {
                        *cycles -= 1;
                    }

                    InstructionInput::Address(address_x)
                }
                ImpliedRegister::Y => {
                    let address = self.fetch_word(cycles);
                    let address_y = address + self.registers.y as Word;
                    if address_y - address >= 0xFF {
                        *cycles -= 1;
                    }

                    InstructionInput::Address(address_y)
                }
            },
            AddressingMode::ZeroPage => match register {
                ImpliedRegister::None => InstructionInput::Address(self.fetch_byte(cycles) as Word),
                ImpliedRegister::X => {
                    let address = self.fetch_byte(cycles);
                    let address = address.wrapping_add(self.registers.x);
                    InstructionInput::Address(address as Word)
                }
                ImpliedRegister::Y => {
                    let address = self.fetch_byte(cycles);
                    let address = address.wrapping_add(self.registers.y);
                    InstructionInput::Address(address as Word)
                }
            },
            AddressingMode::Indirect => match register {
                ImpliedRegister::None => InstructionInput::Unknown,
                ImpliedRegister::X => {
                    let address = self.fetch_byte(cycles) + self.registers.x;
                    let address = self.read_word(cycles, address as Word);
                    *cycles -= 1;

                    InstructionInput::Address(address)
                }
                ImpliedRegister::Y => {
                    let address = self.fetch_byte(cycles);
                    let address = self.read_word(cycles, address as Word);
                    let address_y = address + self.registers.y as Word;

                    if address_y - address >= 0xFF {
                        *cycles -= 1;
                    }

                    InstructionInput::Address(address_y)
                }
            },
        };

        (instruction, input)
    }

    fn fetch_byte(&mut self, cycles: &mut i32) -> Byte {
        let data = self.memory[self.registers.pc];
        self.registers.pc += 1;
        *cycles -= 1;
        data
    }

    fn fetch_word(&mut self, cycles: &mut i32) -> Word {
        // 6502 is little endian
        let mut data = self.memory[self.registers.pc] as Word;
        self.registers.pc += 1;

        data |= (self.memory[self.registers.pc] as Word) << 8;
        self.registers.pc += 1;

        *cycles -= 2;
        data
    }

    fn read_byte_zp(&mut self, cycles: &mut i32, address: Byte) -> Byte {
        *cycles -= 1;
        self.memory[address as Word]
    }

    fn read_byte(&mut self, cycles: &mut i32, address: Word) -> Byte {
        *cycles -= 1;
        self.memory[address]
    }

    fn read_word(&mut self, cycles: &mut i32, address: Word) -> Word {
        let low = self.read_byte(cycles, address);
        let high = self.read_byte(cycles, address + 1);

        low as Word | (high as Word) << 8
    }
}
