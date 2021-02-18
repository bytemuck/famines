#[cfg(test)]
mod tests {
    use emu6502::{Processor, Memory, LDA_ZERO_PAGE};

    #[test]
    fn lda_zero_page() {
        let mut processor = Processor::new();
        let mut memory = Memory::new();

        processor.reset(&mut memory);

        memory[0xFFFC] = LDA_ZERO_PAGE;
        memory[0xFFFD] = 0x42;
        memory[0x42] = 0x84;

        let processor_copy = processor;
        let used_cycles = processor.execute(3, &mut memory);

        assert_eq!(processor.a, 0x84);
        assert_eq!(used_cycles, 3);

        assert_eq!(processor.z, false as u8);
        assert_eq!(processor.n, true as u8);

        assert_eq!(processor.c, processor_copy.c);
        assert_eq!(processor.i, processor_copy.i);
        assert_eq!(processor.d, processor_copy.d);
        assert_eq!(processor.b, processor_copy.b);
        assert_eq!(processor.v, processor_copy.v);
    }
}