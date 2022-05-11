use emu6502::*;

#[test]
fn ror_implied() {
    {
        let mut processor = Processor::new();

        processor.registers.c = true; // 1
        processor.registers.a = 0b0111_1111;

        processor.memory[0xFFFC] = ROR_ACCUMULATOR;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(processor.registers.c, true);
        assert_eq!(processor.registers.a, 0b1011_1111);
        assert_eq!(used_cycles, expected_cycles);
    }
    {
        let mut processor = Processor::new();

        processor.registers.c = false; // 0
        processor.registers.a = 0b0111_1111;

        processor.memory[0xFFFC] = ROR_ACCUMULATOR;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(processor.registers.c, true);
        assert_eq!(processor.registers.a, 0b0011_1111);
        assert_eq!(used_cycles, expected_cycles);
    }
    {
        let mut processor = Processor::new();

        processor.registers.c = true; // 0
        processor.registers.a = 0b1111_1110;

        processor.memory[0xFFFC] = ROR_ACCUMULATOR;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(processor.registers.c, false);
        assert_eq!(processor.registers.a, 0b1111_1111);
        assert_eq!(used_cycles, expected_cycles);
    }
    {
        let mut processor = Processor::new();

        processor.registers.c = false; // 1
        processor.registers.a = 0b1111_1110;

        processor.memory[0xFFFC] = ROR_ACCUMULATOR;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(processor.registers.c, false);
        assert_eq!(processor.registers.a, 0b0111_1111);
        assert_eq!(used_cycles, expected_cycles);
    }
}

#[test]
fn ror_zero_page() {
    let mut processor = Processor::new();

    processor.registers.c = true; // 1

    processor.memory[0xFFFC] = ROR_ZERO_PAGE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0042] = 0b0111_1110;

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.c, false);
    assert_eq!(processor.memory[0x0042], 0b1011_1111);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn rol_zero_page_x() {
    let mut processor = Processor::new();

    processor.registers.c = true; // 1
    processor.registers.x = 0x32;

    processor.memory[0xFFFC] = ROR_ZERO_PAGE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0x0074] = 0b0111_1110;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.c, false);
    assert_eq!(processor.memory[0x0074], 0b1011_1111);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn rol_absolute() {
    let mut processor = Processor::new();

    processor.registers.c = true; // 1

    processor.memory[0xFFFC] = ROR_ABSOLUTE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = 0b0111_1110;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.c, false);
    assert_eq!(processor.memory[0x4242], 0b1011_1111);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn rol_absolute_x() {
    let mut processor = Processor::new();

    processor.registers.c = true; // 1
    processor.registers.x = 0x32;

    processor.memory[0xFFFC] = ROR_ABSOLUTE_X;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4274] = 0b0111_1110;

    let expected_cycles = 7;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.c, false);
    assert_eq!(processor.memory[0x4274], 0b1011_1111);
    assert_eq!(used_cycles, expected_cycles);
}