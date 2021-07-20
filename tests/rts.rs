use emu6502::*;

#[test]
fn rts_implied() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = JSR_ABSOLUTE; // 6
    processor.memory[0xFFFD] = 0x00;
    processor.memory[0xFFFE] = 0x10;

    processor.memory[0x1000] = JSR_ABSOLUTE; // 6
    processor.memory[0x1001] = 0x00;
    processor.memory[0x1002] = 0x20;
    processor.memory[0x2000] = RTS_IMPLIED; // 6
    processor.memory[0x1003] = LDA_IMMEDIATE; // 2
    processor.memory[0x1004] = 0x42;

    let expected_cycles = 6 + 6 + 6 + 2;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);
    assert_eq!(processor.registers.a, 0x42);
}
