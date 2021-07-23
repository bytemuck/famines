use emu6502::*;

#[test]
fn eor_immediate() {
    let mut processor = Processor::new();

    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_IMMEDIATE;
    processor.memory[0xFFFD] = 0x42;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
}

#[test]
fn eor_zero_page() {
    let mut processor = Processor::new();

    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x66;
    processor.memory[0x0066] = 0x42;

    let expected_cycles = 3;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
}

#[test]
fn eor_zero_page_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0x10;
    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x66;
    processor.memory[0x0076] = 0x42;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
}

#[test]
fn eor_absolute() {
    let mut processor = Processor::new();

    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_ABSOLUTE;
    processor.memory[0xFFFD] = 0x66;
    processor.memory[0xFFFE] = 0x66;
    processor.memory[0x6666] = 0x42;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
}

#[test]
fn eor_absolute_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0x10;
    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x66;
    processor.memory[0xFFFE] = 0x66;
    processor.memory[0x6676] = 0x42;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
}

#[test]
fn eor_absolute_x_crosses() {
    let mut processor = Processor::new();

    processor.registers.x = 0xFF;
    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x66;
    processor.memory[0xFFFE] = 0x66;
    processor.memory[0x6765] = 0x42;

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
}

#[test]
fn eor_absolute_y() {
    let mut processor = Processor::new();

    processor.registers.y = 0x10;
    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_ABSOLUTE_Y;
    processor.memory[0xFFFD] = 0x66;
    processor.memory[0xFFFE] = 0x66;
    processor.memory[0x6676] = 0x42;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
}

#[test]
fn eor_absolute_y_crosses() {
    let mut processor = Processor::new();

    processor.registers.y = 0xFF;
    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_ABSOLUTE_Y;
    processor.memory[0xFFFD] = 0x66;
    processor.memory[0xFFFE] = 0x66;
    processor.memory[0x6765] = 0x42;

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
}

#[test]
fn eor_indirect_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0x04;
    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_INDIRECT_X;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0006] = 0x00; // 0x02 + 0x04
    processor.memory[0x0007] = 0x80;
    processor.memory[0x8000] = 0x42;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn eor_indirect_y() {
    let mut processor = Processor::new();

    processor.registers.y = 0x04;
    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_INDIRECT_Y;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0002] = 0x00; // 0x02 + 0x04
    processor.memory[0x0003] = 0x80;
    processor.memory[0x8004] = 0x42;

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn eor_indirect_y_crosses() {
    let mut processor = Processor::new();

    processor.registers.y = 0xFF;
    processor.registers.a = 0x45;

    processor.memory[0xFFFC] = EOR_INDIRECT_Y;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0002] = 0x02; // 0x02 + 0x04
    processor.memory[0x0003] = 0x80;
    processor.memory[0x8101] = 0x42; // 0x8002 + 0xFF

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x45 ^ 0x42);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}
