use emu6502::Processor;

// this should panic because when it reaches the end of the program,
// the program continue since there is no way to stop a cpu executing the next instruction
// and the stack pointer overflow, if its printing "success!" we reached the end.
fn main() {
    let mut processor = Processor::new();
    processor.registers.pc = 0x400;

    processor.memory.bytes[0x000A..]
        .clone_from_slice(&(*include_bytes!("6502_functional_test.bin")));

    processor.execute();
}
