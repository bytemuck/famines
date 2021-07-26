use emu6502::*;

#[test]
fn brk_implied() {
    let mut processor = Processor::new();

    processor.registers.pc = 0xFF00;
    processor.memory[0xFF00] = BRK_IMPLIED;
    let oldsp = processor.registers.sp as u16;
    let oldps = processor.registers.to_byte() as u16;

    let expected_cycles = 7;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);
    assert_eq!(processor.memory[(0x0100 | oldsp) - 0], 0xFF);
    assert_eq!(processor.memory[(0x0100 | oldsp) - 1], 0x02);
    assert_eq!(
        processor.memory[(0x0100 | oldsp) - 2],
        oldps as u8 | FLAG_BREAK | FLAG_UNUSED
    );

    assert_eq!(processor.registers.i, true);
}
