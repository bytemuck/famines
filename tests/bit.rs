use emu6502::*;

#[test]
fn bit_zero_page() {
    let mut processor = Processor::new();

    processor.registers.a = 0b0100_0000;

    processor.memory[0xFFFC] = BIT_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0042] = 0b1100_0000;

    let expected_cycles = 3;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.z, false);

    assert_eq!(processor.registers.v, true);
    assert_eq!(processor.registers.n, true);
}

#[test]
fn bit_absolute() {
    let mut processor = Processor::new();

    processor.registers.a = 0b0100_0000;

    processor.memory[0xFFFC] = BIT_ABSOLUTE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = 0b1100_0000;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.z, false);

    assert_eq!(processor.registers.v, true);
    assert_eq!(processor.registers.n, true);
}
