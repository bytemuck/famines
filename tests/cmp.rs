use emu6502::*;

#[test]
fn cmp_immediate() {
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x45;

        processor.memory[0xFFFC] = CMP_IMMEDIATE;
        processor.memory[0xFFFD] = 0x42;

        let expected_cycles = 2;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x42;

        processor.memory[0xFFFC] = CMP_IMMEDIATE;
        processor.memory[0xFFFD] = 0x42;

        let expected_cycles = 2;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cmp_zero_page() {
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x68;

        processor.memory[0xFFFC] = CMP_ZERO_PAGE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0x0042] = 0x67;

        let expected_cycles = 3;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x67;

        processor.memory[0xFFFC] = CMP_ZERO_PAGE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0x0042] = 0x67;

        let expected_cycles = 3;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cmp_zero_page_x() {
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x10;
        processor.registers.a = 0x68;

        processor.memory[0xFFFC] = CMP_ZERO_PAGE_X;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0x0052] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x10;
        processor.registers.a = 0x67;

        processor.memory[0xFFFC] = CMP_ZERO_PAGE_X;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0x0052] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cmp_absolute() {
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x68;

        processor.memory[0xFFFC] = CMP_ABSOLUTE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4242] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x67;

        processor.memory[0xFFFC] = CMP_ABSOLUTE;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4242] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cmp_absolute_x() {
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x10;
        processor.registers.a = 0x68;

        processor.memory[0xFFFC] = CMP_ABSOLUTE_X;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4252] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.x = 0x10;
        processor.registers.a = 0x67;

        processor.memory[0xFFFC] = CMP_ABSOLUTE_X;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4252] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cmp_absolute_y() {
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x10;
        processor.registers.a = 0x68;

        processor.memory[0xFFFC] = CMP_ABSOLUTE_Y;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4252] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.y = 0x10;
        processor.registers.a = 0x67;

        processor.memory[0xFFFC] = CMP_ABSOLUTE_Y;
        processor.memory[0xFFFD] = 0x42;
        processor.memory[0xFFFE] = 0x42;
        processor.memory[0x4252] = 0x67;

        let expected_cycles = 4;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cmp_indirect_x() {
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x68;
        processor.registers.x = 0x04;

        processor.memory[0xFFFC] = CMP_INDIRECT_X;
        processor.memory[0xFFFD] = 0x02;
        processor.memory[0x0006] = 0x42;
        processor.memory[0x0007] = 0x42;
        processor.memory[0x4242] = 0x67;

        let expected_cycles = 6;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x67;
        processor.registers.x = 0x04;

        processor.memory[0xFFFC] = CMP_INDIRECT_X;
        processor.memory[0xFFFD] = 0x02;
        processor.memory[0x0006] = 0x42;
        processor.memory[0x0007] = 0x42;
        processor.memory[0x4242] = 0x67;

        let expected_cycles = 6;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cmp_indirect_y() {
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x68;
        processor.registers.y = 0x04;

        processor.memory[0xFFFC] = CMP_INDIRECT_Y;
        processor.memory[0xFFFD] = 0x02;
        processor.memory[0x0002] = 0x00;
        processor.memory[0x0003] = 0x80;
        processor.memory[0x8004] = 0x67; // 0x8000 + 0x04

        let expected_cycles = 5;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x67;
        processor.registers.y = 0x04;

        processor.memory[0xFFFC] = CMP_INDIRECT_Y;
        processor.memory[0xFFFD] = 0x02;
        processor.memory[0x0002] = 0x00;
        processor.memory[0x0003] = 0x80;
        processor.memory[0x8004] = 0x67; // 0x8000 + 0x04

        let expected_cycles = 5;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}

#[test]
fn cmp_indirect_y_crosses() {
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x68;
        processor.registers.y = 0xFF;

        processor.memory[0xFFFC] = CMP_INDIRECT_Y;
        processor.memory[0xFFFD] = 0x02;
        processor.memory[0x0002] = 0x02;
        processor.memory[0x0003] = 0x80;
        processor.memory[0x8101] = 0x67; // 0x8002 + 0xFF

        let expected_cycles = 6;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), false);
    }
    {
        let mut processor = Processor::new();

        processor.registers.a = 0x67;
        processor.registers.y = 0xFF;

        processor.memory[0xFFFC] = CMP_INDIRECT_Y;
        processor.memory[0xFFFD] = 0x02;
        processor.memory[0x0002] = 0x02;
        processor.memory[0x0003] = 0x80;
        processor.memory[0x8101] = 0x67; // 0x8002 + 0xFF

        let expected_cycles = 6;
        let used_cycles = processor.execute(expected_cycles);

        assert_eq!(used_cycles, expected_cycles);

        assert_eq!(processor.registers.get_negative(), false);
        assert_eq!(processor.registers.get_carry(), true);
        assert_eq!(processor.registers.get_zero(), true);
    }
}
