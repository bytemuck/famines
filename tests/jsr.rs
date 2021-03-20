use emu6502::*;

#[test]
fn jsr_immediate() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = JSR_ABSOLUTE;
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = LDA_IMMEDIATE;
    processor.memory[0x4243] = 0x84;

    let used_cycles = processor.execute(8);

    assert_eq!(processor.registers.a, 0x84);
    assert_eq!(used_cycles, 8);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), true);
}
