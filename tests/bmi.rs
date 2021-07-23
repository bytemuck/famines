use emu6502::*;

#[test]
fn bmi_relative() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = LDA_IMMEDIATE;
    processor.memory[0xFFFD] = 0b1000_0000; // sets the negative flag to true
    processor.memory[0xFFFE] = BMI_RELATIVE;
    processor.memory[0xFFFF] = 0xF0; // -16

    let expected_cycles = 6;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0b1000_0000);
    assert_eq!(processor.registers.pc, Address(0xFFF0)); // 0xFFFF + -16
    assert_eq!(used_cycles, expected_cycles);

    assert_eq!(processor.registers.get_zero(), false);
    assert_eq!(processor.registers.get_negative(), true);
}
