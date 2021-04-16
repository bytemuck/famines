use emu6502::*;

#[test]
fn bvs_relative() {
    let mut processor = Processor::new();

    processor.registers.set_overflow(true);

    processor.memory[0xFFFC] = LDA_IMMEDIATE; // 2 // place holder, NOP isn't implemented yet.
    processor.memory[0xFFFD] = 0b1000_0010; // place holder, NOP isn't implemented yet.
    processor.memory[0xFFFE] = BVS_RELATIVE; // 4
    processor.memory[0xFFFF] = 0xF0; // -16

    let expected_cycles = 6;
    let used_cycles = processor.execute(expected_cycles);

    assert_eq!(used_cycles, expected_cycles);
    assert_eq!(processor.registers.pc, Address(0xFFF0)); // 0xFFFF + -16

    assert_eq!(processor.registers.get_overflow(), true);
}
