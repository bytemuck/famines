use famines::{
    cartridge::Cartridge,
    cpu::CPU,
    memory::{ram::RAM, Byte},
};

fn main() {
    let bytes: Vec<Byte> = std::fs::read("res/nestest.nes").unwrap();
    let cartridge = Cartridge::new(&bytes).unwrap();

    let mut cpu = CPU::new(cartridge, RAM::new());
    cpu.reset();
    cpu.registers.pc = 0xC000;

    loop {
        if !cpu.step() {
            break;
        }
    }
}
