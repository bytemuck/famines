use emu6502::*;

#[test]
fn sec_implied() {
    let mut processor = Processor::new();

    processor.registers.c = false;

    processor.memory[0xFFFC] = SEC_IMPLIED;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.c, true);
}
