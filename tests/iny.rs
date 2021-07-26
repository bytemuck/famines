use emu6502::*;

#[test]
fn iny_implied() {
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x42;

        processor.memory[0xFFFC] = INY_IMPLIED;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(processor.registers.y, 0x43);
        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.z, false);
        assert_eq!(processor.registers.n, false);
    }

    {
        let mut processor = Processor::new();

        processor.registers.y = 0b0111_1111;

        processor.memory[0xFFFC] = INY_IMPLIED;

        let expected_cycles = 2;
        let used_cycles = processor.execute_cycles(expected_cycles);

        assert_eq!(processor.registers.y, 0b1000_0000);
        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.z, false);
        assert_eq!(processor.registers.n, true);
    }
}
