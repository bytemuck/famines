use crate::memory::{Address, Byte, Memory};

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_SIZE: usize = 16384;
const CHR_SIZE: usize = 8192;

pub struct Cartridge {
    pub prg: Vec<Byte>,
    pub chr: Vec<Byte>,
    pub(crate) _mapper: u8,
    pub(crate) _mirroring: Mirroring,
}

impl Cartridge {
    pub fn new(raw: &[u8]) -> Result<Cartridge, String> {
        if raw[0..4] != NES_TAG {
            return Err("File is not in iNES file format".to_string());
        }

        let _mapper = (raw[7] & 0b1111_0000) | (raw[6] >> 4);

        let ines_ver = (raw[7] >> 2) & 0b11;
        if ines_ver != 0 {
            return Err("NES2.0 format is not supported".to_string());
        }

        let four_screen = raw[6] & 0b1000 != 0;
        let vertical_mirroring = raw[6] & 0b1 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = raw[4] as usize * PRG_SIZE;
        let chr_rom_size = raw[5] as usize * CHR_SIZE;

        let skip_trainer = raw[6] & 0b100 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Self {
            prg: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            _mapper,
            _mirroring: screen_mirroring,
        })
    }
}

impl Memory for Cartridge {
    fn read_byte(&mut self, address: Address) -> Byte {
        let mut address = address;
        if self.prg.len() == 0x4000 && address >= 0x4000 {
            address %= 0x4000;
        }

        self.prg[address as usize]
    }

    fn write_byte(&mut self, _address: Address, _value: Byte) {
        panic!("You cannot write to a Cartridge.");
    }
}
