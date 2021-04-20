use emu6502::*;

#[test]
fn pla_implied() {
    let mut processor = Processor::new();

    processor.stack_push(0x42);

    processor.memory[0xFFFC] = PLA_IMPLIED;

    let expected_cycles = 3;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.a, 0x42);
    assert_eq!(used_cycles, expected_cycles);
}
