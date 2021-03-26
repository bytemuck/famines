use emu6502::*;

#[test]
fn adc_immediate() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;

    processor.memory[0xFFFC] = ADC_IMMEDIATE;
    processor.memory[0xFFFD] = 0x42;

    let expected_cycles = 2;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_zero_page() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;

    processor.memory[0xFFFC] = ADC_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0042] = 0x52;

    let expected_cycles = 3;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x52 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), true); // 133 is 0b1000_0101
}

#[test]
fn adc_zero_page_x() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;
    processor.registers.x = 0x14;

    processor.memory[0xFFFC] = ADC_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0056] = 0x42; // 0x0042 + 0x14

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_absolute() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;

    processor.memory[0xFFFC] = ADC_ABSOLUTE;
    processor.memory[0xFFFD] = 0x16;
    processor.memory[0xFFFE] = 0x16; // 0x4242
    processor.memory[0x1616] = 0x42;

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_absolute_x() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;
    processor.registers.x = 0x14;

    processor.memory[0xFFFC] = ADC_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x16;
    processor.memory[0xFFFE] = 0x16; // 0x1616
    processor.memory[0x162A] = 0x42; // 0x1616 + 0x14

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_absolute_x_crosses() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;
    processor.registers.x = 0xFF;

    processor.memory[0xFFFC] = ADC_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0xFFFE] = 0x44; // 0x4402
    processor.memory[0x4501] = 0x42; // 0x4402 + 0xFF crosses page boundary!

    let expected_cycles = 5;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_absolute_y() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;
    processor.registers.y = 0x14;

    processor.memory[0xFFFC] = ADC_ABSOLUTE_Y;
    processor.memory[0xFFFD] = 0x16;
    processor.memory[0xFFFE] = 0x16; // 0x1616
    processor.memory[0x162A] = 0x42; // 0x1616 + 0x14

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_absolute_y_crosses() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;
    processor.registers.y = 0xFF;

    processor.memory[0xFFFC] = ADC_ABSOLUTE_Y;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0xFFFE] = 0x44; // 0x4402
    processor.memory[0x4501] = 0x42; // 0x4402 + 0xFF crosses page boundary!

    let expected_cycles = 5;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_indirect_x() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;
    processor.registers.x = 0x04;

    processor.memory[0xFFFC] = ADC_INDIRECT_X;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0006] = 0x00; // 0x02 + 0x04
    processor.memory[0x0007] = 0x80;
    processor.memory[0x8000] = 0x42;

    let expected_cycles = 6;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_indirect_y() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;
    processor.registers.y = 0x04;

    processor.memory[0xFFFC] = ADC_INDIRECT_Y;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0002] = 0x00;
    processor.memory[0x0003] = 0x80;
    processor.memory[0x8004] = 0x42; // 0x8000 + 0x04

    let expected_cycles = 5;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn adc_indirect_y_crosses() {
    let mut processor = Processor::new();

    processor.registers.status |= FLAG_CARRY;
    processor.registers.a = 0x32;
    processor.registers.y = 0xFF;

    processor.memory[0xFFFC] = ADC_INDIRECT_Y;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0002] = 0x02;
    processor.memory[0x0003] = 0x80;
    processor.memory[0x8101] = 0x42; // 0x8000 + 0x04

    let expected_cycles = 6;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 + 0x42 + 0x01);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}
