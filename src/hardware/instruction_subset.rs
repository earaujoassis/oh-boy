use super::cpu::CPU;
use super::memory::Memory;
use super::bit_operations;

/// This function represents the instruction subset executor within the 0xCB prefix.
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
        _ => panic!("Opcode unknown within prefix 0xCB: ${:02X}", opcode)
    }
}
