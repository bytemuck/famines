use emu6502::*;

#[test]
fn php_implied() {
    let mut processor = Processor::new();

    processor.registers.status = 0xCC;

    processor.memory[0xFFFC] = PHP_IMPLIED;

    let expected_cycles = 3;
    let used_cycles = processor.execute_cycles(expected_cycles);

    //let byte = 0b00110000;

    assert_eq!(
        processor.memory[processor.sp_to_address().to_word() + 1], // because the stack pointer decrement when you write a to the stack.
        0xCC | FLAG_BREAK | FLAG_UNUSED
    );
    assert_eq!(used_cycles, expected_cycles);
}
