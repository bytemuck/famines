use emu6502::*;

#[test]
fn nop_implied() {
    let mut processor = Processor::new();

    processor.registers.a = 0b1100_0101; // -> 0b0110_0010

    processor.memory[0xFFFC] = NOP_IMPLIED;
    processor.memory[0xFFFD] = NOP_IMPLIED;
    processor.memory[0xFFFE] = NOP_IMPLIED;
    processor.memory[0xFFFF] = LSR_ACCUMULATOR;

    let expected_cycles = 8;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0b0110_0010);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_negative(), false);
    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_carry(), true);
}
