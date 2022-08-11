use crate::Cartridge;
use crate::Memory;
use crate::cartridge;

pub struct BUS {
    ram: [u8; 2048],
    cartridge: Cartridge,
}

impl BUS {
    const RAM_BEGIN: u16 = 0x0000;
    const RAM_MIRRORS_END: u16 = 0x1FFF;
    const PPU_REGISTERS_BEGIN: u16 = 0x2000;
    const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            ram: [0; 2048],
            cartridge
        }
    }
    
    pub fn read_prg(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.cartridge.prg.len() == 0x4000 && addr >= 0x4000 {
            addr %= 0x4000;
        }

        self.cartridge.prg[addr as usize]
    }
}

impl Memory for BUS {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            Self::RAM_BEGIN..=Self::RAM_MIRRORS_END => {
                let addr = addr & 0b0000_0111_1111_1111;
                self.ram[addr as usize]
            }
            Self::PPU_REGISTERS_BEGIN..=Self::PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                //todo!("PPU is not supported yet");
                0
            }
            0x8000..=0xFFFF => self.read_prg(addr),
            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            Self::RAM_BEGIN..=Self::RAM_MIRRORS_END => {
                let addr = addr & 0b0000_0111_1111_1111;
                self.ram[addr as usize] = value;
            }
            Self::PPU_REGISTERS_BEGIN..=Self::PPU_REGISTERS_MIRRORS_END => {
                let _addr = addr & 0b0010_0000_0000_0111;
                // todo!("PPU is not supported yet");
            }
            0x8000..=0xFFFF => panic!("Attempt to write to Cartridge ROM space: {:x}", addr),
            _ => {
                println!("Ignoring write to address: {:X}", addr);
            }
        }
    }
}