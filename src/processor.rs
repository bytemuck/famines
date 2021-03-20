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
            match self.fetch_instruction() {
                (Instruction::INC, InstructionInput::Address(address), instruction_cycles) => {
                    let mut result = self.read_byte(address);
                    result += 0x01;

                    self.memory.write_byte(result, address);
                    self.registers.set_zero(result);
                    self.registers.set_negative(result);

                    cycles -= instruction_cycles as i32;
                }
                (Instruction::LDA, InstructionInput::Immediate(value), instruction_cycles) => {
                    self.registers.a = value;
                    self.registers.set_zero_a();
                    self.registers.set_negative_a();

                    cycles -= instruction_cycles as i32;
                }
                (Instruction::LDA, InstructionInput::Address(address), instruction_cycles) => {
                    self.registers.a = self.read_byte(address);
                    self.registers.set_zero_a();
                    self.registers.set_negative_a();

                    cycles -= instruction_cycles as i32;
                }
                (Instruction::LDX, InstructionInput::Immediate(value), instruction_cycles) => {
                    self.registers.x = value;
                    self.registers.set_zero_x();
                    self.registers.set_negative_x();

                    cycles -= instruction_cycles as i32;
                }
                (Instruction::LDX, InstructionInput::Address(address), instruction_cycles) => {
                    self.registers.x = self.read_byte(address);
                    self.registers.set_zero_x();
                    self.registers.set_negative_x();

                    cycles -= instruction_cycles as i32;
                }
                (Instruction::LDY, InstructionInput::Immediate(value), instruction_cycles) => {
                    self.registers.y = value;
                    self.registers.set_zero_y();
                    self.registers.set_negative_y();

                    cycles -= instruction_cycles as i32;
                }
                (Instruction::LDY, InstructionInput::Address(address), instruction_cycles) => {
                    self.registers.y = self.read_byte(address);
                    self.registers.set_zero_y();
                    self.registers.set_negative_y();

                    cycles -= instruction_cycles as i32;
                }
                (Instruction::JSR, InstructionInput::Address(address), instruction_cycles) => {
                    self.memory
                        .write_word(self.registers.pc - 1, self.registers.sp as Word);
                    self.registers.sp += 2;
                    self.registers.pc = address;

                    cycles -= instruction_cycles as i32;
                }
                _ => {}
            }
        }

        cycles_requested - cycles
    }

    fn fetch_instruction(&mut self) -> DecodedInstruction {
        let code = self.fetch_byte();
        let (instruction, mode, register, cycles) = INSTRUCTION_CODE[code as usize];
        let mut cycles = cycles;

        let input = match mode {
            AddressingMode::Unknown => InstructionInput::Unknown,
            AddressingMode::Accumulator => InstructionInput::Unknown,
            AddressingMode::Immediate => InstructionInput::Immediate(self.fetch_byte()),
            AddressingMode::Implied => InstructionInput::Unknown,
            AddressingMode::Relative => InstructionInput::Unknown,
            AddressingMode::Absolute(variant) => match register {
                ImpliedRegister::None => InstructionInput::Address(self.fetch_word()),
                ImpliedRegister::X => {
                    let address = self.fetch_word();
                    let address_x = address + self.registers.x as Word;
                    if address_x - address >= 0xFF && variant {
                        cycles += 1;
                    }

                    InstructionInput::Address(address_x)
                }
                ImpliedRegister::Y => {
                    let address = self.fetch_word();
                    let address_y = address + self.registers.y as Word;
                    if address_y - address >= 0xFF {
                        cycles += 1;
                    }

                    InstructionInput::Address(address_y)
                }
            },
            AddressingMode::ZeroPage => match register {
                ImpliedRegister::None => InstructionInput::Address(self.fetch_byte() as Word),
                ImpliedRegister::X => {
                    let address = self.fetch_byte();
                    let address = address.wrapping_add(self.registers.x);
                    InstructionInput::Address(address as Word)
                }
                ImpliedRegister::Y => {
                    let address = self.fetch_byte();
                    let address = address.wrapping_add(self.registers.y);
                    InstructionInput::Address(address as Word)
                }
            },
            AddressingMode::Indirect => InstructionInput::Unknown,
            AddressingMode::IndexedIndirect => match register {
                ImpliedRegister::None => InstructionInput::Unknown,
                ImpliedRegister::X => {
                    let address = self.fetch_byte() + self.registers.x;
                    let address = self.read_word(address as Word);

                    InstructionInput::Address(address)
                }
                ImpliedRegister::Y => InstructionInput::Unknown,
            },
            AddressingMode::IndirectIndexed(variant) => match register {
                ImpliedRegister::None => InstructionInput::Unknown,
                ImpliedRegister::X => InstructionInput::Unknown,
                ImpliedRegister::Y => {
                    let address = self.fetch_byte();
                    let address = self.read_word(address as Word);
                    let address_y = address + self.registers.y as Word;

                    if address_y - address >= 0xFF && variant {
                        cycles += 1;
                    }

                    InstructionInput::Address(address_y)
                }
            },
        };

        (instruction, input, cycles)
    }

    fn fetch_byte(&mut self) -> Byte {
        let data = self.memory[self.registers.pc];
        self.registers.pc += 1;
        data
    }

    fn fetch_word(&mut self) -> Word {
        // 6502 is little endian
        let mut data = self.memory[self.registers.pc] as Word;
        self.registers.pc += 1;

        data |= (self.memory[self.registers.pc] as Word) << 8;
        self.registers.pc += 1;

        data
    }

    fn read_byte(&mut self, address: Word) -> Byte {
        self.memory[address]
    }

    fn read_word(&mut self, address: Word) -> Word {
        let low = self.read_byte(address);
        let high = self.read_byte(address + 1);

        low as Word | (high as Word) << 8
    }
}
