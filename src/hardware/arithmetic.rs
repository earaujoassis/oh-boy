/// Arithmetic module defines functions for 8-bit arithmetic.
/// It appropriately changes the Flag Register for each operation type.

use super::cpu::CPU;

pub fn increment(cpu: &mut CPU, register: u8) -> u8 {
    let d8 = register.wrapping_add(1);
    let zero_flag = if d8 == 0 { 0x80 } else { 0x00 };
    // let subtract_flag = 0x0000; -> this is implied, so we're not adding this
    let half_carry_flag = if (register & 0xF) + 1 > 0xF { 0x20 } else { 0x00 };
    let carry_flag = cpu.registers.r_f & 0x10; // we're maitaining its value
    cpu.registers.r_f = (zero_flag | half_carry_flag | carry_flag) as u8;
    d8
}
