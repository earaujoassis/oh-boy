use super::flags;

/// Swap nibbles. The nibble size could be either 8bits (full word size is 16bits) or
/// 4bits (full word size is 8bits)
pub fn swap_nibbles(op: u16, nibble_size: u8) -> u16 {
    match nibble_size {
        8 => (((op & 0x00FF) << nibble_size | (op & 0xFF00) >> nibble_size)) & 0xFFFF,
        4 => (((op & 0x0F)   << nibble_size | (op & 0xF0)   >> nibble_size)) & 0x00FF,
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

pub fn rotate_left(op: u8) -> (u8, u8) {
    let b7 = op >> 7;
    let d8 = ((op << 1) | (op >> 7)) & 0xFF;
    let zero_flag = if d8 == 0x00 { flags::ZERO } else { flags::RESET };
    // let subtract_flag = flags::RESET; -> this is implied; reset
    // let half_carry_flag = flags::RESET; -> this is implied; reset
    let carry_flag = if b7 == 1 { flags::CARRY } else { flags::RESET };
    let flags = (zero_flag | carry_flag) as u8;
    (d8, flags)
}

pub fn rotate_left_carry(op: u8, register_flags: u8) -> (u8, u8) {
    let b7 = op >> 7;
    let d8 = ((op << 1) | ((register_flags & flags::CARRY) as u8)) & 0xFF;
    let zero_flag = if d8 == 0x00 { flags::ZERO } else { flags::RESET };
    // let subtract_flag = flags::RESET; -> this is implied; reset
    // let half_carry_flag = flags::RESET; -> this is implied; reset
    let carry_flag = if b7 == 1 { flags::CARRY } else { flags::RESET };
    let flags = (zero_flag | carry_flag) as u8;
    (d8, flags)
}

pub fn rotate_right(op: u8) -> (u8, u8) {
    let b0 = op & 0x01;
    let d8 = (op >> 1) & 0xFF;
    let zero_flag = if d8 == 0x00 { flags::ZERO } else { flags::RESET };
    // let subtract_flag = flags::RESET; -> this is implied; reset
    // let half_carry_flag = flags::RESET; -> this is implied; reset
    let carry_flag = if b0 == 1 { flags::CARRY } else { flags::RESET };
    let flags = (zero_flag | carry_flag) as u8;
    (d8, flags)
}

pub fn rotate_right_carry(op: u8, register_flags: u8) -> (u8, u8) {
    let b0 = op & 0x01;
    let d8 = ((op >> 1) | (((register_flags & flags::CARRY) as u8) << 7)) & 0xFF;
    let zero_flag = if d8 == 0x00 { flags::ZERO } else { flags::RESET };
    // let subtract_flag = flags::RESET; -> this is implied; reset
    // let half_carry_flag = flags::RESET; -> this is implied; reset
    let carry_flag = if b0 == 1 { flags::CARRY } else { flags::RESET };
    let flags = (zero_flag | carry_flag) as u8;
    (d8, flags)
}

pub fn shift_left(op: u8) -> (u8, u8) {
    let b7 = op >> 7;
    let d8 = (op << 1) & 0xFF;
    let zero_flag = if d8 == 0x00 { flags::ZERO } else { flags::RESET };
    // let subtract_flag = flags::RESET; -> this is implied; reset
    // let half_carry_flag = flags::RESET; -> this is implied; reset
    let carry_flag = if b7 == 1 { flags::CARRY } else { flags::RESET };
    let flags = (zero_flag | carry_flag) as u8;
    (d8, flags)
}

pub fn shift_right_reset(op: u8) -> (u8, u8) {
    let d8 = (op >> 1) & 0xFF;
    let zero_flag = if d8 == 0x00 { flags::ZERO } else { flags::RESET };
    // let subtract_flag = flags::RESET; -> this is implied; reset
    // let half_carry_flag = flags::RESET; -> this is implied; reset
    let carry_flag = if (op & 0x01) == 1 { flags::CARRY } else { flags::RESET };
    let flags = (zero_flag | carry_flag) as u8;
    (d8, flags)
}

pub fn shift_right(op: u8) -> (u8, u8) {
    let d8 = ((op >> 1) | (op & 0x80)) & 0xFF;
    let zero_flag = if d8 == 0x00 { flags::ZERO } else { flags::RESET };
    // let subtract_flag = flags::RESET; -> this is implied; reset
    // let half_carry_flag = flags::RESET; -> this is implied; reset
    let carry_flag = if (op & 0x01) == 1 { flags::CARRY } else { flags::RESET };
    let flags = (zero_flag | carry_flag) as u8;
    (d8, flags)
}

pub fn bit(op: u8, bit: usize) -> u8 {
    let is_set = match bit {
        0 => 0x01 & op > 0,
        1 => 0x02 & op > 0,
        2 => 0x04 & op > 0,
        3 => 0x08 & op > 0,
        4 => 0x10 & op > 0,
        5 => 0x20 & op > 0,
        6 => 0x40 & op > 0,
        7 => 0x80 & op > 0,
        _ => false
    };
    if is_set {
        return flags::HALF_CARRY as u8
    }
    (flags::ZERO | flags::HALF_CARRY) as u8
}

pub fn set(op: u8, bit: usize) -> u8 {
    match bit {
        0 => ((0x01 | op) & 0xFF) as u8,
        1 => ((0x02 | op) & 0xFF) as u8,
        2 => ((0x04 | op) & 0xFF) as u8,
        3 => ((0x08 | op) & 0xFF) as u8,
        4 => ((0x10 | op) & 0xFF) as u8,
        5 => ((0x20 | op) & 0xFF) as u8,
        6 => ((0x40 | op) & 0xFF) as u8,
        7 => ((0x80 | op) & 0xFF) as u8,
        _ => (op & 0xFF) as u8
    }
}

pub fn reset(op: u8, bit: usize) -> u8 {
    match bit {
        0 => ((!(0x01 as u8) & op) & 0xFF) as u8,
        1 => ((!(0x02 as u8) & op) & 0xFF) as u8,
        2 => ((!(0x04 as u8) & op) & 0xFF) as u8,
        3 => ((!(0x08 as u8) & op) & 0xFF) as u8,
        4 => ((!(0x10 as u8) & op) & 0xFF) as u8,
        5 => ((!(0x20 as u8) & op) & 0xFF) as u8,
        6 => ((!(0x40 as u8) & op) & 0xFF) as u8,
        7 => ((!(0x80 as u8) & op) & 0xFF) as u8,
        _ => (op & 0xFF) as u8
    }
}
