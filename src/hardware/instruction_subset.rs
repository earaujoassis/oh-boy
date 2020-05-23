use super::cpu::CPU;
use super::memory::Memory;
use super::flags;
use super::bit_operations;

/// This function represents the instruction subset executor within the 0xCB prefix.
#[allow(unreachable_patterns)]
pub fn execute(cpu: &mut CPU, memory: &mut Memory, opcode: u8) -> usize {
    match opcode {
        /* RLC r/(HL) */ 0x00..=0x07 => {
            let mut cycles = 2;
            let action_pair;

            match opcode & 0x07 {
                0x07 => {
                    action_pair = bit_operations::rotate_left_carry(cpu.registers.r_a, cpu.registers.r_f);
                    cpu.registers.r_a = action_pair.0;
                },
                0x00 => {
                    action_pair = bit_operations::rotate_left_carry(cpu.registers.r_b, cpu.registers.r_f);
                    cpu.registers.r_b = action_pair.0;
                },
                0x01 => {
                    action_pair = bit_operations::rotate_left_carry(cpu.registers.r_c, cpu.registers.r_f);
                    cpu.registers.r_c = action_pair.0;
                },
                0x02 => {
                    action_pair = bit_operations::rotate_left_carry(cpu.registers.r_d, cpu.registers.r_f);
                    cpu.registers.r_d = action_pair.0;
                },
                0x03 => {
                    action_pair = bit_operations::rotate_left_carry(cpu.registers.r_e, cpu.registers.r_f);
                    cpu.registers.r_e = action_pair.0;
                },
                0x04 => {
                    action_pair = bit_operations::rotate_left_carry(cpu.registers.r_h, cpu.registers.r_f);
                    cpu.registers.r_h = action_pair.0;
                },
                0x05 => {
                    action_pair = bit_operations::rotate_left_carry(cpu.registers.r_l, cpu.registers.r_f);
                    cpu.registers.r_l = action_pair.0;
                },
                0x06 => {
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8 = cpu.fetch_data(memory, a16_hl);
                    action_pair = bit_operations::rotate_left_carry(d8, cpu.registers.r_f);
                    cpu.write_data(memory, a16_hl, action_pair.0);
                    cycles = 4;
                },
                _ => panic!("There's an error at RLC"),
            };
            cpu.registers.r_f = action_pair.1 as u8;
            cycles
        },
        /* RRC r/(HL) */ 0x08..=0x0F => {
            let mut cycles = 2;
            let action_pair;

            match opcode & 0x07 {
                /* A */ 0x07 => {
                    action_pair = bit_operations::rotate_right_carry(cpu.registers.r_a, cpu.registers.r_f);
                    cpu.registers.r_a = action_pair.0;
                },
                /* B */ 0x00 => {
                    action_pair = bit_operations::rotate_right_carry(cpu.registers.r_b, cpu.registers.r_f);
                    cpu.registers.r_b = action_pair.0;
                },
                /* C */ 0x01 => {
                    action_pair = bit_operations::rotate_right_carry(cpu.registers.r_c, cpu.registers.r_f);
                    cpu.registers.r_c = action_pair.0;
                },
                /* D */ 0x02 => {
                    action_pair = bit_operations::rotate_right_carry(cpu.registers.r_d, cpu.registers.r_f);
                    cpu.registers.r_d = action_pair.0;
                },
                /* E */ 0x03 => {
                    action_pair = bit_operations::rotate_right_carry(cpu.registers.r_e, cpu.registers.r_f);
                    cpu.registers.r_e = action_pair.0;
                },
                /* H */ 0x04 => {
                    action_pair = bit_operations::rotate_right_carry(cpu.registers.r_h, cpu.registers.r_f);
                    cpu.registers.r_h = action_pair.0;
                },
                /* L */ 0x05 => {
                    action_pair = bit_operations::rotate_right_carry(cpu.registers.r_l, cpu.registers.r_f);
                    cpu.registers.r_l = action_pair.0;
                },
                /* (HL) */ 0x06 => {
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8 = cpu.fetch_data(memory, a16_hl);
                    action_pair = bit_operations::rotate_right_carry(d8, cpu.registers.r_f);
                    cpu.write_data(memory, a16_hl, action_pair.0);
                    cycles = 4;
                },
                _ => panic!("There's an error at RRC"),
            };
            cpu.registers.r_f = action_pair.1 as u8;
            cycles
        },
        /* RL r/(HL) */ 0x10..=0x17 => {
            let mut cycles = 2;
            let action_pair;

            match opcode & 0x07 {
                /* A */ 0x07 => {
                    action_pair = bit_operations::rotate_left(cpu.registers.r_a, cpu.registers.r_f);
                    cpu.registers.r_a = action_pair.0;
                },
                /* B */ 0x00 => {
                    action_pair = bit_operations::rotate_left(cpu.registers.r_b, cpu.registers.r_f);
                    cpu.registers.r_b = action_pair.0;
                },
                /* C */ 0x01 => {
                    action_pair = bit_operations::rotate_left(cpu.registers.r_c, cpu.registers.r_f);
                    cpu.registers.r_c = action_pair.0;
                },
                /* D */ 0x02 => {
                    action_pair = bit_operations::rotate_left(cpu.registers.r_d, cpu.registers.r_f);
                    cpu.registers.r_d = action_pair.0;
                },
                /* E */ 0x03 => {
                    action_pair = bit_operations::rotate_left(cpu.registers.r_e, cpu.registers.r_f);
                    cpu.registers.r_e = action_pair.0;
                },
                /* H */ 0x04 => {
                    action_pair = bit_operations::rotate_left(cpu.registers.r_h, cpu.registers.r_f);
                    cpu.registers.r_h = action_pair.0;
                },
                /* L */ 0x05 => {
                    action_pair = bit_operations::rotate_left(cpu.registers.r_l, cpu.registers.r_f);
                    cpu.registers.r_l = action_pair.0;
                },
                /* (HL) */ 0x06 => {
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8 = cpu.fetch_data(memory, a16_hl);
                    action_pair = bit_operations::rotate_left(d8, cpu.registers.r_f);
                    cpu.write_data(memory, a16_hl, action_pair.0);
                    cycles = 4;
                },
                _ => panic!("There's an error at RL"),
            };
            cpu.registers.r_f = action_pair.1 as u8;
            cycles
        },
        /* RR r/(HL) */ 0x18..=0x1F => {
            let mut cycles = 2;
            let action_pair;

            match opcode & 0x07 {
                /* A */ 0x07 => {
                    action_pair = bit_operations::rotate_right(cpu.registers.r_a, cpu.registers.r_f);
                    cpu.registers.r_a = action_pair.0;
                },
                /* B */ 0x00 => {
                    action_pair = bit_operations::rotate_right(cpu.registers.r_b, cpu.registers.r_f);
                    cpu.registers.r_b = action_pair.0;
                },
                /* C */ 0x01 => {
                    action_pair = bit_operations::rotate_right(cpu.registers.r_c, cpu.registers.r_f);
                    cpu.registers.r_c = action_pair.0;
                },
                /* D */ 0x02 => {
                    action_pair = bit_operations::rotate_right(cpu.registers.r_d, cpu.registers.r_f);
                    cpu.registers.r_d = action_pair.0;
                },
                /* E */ 0x03 => {
                    action_pair = bit_operations::rotate_right(cpu.registers.r_e, cpu.registers.r_f);
                    cpu.registers.r_e = action_pair.0;
                },
                /* H */ 0x04 => {
                    action_pair = bit_operations::rotate_right(cpu.registers.r_h, cpu.registers.r_f);
                    cpu.registers.r_h = action_pair.0;
                },
                /* L */ 0x05 => {
                    action_pair = bit_operations::rotate_right(cpu.registers.r_l, cpu.registers.r_f);
                    cpu.registers.r_l = action_pair.0;
                },
                /* (HL) */ 0x06 => {
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8 = cpu.fetch_data(memory, a16_hl);
                    action_pair = bit_operations::rotate_right(d8, cpu.registers.r_f);
                    cpu.write_data(memory, a16_hl, action_pair.0);
                    cycles = 4;
                },
                _ => panic!("There's an error at RR"),
            };
            cpu.registers.r_f = action_pair.1 as u8;
            cycles
        },
        /* SLA r/(HL) */ 0x20..=0x27 => {
            let mut cycles = 2;
            let action_pair;

            match opcode & 0x07 {
                /* A */ 0x07 => {
                    action_pair = bit_operations::shift_left(cpu.registers.r_a);
                    cpu.registers.r_a = action_pair.0;
                },
                /* B */ 0x00 => {
                    action_pair = bit_operations::shift_left(cpu.registers.r_b);
                    cpu.registers.r_b = action_pair.0;
                },
                /* C */ 0x01 => {
                    action_pair = bit_operations::shift_left(cpu.registers.r_c);
                    cpu.registers.r_c = action_pair.0;
                },
                /* D */ 0x02 => {
                    action_pair = bit_operations::shift_left(cpu.registers.r_d);
                    cpu.registers.r_d = action_pair.0;
                },
                /* E */ 0x03 => {
                    action_pair = bit_operations::shift_left(cpu.registers.r_e);
                    cpu.registers.r_e = action_pair.0;
                },
                /* H */ 0x04 => {
                    action_pair = bit_operations::shift_left(cpu.registers.r_h);
                    cpu.registers.r_h = action_pair.0;
                },
                /* L */ 0x05 => {
                    action_pair = bit_operations::shift_left(cpu.registers.r_l);
                    cpu.registers.r_l = action_pair.0;
                },
                /* (HL) */ 0x06 => {
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8 = cpu.fetch_data(memory, a16_hl);
                    action_pair = bit_operations::shift_left(d8);
                    cpu.write_data(memory, a16_hl, action_pair.0);
                    cycles = 4;
                },
                _ => panic!("There's an error at SLA"),
            };
            cpu.registers.r_f = action_pair.1 as u8;
            cycles
        },
        /* SRA r/(HL) */ 0x28..=0x2F => {
            let mut cycles = 2;
            let action_pair;

            match opcode & 0x07 {
                /* A */ 0x07 => {
                    action_pair = bit_operations::shift_right(cpu.registers.r_a);
                    cpu.registers.r_a = action_pair.0;
                },
                /* B */ 0x00 => {
                    action_pair = bit_operations::shift_right(cpu.registers.r_b);
                    cpu.registers.r_b = action_pair.0;
                },
                /* C */ 0x01 => {
                    action_pair = bit_operations::shift_right(cpu.registers.r_c);
                    cpu.registers.r_c = action_pair.0;
                },
                /* D */ 0x02 => {
                    action_pair = bit_operations::shift_right(cpu.registers.r_d);
                    cpu.registers.r_d = action_pair.0;
                },
                /* E */ 0x03 => {
                    action_pair = bit_operations::shift_right(cpu.registers.r_e);
                    cpu.registers.r_e = action_pair.0;
                },
                /* H */ 0x04 => {
                    action_pair = bit_operations::shift_right(cpu.registers.r_h);
                    cpu.registers.r_h = action_pair.0;
                },
                /* L */ 0x05 => {
                    action_pair = bit_operations::shift_right(cpu.registers.r_l);
                    cpu.registers.r_l = action_pair.0;
                },
                /* (HL) */ 0x06 => {
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8 = cpu.fetch_data(memory, a16_hl);
                    action_pair = bit_operations::shift_right(d8);
                    cpu.write_data(memory, a16_hl, action_pair.0);
                    cycles = 4;
                },
                _ => panic!("There's an error at SRA"),
            };
            cpu.registers.r_f = action_pair.1 as u8;
            cycles
        },
        /* SWAP r/(HL) */ 0x30..=0x37 => {
            let mut cycles = 2;
            let register_data;

            match opcode & 0x07 {
                /* A */ 0x07 => {
                    register_data = bit_operations::swap_nibbles(cpu.registers.r_a as u16, 4) as u8;
                    cpu.registers.r_a = register_data;
                },
                /* B */ 0x00 => {
                    register_data = bit_operations::swap_nibbles(cpu.registers.r_b as u16, 4) as u8;
                    cpu.registers.r_b = register_data;
                },
                /* C */ 0x01 => {
                    register_data = bit_operations::swap_nibbles(cpu.registers.r_c as u16, 4) as u8;
                    cpu.registers.r_c = register_data;
                },
                /* D */ 0x02 => {
                    register_data = bit_operations::swap_nibbles(cpu.registers.r_d as u16, 4) as u8;
                    cpu.registers.r_d = register_data;
                },
                /* E */ 0x03 => {
                    register_data = bit_operations::swap_nibbles(cpu.registers.r_e as u16, 4) as u8;
                    cpu.registers.r_e = register_data;
                },
                /* H */ 0x04 => {
                    register_data = bit_operations::swap_nibbles(cpu.registers.r_h as u16, 4) as u8;
                    cpu.registers.r_h = register_data;
                },
                /* L */ 0x05 => {
                    register_data = bit_operations::swap_nibbles(cpu.registers.r_l as u16, 4) as u8;
                    cpu.registers.r_l = register_data;
                },
                /* (HL) */ 0x06 => {
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8 = cpu.fetch_data(memory, a16_hl);
                    register_data = bit_operations::swap_nibbles(d8 as u16, 4) as u8;
                    cpu.write_data(memory, a16_hl, register_data);
                    cycles = 4;
                },
                _ => panic!("There's an error at SWAP"),
            };
            cpu.registers.r_f = if register_data == 0 { flags::ZERO } else { flags::RESET };
            cycles
        },
        /* SRL r/(HL) */ 0x38..=0x3F => {
            let mut cycles = 2;
            let action_pair;

            match opcode & 0x07 {
                /* A */ 0x07 => {
                    action_pair = bit_operations::shift_right_reset(cpu.registers.r_a);
                    cpu.registers.r_a = action_pair.0;
                },
                /* B */ 0x00 => {
                    action_pair = bit_operations::shift_right_reset(cpu.registers.r_b);
                    cpu.registers.r_b = action_pair.0;
                },
                /* C */ 0x01 => {
                    action_pair = bit_operations::shift_right_reset(cpu.registers.r_c);
                    cpu.registers.r_c = action_pair.0;
                },
                /* D */ 0x02 => {
                    action_pair = bit_operations::shift_right_reset(cpu.registers.r_d);
                    cpu.registers.r_d = action_pair.0;
                },
                /* E */ 0x03 => {
                    action_pair = bit_operations::shift_right_reset(cpu.registers.r_e);
                    cpu.registers.r_e = action_pair.0;
                },
                /* H */ 0x04 => {
                    action_pair = bit_operations::shift_right_reset(cpu.registers.r_h);
                    cpu.registers.r_h = action_pair.0;
                },
                /* L */ 0x05 => {
                    action_pair = bit_operations::shift_right_reset(cpu.registers.r_l);
                    cpu.registers.r_l = action_pair.0;
                },
                /* (HL) */ 0x06 => {
                    let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
                    let d8 = cpu.fetch_data(memory, a16_hl);
                    action_pair = bit_operations::shift_right_reset(d8);
                    cpu.write_data(memory, a16_hl, action_pair.0);
                    cycles = 4;
                },
                _ => panic!("There's an error at SRL"),
            };
            cpu.registers.r_f = action_pair.1 as u8;
            cycles
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
            // Keep the CY flag value; the H flag must be set
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
