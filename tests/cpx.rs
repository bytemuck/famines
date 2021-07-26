use emu6502::*;

#[test]
fn cpx_immediate() {
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x45;

        processor.memory[0xFFFC] = CPX_IMMEDIATE;
        processor.memory[0xFFFD] = 0x42;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.n, false);
        assert_eq!(processor.registers.c, true);
        assert_eq!(processor.registers.z, false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x42;

        processor.memory[0xFFFC] = CPX_IMMEDIATE;
        processor.memory[0xFFFD] = 0x42;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.n, false);
        assert_eq!(processor.registers.c, true);
        assert_eq!(processor.registers.z, true);
    }
}

#[test]
fn cpx_zero_page() {
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x68;

        processor.memory[0xFFFC] = CPX_ZERO_PAGE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0x0042] = 0x67;

        let expected_cycles = 3;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.n, false);
        assert_eq!(processor.registers.c, true);
        assert_eq!(processor.registers.z, false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x67;

        processor.memory[0xFFFC] = CPX_ZERO_PAGE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0x0042] = 0x67;

        let expected_cycles = 3;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.n, false);
        assert_eq!(processor.registers.c, true);
        assert_eq!(processor.registers.z, true);
    }
}

#[test]
fn cpx_absolute() {
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x68;

        processor.memory[0xFFFC] = CPX_ABSOLUTE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4242] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.n, false);
        assert_eq!(processor.registers.c, true);
        assert_eq!(processor.registers.z, false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x67;

        processor.memory[0xFFFC] = CPX_ABSOLUTE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4242] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.n, false);
        assert_eq!(processor.registers.c, true);
        assert_eq!(processor.registers.z, true);
    }
}
