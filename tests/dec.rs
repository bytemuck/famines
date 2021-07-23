use emu6502::*;

#[test]
fn dec_zero_page() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = DEC_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0042] = 0x50;

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x0042], 0x50 - 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn dec_zero_page_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0x16;

    processor.memory[0xFFFC] = DEC_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0058] = 0x50;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x0058], 0x50 - 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn dec_absolute() {
    let mut processor = Processor::new();

    processor.registers.x = 0x16;

    processor.memory[0xFFFC] = DEC_ABSOLUTE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = 0x50;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x4242], 0x50 - 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn dec_absolute_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0x16;

    processor.memory[0xFFFC] = DEC_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4258] = 0x50;

    let expected_cycles = 7;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x4258], 0x50 - 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}
