/// Arithmetic module defines functions for 8-bit arithmetic.
/// It appropriately changes the Flag Register for each operation type.
///
/// ## The Flag Register (F) Bits
///
/// 7   6   5   4   3   2   1   0
/// Z   N   H  C Y  0   0   0   0

/// 0   0   0   1   -> 0x1
/// 0   0   1   0   -> 0x2
/// 0   1   0   0   -> 0x4
/// 1   0   0   0   -> 0x8

use super::cpu::CPU;

pub fn increment(cpu: &mut CPU, register_data: u8) -> u8 {
    let d8 = register_data.wrapping_add(1);
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    // let subtract_flag = 0x00; -> this is implied, so we're not adding this; reset
    let half_carry_flag = if (register_data & 0xF) + 1 > 0xF { 0x20 } else { 0x00 };
    let carry_flag = cpu.registers.r_f & 0x10; // we're maitaining its value
    cpu.registers.r_f = (zero_flag | half_carry_flag | carry_flag) as u8;
    d8
}

pub fn decrement(cpu: &mut CPU, register_data: u8) -> u8 {
    let d8 = register_data.wrapping_sub(1);
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    let subtract_flag = 0x40;
    let half_carry_flag = if ((register_data & 0xF) as i16) - 1 < 0 { 0x20 } else { 0x00 };
    let carry_flag = cpu.registers.r_f & 0x10; // we're maitaining its value
    cpu.registers.r_f = (zero_flag | subtract_flag | half_carry_flag | carry_flag) as u8;
    d8
}

pub fn add(cpu: &mut CPU, data: u8) {
    let register_data = cpu.registers.r_a;
    let d8 = register_data.wrapping_add(data);
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    // let subtract_flag = 0x00; -> this is implied, so we're not adding this; reset
    let half_carry_flag = if (register_data & 0xF) + (data & 0xF) > 0xF { 0x20 } else { 0x00 };
    let carry_flag = if (register_data as i16) + (data as i16) > 0xFF { 0x10 } else { 0x00 };
    cpu.registers.r_f = (zero_flag | half_carry_flag | carry_flag) as u8;
    cpu.registers.r_a = d8;
}

pub fn add_carry(cpu: &mut CPU, data: u8) {
    let carry = (cpu.registers.r_f & 0x10) as u8;
    let register_data = cpu.registers.r_a;
    let d8 = register_data.wrapping_add(data);
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    // let subtract_flag = 0x00; -> this is implied, so we're not adding this; reset
    let half_carry_flag = if (register_data & 0xF) + (data & 0xF) + (carry & 0xF) > 0xF { 0x20 } else { 0x00 };
    let carry_flag = if (register_data as i16) + (data as i16) + (carry as i16) > 0xFF { 0x10 } else { 0x00 };
    cpu.registers.r_f = (zero_flag | half_carry_flag | carry_flag) as u8;
    cpu.registers.r_a = d8;
}

pub fn sub(cpu: &mut CPU, data: u8) {
    let register_data = cpu.registers.r_a;
    let d8 = register_data.wrapping_sub(data);
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    let subtract_flag = 0x40;
    let half_carry_flag = if ((register_data & 0xF) as i16) - ((data & 0xF) as i16) < 0 { 0x20 } else { 0x00 };
    let carry_flag = if (register_data as i16) - (data as i16) < 0 { 0x10 } else { 0x00 };
    cpu.registers.r_f = (zero_flag | subtract_flag | half_carry_flag | carry_flag) as u8;
    cpu.registers.r_a = d8;
}

pub fn sub_carry(cpu: &mut CPU, data: u8) {
    let carry = (cpu.registers.r_f & 0x10) as u8;
    let register_data = cpu.registers.r_a;
    let d8 = register_data.wrapping_sub(data);
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    let subtract_flag = 0x40;
    let half_carry_flag = if ((register_data & 0xF) as i16) - ((data & 0xF) as i16) - (carry as i16) < 0 { 0x20 } else { 0x00 };
    let carry_flag = if (register_data as i16) - (data as i16) - (carry as i16) < 0 { 0x10 } else { 0x00 };
    cpu.registers.r_f = (zero_flag | subtract_flag | half_carry_flag | carry_flag) as u8;
    cpu.registers.r_a = d8;
}

pub fn and(cpu: &mut CPU, data: u8) {
    let register_data = cpu.registers.r_a;
    let d8 = register_data & data;
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    // let subtract_flag = 0x00; -> this is implied, so we're not adding this; reset
    let half_carry_flag = 0x20; // -> set
    // let carry_flag = 0x00; -> this is implied, so we're not adding this; reset
    cpu.registers.r_f = (zero_flag | half_carry_flag) as u8;
    cpu.registers.r_a = d8;
}

pub fn or(cpu: &mut CPU, data: u8) {
    let register_data = cpu.registers.r_a;
    let d8 = register_data | data;
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    // let subtract_flag = 0x00; -> this is implied, so we're not adding this; reset
    // let half_carry_flag = 0x00; -> this is implied, so we're not adding this; reset
    // let carry_flag = 0x00; -> this is implied, so we're not adding this; reset
    cpu.registers.r_f = zero_flag as u8;
    cpu.registers.r_a = d8;
}

pub fn xor(cpu: &mut CPU, data: u8) {
    let register_data = cpu.registers.r_a;
    let d8 = register_data ^ data;
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    // let subtract_flag = 0x00; -> this is implied, so we're not adding this; reset
    // let half_carry_flag = 0x00; -> this is implied, so we're not adding this; reset
    // let carry_flag = 0x00; -> this is implied, so we're not adding this; reset
    cpu.registers.r_f = zero_flag as u8;
    cpu.registers.r_a = d8;
}

pub fn compare(cpu: &mut CPU, data: u8) {
    let register_data = cpu.registers.r_a;
    let d8 = register_data.wrapping_sub(data);
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    let subtract_flag = 0x40;
    let half_carry_flag = if ((register_data & 0xF) as i16) - ((data & 0xF) as i16) < 0 { 0x20 } else { 0x00 };
    let carry_flag = if (register_data as i16) - (data as i16) < 0 { 0x10 } else { 0x00 };
    cpu.registers.r_f = (zero_flag | subtract_flag | half_carry_flag | carry_flag) as u8;
}
