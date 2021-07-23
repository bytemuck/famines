use emu6502::*;

#[test]
fn jmp_absolute() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = JMP_ABSOLUTE; // 3
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = LDA_IMMEDIATE; // 2
    processor.memory[0x4243] = 0x84;

    let expected_cycles = 5;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x84);
    assert_eq!(used_cycles, expected_cycles);
}

#[test]
fn jmp_indirect() {
    let mut processor = Processor::new();

    processor.memory[0xFFFC] = JMP_INDIRECT; // 5
    processor.memory[0xFFFD] = 0x42;
    processor.memory[0xFFFE] = 0x42;
    processor.memory[0x4242] = 0x30;
    processor.memory[0x4243] = 0x30;
    processor.memory[0x3030] = LDA_IMMEDIATE; // 2
    processor.memory[0x3031] = 0x84;

    let expected_cycles = 7;
    let used_cycles = processor.execute_cycles(expected_cycles);

    assert_eq!(processor.registers.a, 0x84);
    assert_eq!(used_cycles, expected_cycles);
}
