use emu6502::*;

trait Asl {
    fn new_asl() -> Processor;
}

impl Asl for Processor {
    fn new_asl() -> Processor {
        let mut p = Processor::new();

        p.registers.a = 0x32;
        p.registers.x = 0x62;

        p
    }
}

#[test]
fn asl_accumulator() {
    let mut processor = Processor::new_asl();

    processor.memory[0xFFFC] = ASL_ACCUMULATOR;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 << 1);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn asl_zero_page() {
    let mut processor = Processor::new_asl();

    processor.memory[0xFFFC] = ASL_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0042] = 0x32;

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x0042], 0x32 << 1);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn asl_zero_page_x() {
    let mut processor = Processor::new_asl();

    processor.memory[0xFFFC] = ASL_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x00A4] = 0x32; // 0x42 + 0x62

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x00A4], 0x32 << 1);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn asl_absolute() {
    let mut processor = Processor::new_asl();

    processor.memory[0xFFFC] = ASL_ABSOLUTE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = 0x32;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x4242], 0x32 << 1);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn asl_absolute_x() {
    let mut processor = Processor::new_asl();

    processor.memory[0xFFFC] = ASL_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x42A4] = 0x32; // 0x4242 + 0x62

    let expected_cycles = 7;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.memory[0x42A4], 0x32 << 1);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}
