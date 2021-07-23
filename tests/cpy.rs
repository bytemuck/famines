use emu6502::*;

#[test]
fn cpy_immediate() {
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x45;

        processor.memory[0xFFFC] = CPY_IMMEDIATE;
        processor.memory[0xFFFD] = 0x42;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x42;

        processor.memory[0xFFFC] = CPY_IMMEDIATE;
        processor.memory[0xFFFD] = 0x42;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cpy_zero_page() {
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x68;

        processor.memory[0xFFFC] = CPY_ZERO_PAGE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0x0042] = 0x67;

        let expected_cycles = 3;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x67;

        processor.memory[0xFFFC] = CPY_ZERO_PAGE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0x0042] = 0x67;

        let expected_cycles = 3;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cpy_absolute() {
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x68;

        processor.memory[0xFFFC] = CPY_ABSOLUTE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4242] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x67;

        processor.memory[0xFFFC] = CPY_ABSOLUTE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4242] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}
