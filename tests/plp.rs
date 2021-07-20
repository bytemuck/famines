use emu6502::*;

#[test]
fn plp_implied() {
    let mut processor = Processor::new();

    processor.memory[0x01FF] = 0x42;
    processor.registers.sp = 0xFE;

    processor.memory[0xFFFC] = PLP_IMPLIED;

    let expected_cycles = 4;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.status, 0x42);
    assert_eq!(used_cycles, expected_cycles);
}
