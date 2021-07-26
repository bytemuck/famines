use emu6502::*;

#[test]
fn rti_implied() {
    let mut processor = Processor::new();

    processor.memory[0x01FF] = 0x42;
    processor.memory[0x01FE] = 0x42;
    processor.memory[0x01FD] = 0b1010_1010;
    processor.registers.sp = 0xFC;

    processor.memory[0xFFFC] = RTI_IMPLIED;

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.to_byte(), 0b1000_1010);
    assert_eq!(processor.registers.sp, 0xFF);
    assert_eq!(processor.registers.pc, 0x4242);
    assert_eq!(used_cycles, expected_cycles);
}
