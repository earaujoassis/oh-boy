#[cfg(test)]
mod tests {
    extern crate gameboy_emulator;
    use tests::gameboy_emulator::hardware::bit_operations::*;

    #[test]
    fn test_swap_nibbles() {
        assert_eq!(swap_nibbles(0x0102, 8), 0x0201);
        assert_eq!(swap_nibbles(0xF102, 8), 0x02F1);
        assert_eq!(swap_nibbles(0xF188, 8), 0x88F1);
    }

    #[test]
    fn test_join_words() {
        assert_eq!(join_words(0xF1, 0x88, 8), 0xF188);
        assert_eq!(join_words(0x02, 0x01, 8), 0x0201);
        assert_eq!(join_words(0x88, 0xFF, 8), 0x88FF);
        assert_eq!(join_words(0x80, 0x01, 8), 0x8001);
    }

    #[test]
    fn test_endianess() {
        assert_eq!(endianess(0xF1, 0x88, 8), 0x88F1);
        assert_eq!(endianess(0x88, 0xF1, 8), 0xF188);
        assert_eq!(endianess(0x80, 0x01, 8), 0x0180);
        assert_eq!(endianess(0x01, 0x80, 8), 0x8001);
    }

    #[test]
    fn test_msb() {
        assert_eq!(msb(0xF102, 8), 0xF1);
        assert_eq!(msb(0xF188, 8), 0xF1);
        assert_eq!(msb(0x88F1, 8), 0x88);
        assert_eq!(msb(0x8001, 8), 0x80);
    }

    #[test]
    fn test_lsb() {
        assert_eq!(lsb(0xF102, 8), 0x02);
        assert_eq!(lsb(0xF188, 8), 0x88);
        assert_eq!(lsb(0x88F1, 8), 0xF1);
        assert_eq!(lsb(0x8001, 8), 0x01);
    }
}
