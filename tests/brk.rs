use emu6502::*;

#[test]
fn brk_implied() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = BRK_IMPLIED;
    processor.memory[0xFFFE] = 0x00;
    processor.memory[0xFFFF] = 0x80;

    let expected_cycles = 7;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);
    assert_eq!(processor.registers.pc, Address(0x8000));
}
