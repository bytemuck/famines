use emu6502::*;

#[test]
fn dex_implied() {
    let mut processor = Processor::new();

    processor.registers.x = 0x42;
    processor.memory[0xFFFC] = DEX_IMPLIED;

    let expected_cycles = 2;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.x, 0x41);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}
