use emu6502::*;

#[test]
fn dey_implied() {
    let mut processor = Processor::new();

    processor.registers.y = 0x42;
    processor.memory[0xFFFC] = DEY_IMPLIED;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.y, 0x41);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.z, false);
    assert_eq!(processor.registers.n, false);
}
