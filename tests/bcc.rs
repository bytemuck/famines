use emu6502::*;

#[test]
fn bcc_relative() {
    let mut processor = Processor::new();

    processor.registers.c = false;

    processor.memory[0xFFFC] = BCC_RELATIVE;
    processor.memory[0xFFFD] = 0x1;

    let expected_cycles = 3;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);
    assert_eq!(processor.registers.pc, 0xFFFF);
}
