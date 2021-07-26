use emu6502::*;

#[test]
fn bvc_relative() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LDA_IMMEDIATE; // 2 // place holder, NOP isn't implemented yet.
    processor.memory[0xFFFD] = 0b1000_0010; // place holder, NOP isn't implemented yet.
    processor.memory[0xFFFE] = BVC_RELATIVE; // 4
    processor.memory[0xFFFF] = 0xF0; // -16

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);
    assert_eq!(processor.registers.pc, 0xFFF0); // 0xFFFF + -16
}
