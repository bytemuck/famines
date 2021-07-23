use emu6502::*;

#[test]
fn sed_implied() {
    let mut processor = Processor::new();

    processor.registers.set_decimal(false);

    processor.memory[0xFFFC] = SED_IMPLIED;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_decimal(), true);
}
