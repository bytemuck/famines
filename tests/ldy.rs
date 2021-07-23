use emu6502::*;

#[test]
fn ldy_immediate() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LDY_IMMEDIATE;
    processor.memory[0xFFFD] = 0x42;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.y, 0x42);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn ldy_zero_page() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LDY_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x42] = 0x84;

    let expected_cycles = 3;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.y, 0x84);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), true);
}

#[test]
fn ldy_zero_page_x() {
    let mut processor = Processor::new();

    processor.registers.x = 0xFF;

    processor.memory[0xFFFC] = LDY_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x80;
    processor.memory[0x007F] = 0x37;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.y, 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn ldy_absolute() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LDY_ABSOLUTE;
    processor.memory[0xFFFD] = 0x80;
    processor.memory[0xFFFE] = 0x44; // 0x4480
    processor.memory[0x4480] = 0x37;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.y, 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn ldy_absolute_x_crosses() {
    let mut processor = Processor::new();

    processor.registers.x = 0xFF;

    processor.memory[0xFFFC] = LDY_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0xFFFE] = 0x44; // 0x4402
    processor.memory[0x4501] = 0x37; // 0x4402+0xFF crosses page boundary!

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.y, 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn ldy_absolute_x() {
    let mut processor = Processor::new();
    processor.registers.x = 0x01;

    processor.memory[0xFFFC] = LDY_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x80;
    processor.memory[0xFFFE] = 0x44; // 0x4480
    processor.memory[0x4481] = 0x37;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.y, 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}
