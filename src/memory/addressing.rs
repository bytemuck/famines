use crate::cpu::CPU;
use crate::memory::Address;
use crate::memory::Byte;
use crate::memory::Memory;
use crate::memory::Word;
use crate::memory::ZeroPageMemory;

pub trait Addressing<M: Memory> {
    fn create_addressing(cpu: &mut CPU<M>, page_boundary: bool) -> Self;
    fn get_address(&self) -> Option<Address>;
}

pub trait ReadMode<M: Memory> {
    fn read(&self, cpu: &mut CPU<M>) -> Byte
    where
        Self: Addressing<M> + Sized,
    {
        cpu.memory.read_byte(self.get_address().unwrap())
    }
}

pub trait WriteMode<M: Memory> {
    fn write(&self, cpu: &mut CPU<M>, value: Byte)
    where
        Self: Addressing<M> + Sized,
    {
        cpu.memory.write_byte(self.get_address().unwrap(), value);
    }
}

pub struct Implied;
pub struct Relative;
pub struct Indirect;

// IMMEDIATE

pub struct Immediate;

impl<M: Memory> Addressing<M> for Immediate {
    fn create_addressing(_cpu: &mut CPU<M>, _page_boundary: bool) -> Self {
        Self
    }

    fn get_address(&self) -> Option<Address> {
        None
    }
}

impl<M: Memory> ReadMode<M> for Immediate {
    fn read(&self, cpu: &mut CPU<M>) -> Byte {
        cpu.read_next_byte()
    }
}

// ACCUMULATOR

pub struct Accumulator;

impl<M: Memory> Addressing<M> for Accumulator {
    fn create_addressing(_cpu: &mut CPU<M>, _page_boundary: bool) -> Self {
        Self
    }

    fn get_address(&self) -> Option<Address> {
        None
    }
}

impl<M: Memory> ReadMode<M> for Accumulator {
    fn read(&self, cpu: &mut CPU<M>) -> Byte {
        cpu.registers.a
    }
}

impl<M: Memory> WriteMode<M> for Accumulator {
    fn write(&self, cpu: &mut CPU<M>, value: Byte) {
        cpu.registers.a = value;
    }
}

// ZERO_PAGE

pub struct ZeroPage {
    address: Address,
}

impl<M: Memory> Addressing<M> for ZeroPage {
    fn create_addressing(cpu: &mut CPU<M>, _page_boundary: bool) -> Self {
        Self {
            address: cpu.read_next_byte() as Word,
        }
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.address)
    }
}

impl<M: Memory> ReadMode<M> for ZeroPage {}
impl<M: Memory> WriteMode<M> for ZeroPage {}

// ZERO_PAGE_X

pub struct ZeroPageX {
    address: Address,
}

impl<M: Memory> Addressing<M> for ZeroPageX {
    fn create_addressing(cpu: &mut CPU<M>, _page_boundary: bool) -> Self {
        Self {
            address: (cpu.read_next_byte().wrapping_add(cpu.registers.x)) as Word,
        }
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.address)
    }
}

impl<M: Memory> ReadMode<M> for ZeroPageX {}
impl<M: Memory> WriteMode<M> for ZeroPageX {}

// ZERO_PAGE_Y

pub struct ZeroPageY {
    address: Address,
}

impl<M: Memory> Addressing<M> for ZeroPageY {
    fn create_addressing(cpu: &mut CPU<M>, _page_boundary: bool) -> Self {
        Self {
            address: (cpu.read_next_byte().wrapping_add(cpu.registers.y)) as Word,
        }
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.address)
    }
}

impl<M: Memory> ReadMode<M> for ZeroPageY {}
impl<M: Memory> WriteMode<M> for ZeroPageY {}

// ABSOLUTE

pub struct Absolute {
    address: Address,
}

impl<M: Memory> Addressing<M> for Absolute {
    fn create_addressing(cpu: &mut CPU<M>, _page_boundary: bool) -> Self {
        Self {
            address: cpu.read_next_word(),
        }
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.address)
    }
}

impl<M: Memory> ReadMode<M> for Absolute {}
impl<M: Memory> WriteMode<M> for Absolute {}

// ABSOLUTE_X

pub struct AbsoluteX {
    address: Address,
}

impl<M: Memory> Addressing<M> for AbsoluteX {
    fn create_addressing(cpu: &mut CPU<M>, page_boundary: bool) -> Self {
        let low = cpu.read_next_byte() as Word;
        let high = cpu.read_next_byte() as Word;
        let address = (high << 8 | low).wrapping_add(cpu.registers.x as Word);
        
        if page_boundary {
            if (address & 0xFF00) >> 8 != high as Word {
                cpu.cycles += 1;
            }
        }

        Self {
            address,
        }
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.address)
    }
}

impl<M: Memory> ReadMode<M> for AbsoluteX {}
impl<M: Memory> WriteMode<M> for AbsoluteX {}

// ABSOLUTE_Y

pub struct AbsoluteY {
    address: Address,
}

impl<M: Memory> Addressing<M> for AbsoluteY {
    fn create_addressing(cpu: &mut CPU<M>, page_boundary: bool) -> Self {
        let low = cpu.read_next_byte() as Word;
        let high = cpu.read_next_byte() as Word;
        let address = (high << 8 | low).wrapping_add(cpu.registers.y as Word);
        
        if page_boundary {
            if (address & 0xFF00) >> 8 != high as Word {
                cpu.cycles += 1;
            }
        }

        Self {
            address,
        }
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.address)
    }
}

impl<M: Memory> ReadMode<M> for AbsoluteY {}
impl<M: Memory> WriteMode<M> for AbsoluteY {}

// INDEXED_INDIRECT_X

pub struct IndexedIndirectX {
    address: Address,
}

impl<M: Memory> Addressing<M> for IndexedIndirectX {
    fn create_addressing(cpu: &mut CPU<M>, _page_boundary: bool) -> Self {
        let value = cpu.read_next_byte() + cpu.registers.x;
        let address = cpu.read_word_zero_page(value);
        Self { address }
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.address)
    }
}

impl<M: Memory> ReadMode<M> for IndexedIndirectX {}
impl<M: Memory> WriteMode<M> for IndexedIndirectX {}

// INDIRECT_INDEXED_Y

pub struct IndirectIndexedY {
    address: Address,
}

impl<M: Memory> Addressing<M> for IndirectIndexedY {
    fn create_addressing(cpu: &mut CPU<M>, page_boundary: bool) -> Self {
        let address = cpu.read_next_byte() as Word;

        let low = cpu.read_byte(address) as Word;

        let high = if address < 0xFF {
            cpu.read_byte(address + 1)
        } else {
            cpu.read_byte(0x0000)
        } as Word;

        let address = (high << 8 | low).wrapping_add(cpu.registers.y as Word);
        if page_boundary {
            if (address & 0xFF00) >> 8 != high as Word {
                cpu.cycles += 1;
            }
        }

        Self { address }
    }

    fn get_address(&self) -> Option<Address> {
        Some(self.address)
    }
}

impl<M: Memory> ReadMode<M> for IndirectIndexedY {}
impl<M: Memory> WriteMode<M> for IndirectIndexedY {}
