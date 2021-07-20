use emu6502::*;

#[test]
fn sty_zero_page() {
    let mut processor = Processor::new();

    processor.registers.y = 0x42;

    processor.memory[0xFFFC] = STY_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x16;

    let expected_cycles = 3;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.memory[0x0016], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn sty_zero_page_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0x12;
    processor.registers.y = 0x42;

    processor.memory[0xFFFC] = STY_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x16;

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.memory[0x0028], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn sty_absolute() {
    let mut processor = Processor::new();

    processor.registers.y = 0x42;

    processor.memory[0xFFFC] = STY_ABSOLUTE;
    processor.memory[0xFFFD] = 0x16;
    processor.memory[0xFFFE] = 0x16;

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.memory[0x1616], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}
