use emu6502::*;

#[test]
fn finish_instruction() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LDA_IMMEDIATE;
    processor.memory[0xFFFD] = 0x42;

    let used_cycles = processor.execute_cycles(1);

    assert_eq!(used_cycles, 2);
}

#[test]
fn no_cycles() {
    let mut processor = Processor::new();

    let used_cycles = processor.execute_cycles(0);

    assert_eq!(used_cycles, 0);
}
