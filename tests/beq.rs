use emu6502::*;

#[test]
fn beq_relative() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LDA_IMMEDIATE;
    processor.memory[0xFFFD] = 0x00; // sets the zero flag to true
    processor.memory[0xFFFE] = BEQ_RELATIVE;
    processor.memory[0xFFFF] = 0xF0; // -16

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x00);
    assert_eq!(processor.registers.pc, 0xFFF0); // 0xFFFF + -16
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.z, true);
    assert_eq!(processor.registers.n, false);
}
