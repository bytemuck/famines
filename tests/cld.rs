use emu6502::*;

#[test]
fn cld_implied() {
    let mut processor = Processor::new();

    processor.registers.set_decimal(true);

    processor.memory[0xFFFC] = CLD_IMPLIED;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_decimal(), false);
}
