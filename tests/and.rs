use emu6502::*;

trait And {
    fn new_and() -> Processor;
}

impl And for Processor {
    fn new_and() -> Processor {
        let mut p = Processor::new();

        p.registers.a = 0x32;

        p
    }
}

#[test]
fn and_immediate() {
    let mut processor = Processor::new_and();

    processor.memory[0xFFFC] = AND_IMMEDIATE;
    processor.memory[0xFFFD] = 0x42;

    let expected_cycles = 2;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x42);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_zero_page() {
    let mut processor = Processor::new_and();

    processor.memory[0xFFFC] = AND_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x42] = 0x84;

    let expected_cycles = 3;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x84);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), true); // gives 0
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_zero_page_x() {
    let mut processor = Processor::new_and();

    processor.registers.x = 0xFF;

    processor.memory[0xFFFC] = AND_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x80;
    processor.memory[0x007F] = 0x37;

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_absolute() {
    let mut processor = Processor::new_and();

    processor.memory[0xFFFC] = AND_ABSOLUTE;
    processor.memory[0xFFFD] = 0x80;
    processor.memory[0xFFFE] = 0x44; // 0x4480
    processor.memory[0x4480] = 0x37;

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_absolute_x() {
    let mut processor = Processor::new_and();

    processor.registers.x = 0x01;

    processor.memory[0xFFFC] = AND_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x80;
    processor.memory[0xFFFE] = 0x44; // 0x4480
    processor.memory[0x4481] = 0x37;

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_absolute_x_crosses() {
    let mut processor = Processor::new_and();

    processor.registers.x = 0xFF;

    processor.memory[0xFFFC] = AND_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0xFFFE] = 0x44; // 0x4402
    processor.memory[0x4501] = 0x37; // 0x4402+0xFF crosses page boundary!

    let expected_cycles = 5;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_absolute_y() {
    let mut processor = Processor::new_and();
    processor.registers.y = 0x01;

    processor.memory[0xFFFC] = AND_ABSOLUTE_Y;
    processor.memory[0xFFFD] = 0x80;
    processor.memory[0xFFFE] = 0x44; // 0x4480
    processor.memory[0x4481] = 0x37;

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_absolute_y_crosses() {
    let mut processor = Processor::new_and();

    processor.registers.y = 0xFF;

    processor.memory[0xFFFC] = AND_ABSOLUTE_Y;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0xFFFE] = 0x44; // 0x4402
    processor.memory[0x4501] = 0x37; // 0x4402+0xFF crosses page boundary!

    let expected_cycles = 5;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_indirect_x() {
    let mut processor = Processor::new_and();
    processor.registers.x = 0x04;

    processor.memory[0xFFFC] = AND_INDIRECT_X;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0006] = 0x00; // 0x02 + 0x04
    processor.memory[0x0007] = 0x80;
    processor.memory[0x8000] = 0x37;

    let expected_cycles = 6;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_indirect_y() {
    let mut processor = Processor::new_and();
    processor.registers.y = 0x04;

    processor.memory[0xFFFC] = AND_INDIRECT_Y;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0002] = 0x00;
    processor.memory[0x0003] = 0x80;
    processor.memory[0x8004] = 0x37; // 0x8000 + 0x04

    let expected_cycles = 5;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}

#[test]
fn and_indirect_y_crosses() {
    let mut processor = Processor::new_and();
    processor.registers.y = 0xFF;

    processor.memory[0xFFFC] = AND_INDIRECT_Y;
    processor.memory[0xFFFD] = 0x02;
    processor.memory[0x0002] = 0x02;
    processor.memory[0x0003] = 0x80;
    processor.memory[0x8101] = 0x37; // 0x8000 + 0xFF

    let expected_cycles = 6;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x32 & 0x37);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}
