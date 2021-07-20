use emu6502::*;

#[test]
fn tay_implied() {
    let mut processor = Processor::new();

    processor.registers.a = 0x42;

    processor.memory[0xFFFC] = TAY_IMPLIED;

    let expected_cycles = 2;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.y, 0x42);
    assert_eq!(used_cycles, expected_cycles);
}
