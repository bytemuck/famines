use emu6502::*;

#[test]
fn pha_implied() {
    let mut processor = Processor::new();

    processor.registers.a = 0x42;

    processor.memory[0xFFFC] = PHA_IMPLIED;

    let expected_cycles = 3;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(
        processor.memory[processor.sp_to_address().to_word() + 1], // because the stack pointer decrement when you write a to the stack.
        0x42
    );
    assert_eq!(used_cycles, expected_cycles);
}
