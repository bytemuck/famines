use emu6502::*;

#[test]
fn lsr_accumulator() {
    let mut processor = Processor::new();

    processor.registers.a = 0b1100_0101; // -> 0b0110_0010

    processor.memory[0xFFFC] = LSR_ACCUMULATOR;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0b0110_0010);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.n, false);
    assert_eq!(processor.registers.z, false);
    assert_eq!(processor.registers.c, true);
}

#[test]
fn lsr_zero_page() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LSR_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0042] = 0b1100_0101;

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x0042], 0b0110_0010);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.n, false);
    assert_eq!(processor.registers.z, false);
    assert_eq!(processor.registers.c, true);
}

#[test]
fn lsr_zero_page_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0x10;

    processor.memory[0xFFFC] = LSR_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0052] = 0b1100_0101;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x0052], 0b0110_0010);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.n, false);
    assert_eq!(processor.registers.z, false);
    assert_eq!(processor.registers.c, true);
}

#[test]
fn lsr_absolute() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LSR_ABSOLUTE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = 0b1100_0101;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x4242], 0b0110_0010);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.n, false);
    assert_eq!(processor.registers.z, false);
    assert_eq!(processor.registers.c, true);
}

#[test]
fn lsr_absolute_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0x10;

    processor.memory[0xFFFC] = LSR_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4252] = 0b1100_0101;

    let expected_cycles = 7;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x4252], 0b0110_0010);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.n, false);
    assert_eq!(processor.registers.z, false);
    assert_eq!(processor.registers.c, true);
}
