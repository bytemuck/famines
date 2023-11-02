pub mod addressing;
pub mod ram;

pub type Byte = u8;
pub type Offset = i8;

pub type Word = u16;
pub type DWord = u32;

pub type Address = Word;
pub type ZeroPageAddress = Byte;

pub trait Memory {
    fn read_byte(&mut self, address: Address) -> Byte;
    fn read_word(&mut self, address: Address) -> Word {
        let low = self.read_byte(address) as Word;
        let high = self.read_byte(address + 1) as Word;

        (high << 8) | low
    }

    fn write_byte(&mut self, address: Address, value: Byte);
    fn write_word(&mut self, address: Address, value: Word) {
        self.write_byte(address, (value >> 8) as Byte);
        self.write_byte(address + 1, (value & 0xff) as Byte)
    }
}

pub trait ZeroPageMemory: Memory {
    fn read_word_zero_page(&mut self, address: ZeroPageAddress) -> Word {
        let low = self.read_byte(address as Word) as Word;
        let high = self.read_byte((address + 1) as Word) as Word;

        (high << 8) | low
    }
}
