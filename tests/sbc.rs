use emu6502::*;

#[test]
fn sbc_immediate() {
    let mut processor = Processor::new();

    processor.registers.set_carry(false);
    processor.registers.a = 0x23;

    processor.memory[0xFFFC] = SBC_IMMEDIATE;
    processor.memory[0xFFFD] = 0x25;

    let expected_cycles = 2;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, ((0x23 - 0x25 - (1 - 0)) as i8) as u8);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), true);
}

#[test]
fn sbc_absolute() {
    let mut processor = Processor::new();

    processor.registers.set_carry(false);
    processor.registers.a = 0x23;

    processor.memory[0xFFFC] = SBC_ABSOLUTE;
    processor.memory[0xFFFD] = 0x25;
    processor.memory[0xFFFE] = 0x25;
    processor.memory[0x2525] = 0x32;

    let expected_cycles = 4;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, ((0x23 - 0x32 - (1 - 0)) as i8) as u8);
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), true);
}
