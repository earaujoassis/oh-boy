#[cfg(test)]
mod tests {
    extern crate gameboy_emulator;
    use tests::gameboy_emulator::hardware::bit_operations::*;

    // Most of the following tests came from:
    // the "GAME BOY Programming Manual Version 1.1" by NINTENDO, INC.

    #[test]
    fn test_swap_nibbles() {
        assert_eq!(swap_nibbles(0x01, 4), 0x10);
        assert_eq!(swap_nibbles(0xF2, 4), 0x2F);
        assert_eq!(swap_nibbles(0x81, 4), 0x18);
        assert_eq!(swap_nibbles(0x00, 4), 0x00);
        assert_eq!(swap_nibbles(0xF0, 4), 0x0F);
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

    #[test]
    fn test_rotate_left_carry() {
        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_left_carry(0x85, flags_entry);
            assert_eq!(r, 0x0B);
            assert_eq!(flags, 0x10); // Z is 0, CY is 1
        }

        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_left_carry(0x00, flags_entry);
            assert_eq!(r, 0x00);
            assert_eq!(flags, 0x80); // Z is 1, CY is 0
        }
    }

    #[test]
    fn test_rotate_left() {
        {
            let flags_entry = 0x10; // CY is 1
            let (r, flags) = rotate_left(0x95, flags_entry);
            assert_eq!(r, 0x2B);
            assert_eq!(flags, 0x10); // Z is 0, CY is 1
        }

        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_left(0x80, flags_entry);
            assert_eq!(r, 0x00);
            assert_eq!(flags, 0x90); // Z is 1, CY is 1
        }

        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_left(0x11, flags_entry);
            assert_eq!(r, 0x22);
            assert_eq!(flags, 0x00); // Z is 0, CY is 0
        }
    }

    #[test]
    fn test_rotate_right_carry() {
        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_right_carry(0x3B, flags_entry);
            assert_eq!(r, 0x9D);
            assert_eq!(flags, 0x10); // Z is 0, CY is 1
        }

        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_right_carry(0x01, flags_entry);
            assert_eq!(r, 0x80);
            assert_eq!(flags, 0x10); // Z is 0, CY is 1
        }

        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_right_carry(0x00, flags_entry);
            assert_eq!(r, 0x00);
            assert_eq!(flags, 0x80); // Z is 1, CY is 0
        }
    }

    #[test]
    fn test_rotate_right() {
        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_right(0x81, flags_entry);
            assert_eq!(r, 0x40);
            assert_eq!(flags, 0x10); // Z is 0, CY is 1
        }

        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_right(0x01, flags_entry);
            assert_eq!(r, 0x00);
            assert_eq!(flags, 0x90); // Z is 1, CY is 1
        }

        {
            let flags_entry = 0x00; // CY is 0
            let (r, flags) = rotate_right(0x8A, flags_entry);
            assert_eq!(r, 0x45);
            assert_eq!(flags, 0x00); // Z is 0, CY is 0
        }
    }

    #[test]
    fn test_shift_left() {
        {
            let (r, flags) = shift_left(0x80);
            assert_eq!(r, 0x00);
            assert_eq!(flags, 0x90); // Z is 1, CY is 1
        }

        {
            let (r, flags) = shift_left(0xFF);
            assert_eq!(r, 0xFE);
            assert_eq!(flags, 0x10); // Z is 0, CY is 1
        }
    }

    #[test]
    fn test_shift_right() {
        {
            let (r, flags) = shift_right(0x8A);
            assert_eq!(r, 0xC5);
            assert_eq!(flags, 0x00); // Z is 0, CY is 0
        }

        {
            let (r, flags) = shift_right(0x01);
            assert_eq!(r, 0x00);
            assert_eq!(flags, 0x90); // Z is 1, CY is 1
        }
    }

    #[test]
    fn test_shift_right_reset() {
        {
            let (r, flags) = shift_right_reset(0x01);
            assert_eq!(r, 0x00);
            assert_eq!(flags, 0x90); // Z is 1, CY is 1
        }

        {
            let (r, flags) = shift_right_reset(0xFF);
            assert_eq!(r, 0x7F);
            assert_eq!(flags, 0x10); // Z is 0, CY is 1
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_bit() {
        let ZSET   = 0xA0; // Z is 1, H is always 1
        let ZRESET = 0x20; // Z is 0, H is always 1
        assert_eq!(bit(0x80, 7), ZRESET);
        assert_eq!(bit(0xEF, 4), ZSET);
    }

    #[test]
    fn test_set() {
        assert_eq!(set(0x80, 3), 0x88);
        assert_eq!(set(0x10, 7), 0x90);
        assert_eq!(set(0x08, 3), 0x08);
    }

    #[test]
    fn test_reset() {
        assert_eq!(reset(0x80, 7), 0x00);
        assert_eq!(reset(0x08, 3), 0x00);
        assert_eq!(reset(0x10, 3), 0x10);
    }
}
