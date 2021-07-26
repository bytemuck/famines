use emu6502::*;

#[test]
fn clv_implied() {
    let mut processor = Processor::new();

    processor.registers.v = true;

    processor.memory[0xFFFC] = CLV_IMPLIED;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.v, false);
}
