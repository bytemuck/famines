use emu6502::{Address, Processor};

fn main() {
    let mut processor = Processor::new();
    processor.registers.pc = Address(0x400);

    processor.memory.bytes[0x000A..]
        .clone_from_slice(&(*include_bytes!("6502_functional_test.bin")));

    processor.execute();
}
