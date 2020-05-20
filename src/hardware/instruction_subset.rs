use super::cpu::CPU;
use super::memory::Memory;
use super::flags;
use super::bit_operations;

/// This function represents the instruction subset executor within the 0xCB prefix.
#[allow(unreachable_patterns)]
pub fn execute(cpu: &mut CPU, memory: &mut Memory, opcode: u8) -> usize {
    match opcode {
        /* RLC B */ 0x00 => {
            let (register_data, register_flags) = bit_operations::rotate_left_carry(cpu.registers.r_b, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_b = register_data;
            2
        },
        /* RLC C */ 0x01 => {
            let (register_data, register_flags) = bit_operations::rotate_left_carry(cpu.registers.r_c, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_c = register_data;
            2
        },
        /* RLC D */ 0x02 => {
            let (register_data, register_flags) = bit_operations::rotate_left_carry(cpu.registers.r_d, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_d = register_data;
            2
        },
        /* RLC E */ 0x03 => {
            let (register_data, register_flags) = bit_operations::rotate_left_carry(cpu.registers.r_e, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_e = register_data;
            2
        },
        /* RLC H */ 0x04 => {
            let (register_data, register_flags) = bit_operations::rotate_left_carry(cpu.registers.r_h, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_h = register_data;
            2
        },
        /* RLC L */ 0x05 => {
            let (register_data, register_flags) = bit_operations::rotate_left_carry(cpu.registers.r_l, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_l = register_data;
            2
        },
        /* RLC (HL) */ 0x06 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let (n8, register_flags) = bit_operations::rotate_left_carry(d8, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.write_data(memory, a16_hl, n8);
            4
        },
        /* RLC A */ 0x07 => {
            let (register_data, register_flags) = bit_operations::rotate_left_carry(cpu.registers.r_a, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_a = register_data;
            2
        },
        /* RRC B */ 0x08 => {
            let (register_data, register_flags) = bit_operations::rotate_right_carry(cpu.registers.r_b, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_b = register_data;
            2
        },
        /* RRC C */ 0x09 => {
            let (register_data, register_flags) = bit_operations::rotate_right_carry(cpu.registers.r_c, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_c = register_data;
            2
        },
        /* RRC D */ 0x0A => {
            let (register_data, register_flags) = bit_operations::rotate_right_carry(cpu.registers.r_d, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_d = register_data;
            2
        },
        /* RRC E */ 0x0B => {
            let (register_data, register_flags) = bit_operations::rotate_right_carry(cpu.registers.r_e, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_e = register_data;
            2
        },
        /* RRC H */ 0x0C => {
            let (register_data, register_flags) = bit_operations::rotate_right_carry(cpu.registers.r_h, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_h = register_data;
            2
        },
        /* RRC L */ 0x0D => {
            let (register_data, register_flags) = bit_operations::rotate_right_carry(cpu.registers.r_l, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_l = register_data;
            2
        },
        /* RRC (HL) */ 0x0E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let (n8, register_flags) = bit_operations::rotate_right_carry(d8, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.write_data(memory, a16_hl, n8);
            4
        },
        /* RRC A */ 0x0F => {
            let (register_data, register_flags) = bit_operations::rotate_right_carry(cpu.registers.r_a, cpu.registers.r_f);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_a = register_data;
            2
        },
        /* RL B */ 0x10 => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_b);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_b = register_data;
            2
        },
        /* RL C */ 0x11 => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_c);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_c = register_data;
            2
        },
        /* RL D */ 0x12 => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_d);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_d = register_data;
            2
        },
        /* RL E */ 0x13 => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_e);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_e = register_data;
            2
        },
        /* RL H */ 0x14 => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_h);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_h = register_data;
            2
        },
        /* RL L */ 0x15 => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_l);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_l = register_data;
            2
        },
        /* RL (HL) */ 0x16 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let (n8, register_flags) = bit_operations::rotate_left(d8);
            cpu.registers.r_f = register_flags as u8;
            cpu.write_data(memory, a16_hl, n8);
            4
        },
        /* RL A */ 0x17 => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_a);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_a = register_data;
            2
        },
        /* RR B */ 0x18 => {
            let (register_data, register_flags) = bit_operations::rotate_right(cpu.registers.r_b);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_b = register_data;
            2
        },
        /* RR C */ 0x19 => {
            let (register_data, register_flags) = bit_operations::rotate_right(cpu.registers.r_c);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_c = register_data;
            2
        },
        /* RR D */ 0x1A => {
            let (register_data, register_flags) = bit_operations::rotate_right(cpu.registers.r_d);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_d = register_data;
            2
        },
        /* RR E */ 0x1B => {
            let (register_data, register_flags) = bit_operations::rotate_right(cpu.registers.r_e);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_e = register_data;
            2
        },
        /* RR H */ 0x1C => {
            let (register_data, register_flags) = bit_operations::rotate_right(cpu.registers.r_h);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_h = register_data;
            2
        },
        /* RR L */ 0x1D => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_l);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_l = register_data;
            2
        },
        /* RR (HL) */ 0x1E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let (n8, register_flags) = bit_operations::rotate_right(d8);
            cpu.registers.r_f = register_flags as u8;
            cpu.write_data(memory, a16_hl, n8);
            4
        },
        /* RR A */ 0x1F => {
            let (register_data, register_flags) = bit_operations::rotate_right(cpu.registers.r_a);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_a = register_data;
            2
        },
        /* SLA B */ 0x20 => {
            let (register_data, register_flags) = bit_operations::shift_left(cpu.registers.r_b);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_b = register_data;
            2
        },
        /* SLA C */ 0x21 => {
            let (register_data, register_flags) = bit_operations::shift_left(cpu.registers.r_c);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_c = register_data;
            2
        },
        /* SLA D */ 0x22 => {
            let (register_data, register_flags) = bit_operations::shift_left(cpu.registers.r_d);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_d = register_data;
            2
        },
        /* SLA E */ 0x23 => {
            let (register_data, register_flags) = bit_operations::shift_left(cpu.registers.r_e);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_e = register_data;
            2
        },
        /* SLA H */ 0x24 => {
            let (register_data, register_flags) = bit_operations::shift_left(cpu.registers.r_h);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_h = register_data;
            2
        },
        /* SLA L */ 0x25 => {
            let (register_data, register_flags) = bit_operations::shift_left(cpu.registers.r_l);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_l = register_data;
            2
        },
        /* SLA (HL) */ 0x26 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let (n8, register_flags) = bit_operations::shift_left(d8);
            cpu.registers.r_f = register_flags as u8;
            cpu.write_data(memory, a16_hl, n8);
            4
        },
        /* SLA A */ 0x27 => {
            let (register_data, register_flags) = bit_operations::shift_left(cpu.registers.r_a);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_a = register_data;
            2
        },
        /* SRA B */ 0x28 => {
            let (register_data, register_flags) = bit_operations::shift_right(cpu.registers.r_b);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_b = register_data;
            2
        },
        /* SRA C */ 0x29 => {
            let (register_data, register_flags) = bit_operations::shift_right(cpu.registers.r_c);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_c = register_data;
            2
        },
        /* SRA D */ 0x2A => {
            let (register_data, register_flags) = bit_operations::shift_right(cpu.registers.r_d);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_d = register_data;
            2
        },
        /* SRA E */ 0x2B => {
            let (register_data, register_flags) = bit_operations::shift_right(cpu.registers.r_e);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_e = register_data;
            2
        },
        /* SRA H */ 0x2C => {
            let (register_data, register_flags) = bit_operations::shift_right(cpu.registers.r_h);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_h = register_data;
            2
        },
        /* SRA L */ 0x2D => {
            let (register_data, register_flags) = bit_operations::shift_right(cpu.registers.r_l);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_l = register_data;
            2
        },
        /* SRA (HL) */ 0x2E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let (n8, register_flags) = bit_operations::shift_right(d8);
            cpu.registers.r_f = register_flags as u8;
            cpu.write_data(memory, a16_hl, n8);
            4
        },
        /* SRA A */ 0x2F => {
            let (register_data, register_flags) = bit_operations::shift_right(cpu.registers.r_a);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_a = register_data;
            2
        },
        /* SWAP B */ 0x30 => {
            let register_data = bit_operations::swap_nibbles(cpu.registers.r_b as u16, 4) as u8;
            cpu.registers.r_f = if register_data == 0 { flags::ZERO } else { flags::RESET };
            cpu.registers.r_b = register_data;
            2
        },
        /* SWAP C */ 0x31 => {
            let register_data = bit_operations::swap_nibbles(cpu.registers.r_c as u16, 4) as u8;
            cpu.registers.r_f = if register_data == 0 { flags::ZERO } else { flags::RESET };
            cpu.registers.r_c = register_data;
            2
        },
        /* SWAP D */ 0x32 => {
            let register_data = bit_operations::swap_nibbles(cpu.registers.r_d as u16, 4) as u8;
            cpu.registers.r_f = if register_data == 0 { flags::ZERO } else { flags::RESET };
            cpu.registers.r_d = register_data;
            2
        },
        /* SWAP E */ 0x33 => {
            let register_data = bit_operations::swap_nibbles(cpu.registers.r_e as u16, 4) as u8;
            cpu.registers.r_f = if register_data == 0 { flags::ZERO } else { flags::RESET };
            cpu.registers.r_e = register_data;
            2
        },
        /* SWAP H */ 0x34 => {
            let register_data = bit_operations::swap_nibbles(cpu.registers.r_h as u16, 4) as u8;
            cpu.registers.r_f = if register_data == 0 { flags::ZERO } else { flags::RESET };
            cpu.registers.r_h = register_data;
            2
        },
        /* SWAP L */ 0x35 => {
            let register_data = bit_operations::swap_nibbles(cpu.registers.r_l as u16, 4) as u8;
            cpu.registers.r_f = if register_data == 0 { flags::ZERO } else { flags::RESET };
            cpu.registers.r_l = register_data;
            2
        },
        /* SWAP (HL) */ 0x36 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let n8 = bit_operations::swap_nibbles(d8 as u16, 4) as u8;
            cpu.registers.r_f = if n8 == 0 { flags::ZERO } else { flags::RESET };
            cpu.write_data(memory, a16_hl, n8);
            4
        },
        /* SWAP A */ 0x37 => {
            let register_data = bit_operations::swap_nibbles(cpu.registers.r_a as u16, 4) as u8;
            cpu.registers.r_f = if register_data == 0 { flags::ZERO } else { flags::RESET };
            cpu.registers.r_a = register_data;
            2
        },
        /* SRL B */ 0x38 => {
            let (register_data, register_flags) = bit_operations::shift_right_reset(cpu.registers.r_b);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_b = register_data;
            2
        },
        /* SRL C */ 0x39 => {
            let (register_data, register_flags) = bit_operations::shift_right_reset(cpu.registers.r_c);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_c = register_data;
            2
        },
        /* SRL D */ 0x3A => {
            let (register_data, register_flags) = bit_operations::shift_right_reset(cpu.registers.r_d);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_d = register_data;
            2
        },
        /* SRL E */ 0x3B => {
            let (register_data, register_flags) = bit_operations::shift_right_reset(cpu.registers.r_e);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_e = register_data;
            2
        },
        /* SRL H */ 0x3C => {
            let (register_data, register_flags) = bit_operations::shift_right_reset(cpu.registers.r_h);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_h = register_data;
            2
        },
        /* SRL L */ 0x3D => {
            let (register_data, register_flags) = bit_operations::shift_right_reset(cpu.registers.r_l);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_l = register_data;
            2
        },
        /* SRL (HL) */ 0x3E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let (n8, register_flags) = bit_operations::shift_right_reset(d8);
            cpu.registers.r_f = register_flags as u8;
            cpu.write_data(memory, a16_hl, n8);
            4
        },
        /* SRL A */ 0x3F => {
            let (register_data, register_flags) = bit_operations::shift_right_reset(cpu.registers.r_a);
            cpu.registers.r_f = register_flags as u8;
            cpu.registers.r_a = register_data;
            2
        },
        /* BIT b,r/(HL) */ 0x40..=0x7F => {
            let mut cycle = 2;
            let bit: usize = match (opcode >> 3) & 0xFF {
                0x08 => 0,
                0x09 => 1,
                0x0A => 2,
                0x0B => 3,
                0x0C => 4,
                0x0D => 5,
                0x0E => 6,
                0x0F => 7,
                _ => panic!("Unrecognized bit position for the (0xCB) BIT opcode")
            };
            let d8: u8 = match (opcode << 5) & 0xFF {
                0xE0 => cpu.registers.r_a,
                0x00 => cpu.registers.r_b,
                0x20 => cpu.registers.r_c,
                0x40 => cpu.registers.r_d,
                0x60 => cpu.registers.r_e,
                0x80 => cpu.registers.r_h,
                0xA0 => cpu.registers.r_l,
                0xC0 => {
                    cycle = 4;
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    cpu.fetch_data(memory, a16_hl)
                },
                _ => panic!("Unrecognized bit position for the (0xCB) BIT opcode")
            };
            let flags = bit_operations::bit(d8, bit);
            cpu.registers.r_f = flags | (cpu.registers.r_f & flags::CARRY) as u8;
            cycle
        },
        /* RES b,r/(HL) */ 0x80..=0xBF => {
            let mut cycle = 2;
            let bit: usize = match (opcode >> 3) & 0xFF {
                0x10 => 0,
                0x11 => 1,
                0x12 => 2,
                0x13 => 3,
                0x14 => 4,
                0x15 => 5,
                0x16 => 6,
                0x17 => 7,
                _ => panic!("Unrecognized bit position for the (0xCB) RES opcode")
            };
            match (opcode << 5) & 0xFF {
                0xE0 => cpu.registers.r_a = bit_operations::reset(cpu.registers.r_a, bit),
                0x00 => cpu.registers.r_b = bit_operations::reset(cpu.registers.r_b, bit),
                0x20 => cpu.registers.r_c = bit_operations::reset(cpu.registers.r_c, bit),
                0x40 => cpu.registers.r_d = bit_operations::reset(cpu.registers.r_d, bit),
                0x60 => cpu.registers.r_e = bit_operations::reset(cpu.registers.r_e, bit),
                0x80 => cpu.registers.r_h = bit_operations::reset(cpu.registers.r_h, bit),
                0xA0 => cpu.registers.r_l = bit_operations::reset(cpu.registers.r_l, bit),
                0xC0 => {
                    cycle = 4;
                    let a16_hl: u16 = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8: u8 = cpu.fetch_data(memory, a16_hl);
                    let n8: u8 = bit_operations::reset(d8, bit);
                    cpu.write_data(memory, a16_hl, n8);
                },
                _ => panic!("Unrecognized bit position for the (0xCB) RES opcode")
            };
            cycle
        },
        /* SET b,r/(HL) */ 0xC0..=0xFF => {
            let mut cycle = 2;
            let bit: usize = match (opcode >> 3) & 0xFF {
                0x18 => 0,
                0x19 => 1,
                0x1A => 2,
                0x1B => 3,
                0x1C => 4,
                0x1D => 5,
                0x1E => 6,
                0x1F => 7,
                _ => panic!("Unrecognized bit position for the (0xCB) SET opcode")
            };
            match (opcode << 5) & 0xFF {
                0xE0 => cpu.registers.r_a = bit_operations::set(cpu.registers.r_a, bit),
                0x00 => cpu.registers.r_b = bit_operations::set(cpu.registers.r_b, bit),
                0x20 => cpu.registers.r_c = bit_operations::set(cpu.registers.r_c, bit),
                0x40 => cpu.registers.r_d = bit_operations::set(cpu.registers.r_d, bit),
                0x60 => cpu.registers.r_e = bit_operations::set(cpu.registers.r_e, bit),
                0x80 => cpu.registers.r_h = bit_operations::set(cpu.registers.r_h, bit),
                0xA0 => cpu.registers.r_l = bit_operations::set(cpu.registers.r_l, bit),
                0xC0 => {
                    cycle = 4;
                    let a16_hl: u16 = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8: u8 = cpu.fetch_data(memory, a16_hl);
                    let n8: u8 = bit_operations::set(d8, bit);
                    cpu.write_data(memory, a16_hl, n8);
                },
                _ => panic!("Unrecognized bit position for the (0xCB) SET opcode")
            };
            cycle
        },
        _ => panic!("Opcode unknown within prefix 0xCB: ${:02X}", opcode)
    }
}
