use emu6502::*;

#[test]
fn stx_zero_page() {
    let mut processor = Processor::new();

    processor.registers.x = 0x42;

    processor.memory[0xFFFC] = STX_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x16;

    let expected_cycles = 3;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x0016], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn stx_zero_page_y() {
    let mut processor = Processor::new();

    processor.registers.y = 0x12;
    processor.registers.x = 0x42;

    processor.memory[0xFFFC] = STX_ZERO_PAGE_Y;
    processor.memory[0xFFFD] = 0x16;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x0028], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn stx_absolute() {
    let mut processor = Processor::new();

    processor.registers.x = 0x42;

    processor.memory[0xFFFC] = STX_ABSOLUTE;
    processor.memory[0xFFFD] = 0x16;
    processor.memory[0xFFFE] = 0x16;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x1616], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}
