use emu6502::*;

#[test]
fn sta_zero_page() {
    let mut processor = Processor::new();

    processor.registers.a = 0x42;

    processor.memory[0xFFFC] = STA_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x16;

    let expected_cycles = 3;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x0016], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn sta_absolute() {
    let mut processor = Processor::new();

    processor.registers.a = 0x42;

    processor.memory[0xFFFC] = STA_ABSOLUTE;
    processor.memory[0xFFFD] = 0x16;
    processor.memory[0xFFFE] = 0x16;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x1616], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn sta_indirect_x() {
    let mut processor = Processor::new();

    processor.registers.a = 0x42;
    processor.registers.x = 0x04;

    processor.memory[0xFFFC] = STA_INDIRECT_X;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0006] = 0x00; // 0x04 + 0x02
    processor.memory[0x0007] = 0x80;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x8000], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn sta_indirect_y() {
    let mut processor = Processor::new();

    processor.registers.y = 0x04;

    processor.memory[0xFFFC] = STA_INDIRECT_X;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0002] = 0x00;
    processor.memory[0x0003] = 0x80;
    processor.memory[0x8004] = 0x42; // 0x8000 + 0x04

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x8004], 0x42);
    assert_eq!(used_cycles, expected_cycles);
}
