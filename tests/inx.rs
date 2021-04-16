use emu6502::*;

#[test]
fn inx_implied() {
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x42;

        processor.memory[0xFFFC] = INX_IMPLIED;

        let expected_cycles = 2;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(processor.registers.x, 0x43);
        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_zero(), false);
        assert_eq!(processor.registers.get_negative(), false);
    }

    {
        let mut processor = Processor::new();

        processor.registers.x = 0b0111_1111;

        processor.memory[0xFFFC] = INX_IMPLIED;

        let expected_cycles = 2;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(processor.registers.x, 0b1000_0000);
        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_zero(), false);
        assert_eq!(processor.registers.get_negative(), true);
    }
}
