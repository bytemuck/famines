use emu6502::*;

#[test]
fn negate8() {
    let mut processor = Processor::new();

    processor.registers.a = 32;

    processor.memory[0xFFFC] = JSR_ABSOLUTE; // 6
    processor.memory[0xFFFD] = 0x00;
    processor.memory[0xFFFE] = 0x10;

    processor.memory[0x1000] = CLC_IMPLIED; // 2
    processor.memory[0x1001] = EOR_IMMEDIATE; // 2
    processor.memory[0x1002] = 0xFF;
    processor.memory[0x1003] = ADC_IMMEDIATE; // 2
    processor.memory[0x1004] = 0x01;

    let expected_cycles = 6 + 2 + 2 + 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, (-32i8) as u8);
    assert_eq!(used_cycles, expected_cycles);
}
