use emu6502::*;

#[test]
fn bcc_relative() {
    let mut processor = Processor::new();

    processor.registers.set_carry(true);

    processor.memory[0xFFFC] = BCC_RELATIVE;
    processor.memory[0xFFFD] = 0x1;

    let expected_cycles = 3;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(processor.registers.pc, 0xFFFF);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), false);
}