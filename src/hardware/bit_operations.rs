/// Swap nibbles. The nibble size could be either 8bits (full word size is 16bits) or
/// 4bits (full word size is 8bits)
pub fn swap_nibbles(op: u16, nibble_size: u8) -> u16 {
    match nibble_size {
        8 => ((op & 0x00FF) << nibble_size | (op & 0xFF00) >> nibble_size),
        4 => ((op & 0x0F)   << nibble_size | (op & 0xF0)   >> nibble_size),
        _ => op
    }
}

pub fn join_words(op_a: u16, op_b: u16, nibble_size: u8) -> u16 {
    op_a << nibble_size | op_b
}

// Convert from one endianess to the other (revert nibbles)
pub fn endianess(lsb: u16, msb: u16, nibble_size: u8) -> u16 {
    swap_nibbles(join_words(lsb, msb, nibble_size), nibble_size)
}

pub fn msb(op: u16, nibble_size: u8) -> u8 {
    (op >> nibble_size) as u8
}

pub fn lsb(op: u16, _nibble_size: u8) -> u8 {
    (op & 0x00FF) as u8
}
