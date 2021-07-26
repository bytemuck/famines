use emu6502::*;

#[test]
fn jsr_absolute() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = JSR_ABSOLUTE; // 6
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = LDA_IMMEDIATE; // 2
    processor.memory[0x4243] = 0x84;

    let expected_cycles = 8;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x84);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.z, false);
    assert_eq!(processor.registers.n, true);
}
