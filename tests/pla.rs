use emu6502::*;

#[test]
fn pla_implied() {
    let mut processor = Processor::new();

    processor.memory[0x01FF] = 0x42;
    processor.registers.sp = 0xFE;

    processor.memory[0xFFFC] = PLA_IMPLIED; // 4 cycles

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x42);
    assert_eq!(used_cycles, expected_cycles);
}
