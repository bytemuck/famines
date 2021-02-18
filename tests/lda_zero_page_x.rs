#[cfg(test)]
mod tests {
    use emu6502::{Processor, Memory, LDA_ZERO_PAGE_X};

    #[cfg(test)]
    #[test]
    fn lda_zero_page() {
        let mut processor = Processor::new();
        let mut memory = Memory::new();

        processor.reset(&mut memory);
        processor.x = 0xFF;

        memory[0xFFFC] = LDA_ZERO_PAGE_X;
        memory[0xFFFD] = 0x80;
        memory[0x007F] = 0x37;

        let processor_copy = processor;
        let used_cycles = processor.execute(4, &mut memory);

        assert_eq!(processor.a, 0x37);
        assert_eq!(used_cycles, 4);

        assert_eq!(processor.z, false as u8);
        assert_eq!(processor.n, false as u8);

        assert_eq!(processor.c, processor_copy.c);
        assert_eq!(processor.i, processor_copy.i);
        assert_eq!(processor.d, processor_copy.d);
        assert_eq!(processor.b, processor_copy.b);
        assert_eq!(processor.v, processor_copy.v);
    }
}