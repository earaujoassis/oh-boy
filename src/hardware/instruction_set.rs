use super::cpu::CPU;
use super::memory::Memory;
use super::memory_map;
use super::bit_operations;

/// This function represents the instruction set executor within the CPU.
/// It receives an 8bit/1byte opcode and checks if argument bytes/bits
/// are necessary (and request it if necessary). Each opcode returns
/// the duration in machine cycles.

//// This is a Little-endian CPU/Memory system (LSB, MSB)
////
pub fn execute(cpu: &mut CPU, memory: &mut Memory, opcode: u8) -> usize {
    match opcode {
        /* NOP  */ 0x00 => { 1 },
        /* STOP */ 0x10 => {
            cpu.fetch_operand(memory);
            cpu.stopped = true;
            1
        },
        /* HALT */ 0x76 => { cpu.halted = true; 1 },
        /* DI   */ 0xF3 => { cpu.interruption_enabled = false; 1 },
        /* EI   */ 0xFB => { cpu.interruption_enabled = true; 1 },
        /* LD BC,d16 */ 0x01 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb.into(), msb.into(), 8);
            cpu.registers.r_b = bit_operations::msb(d16, 8);
            cpu.registers.r_c = bit_operations::lsb(d16, 8);
            3
        },
        /* LD (a16),SP */ 0x08 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb.into(), msb.into(), 8);
            cpu.write_data(memory, d16, bit_operations::lsb(d16, 8));
            cpu.write_data(memory, d16 + 1, bit_operations::msb(d16, 8));
            5
        },
        /* LD DE,d16 */ 0x11 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb.into(), msb.into(), 8);
            cpu.registers.r_d = bit_operations::msb(d16, 8);
            cpu.registers.r_e = bit_operations::lsb(d16, 8);
            3
        },
        /* LD HL,d16 */ 0x21 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb.into(), msb.into(), 8);
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            3
        },
        /* LD SP,d16 */ 0x31 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb.into(), msb.into(), 8);
            cpu.registers.stack_pointer = d16;
            3
        },
        /* POP BC */ 0xC1 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 1;
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 1;
            cpu.registers.r_b = msb;
            cpu.registers.r_c = lsb;
            3
        },
        /* POP DE */ 0xD1 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 1;
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 1;
            cpu.registers.r_d = msb;
            cpu.registers.r_e = lsb;
            3
        },
        /* POP HL */ 0xE1 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 1;
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 1;
            cpu.registers.r_h = msb;
            cpu.registers.r_l = lsb;
            3
        },
        /* POP AF */ 0xF1 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 1;
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 1;
            cpu.registers.r_a = msb;
            cpu.registers.r_f = lsb;
            3
        },
        /* PUSH BC */ 0xC5 => {
            let msb = cpu.registers.r_b;
            let lsb = cpu.registers.r_c;
            cpu.registers.stack_pointer -= 1;
            cpu.write_data(memory, cpu.registers.stack_pointer, lsb);
            cpu.registers.stack_pointer -= 1;
            cpu.write_data(memory, cpu.registers.stack_pointer, msb);
            4
        },
        /* PUSH DE */ 0xD5 => {
            let msb = cpu.registers.r_d;
            let lsb = cpu.registers.r_e;
            cpu.registers.stack_pointer -= 1;
            cpu.write_data(memory, cpu.registers.stack_pointer, lsb);
            cpu.registers.stack_pointer -= 1;
            cpu.write_data(memory, cpu.registers.stack_pointer, msb);
            4
        },
        /* PUSH HL */ 0xE5 => {
            let msb = cpu.registers.r_h;
            let lsb = cpu.registers.r_l;
            cpu.registers.stack_pointer -= 1;
            cpu.write_data(memory, cpu.registers.stack_pointer, lsb);
            cpu.registers.stack_pointer -= 1;
            cpu.write_data(memory, cpu.registers.stack_pointer, msb);
            4
        },
        /* PUSH AF */ 0xF5 => {
            let msb = cpu.registers.r_a;
            let lsb = cpu.registers.r_f;
            cpu.registers.stack_pointer -= 1;
            cpu.write_data(memory, cpu.registers.stack_pointer, lsb);
            cpu.registers.stack_pointer -= 1;
            cpu.write_data(memory, cpu.registers.stack_pointer, msb);
            4
        },
        /* LD HL,SP+r8 */ 0xF8 => {
            let r8 = cpu.fetch_operand(memory);
            let d16 = (cpu.registers.stack_pointer + (r8 as u16)) as u16;
            let carry_from_11th_bit_h = if d16 & 0xF800 > 1 { 0x20 } else { 0x00 };
            let carry_from_15th_bit_c = if d16 & 0x8000 > 1 { 0x10 } else { 0x00 };
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (carry_from_11th_bit_h | carry_from_15th_bit_c) as u8;
            3
        },
        /* LD SP,HL */ 0xF9 => {
            let h = cpu.registers.r_h;
            let l = cpu.registers.r_l;
            cpu.registers.stack_pointer = bit_operations::join_words(h as u16, l as u16, 8);
            2
        },
        /* INC BC */ 0x03 => {
            let msb = cpu.registers.r_b;
            let lsb = cpu.registers.r_c;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8) + 1;
            cpu.registers.r_b = bit_operations::msb(d16, 8);
            cpu.registers.r_c = bit_operations::lsb(d16, 8);
            2
        },
        /* INC DE */ 0x13 => {
            let msb = cpu.registers.r_d;
            let lsb = cpu.registers.r_e;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8) + 1;
            cpu.registers.r_d = bit_operations::msb(d16, 8);
            cpu.registers.r_e = bit_operations::lsb(d16, 8);
            2
        },
        /* INC HL */ 0x23 => {
            let msb = cpu.registers.r_h;
            let lsb = cpu.registers.r_l;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8) + 1;
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            2
        },
        /* INC SP */ 0x33 => {
            cpu.registers.stack_pointer += 1;
            2
        },
        /* ADD HL,BC */ 0x09 => {
            let d16_bc = bit_operations::join_words(cpu.registers.r_b as u16, cpu.registers.r_c as u16, 8) + 1;
            let d16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8) + 1;
            let d16 = d16_hl + d16_bc;
            let carry_from_11th_bit_h = if d16 & 0xF800 > 1 { 0x20 } else { 0x00 };
            let carry_from_15th_bit_c = if d16 & 0x8000 > 1 { 0x10 } else { 0x00 };
            let keep_z_flag = cpu.registers.r_f & 0x80;
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (keep_z_flag | carry_from_11th_bit_h | carry_from_15th_bit_c) as u8;
            2
        },
        /* ADD HL,DE */ 0x19 => {
            let d16_de = bit_operations::join_words(cpu.registers.r_d as u16, cpu.registers.r_e as u16, 8) + 1;
            let d16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8) + 1;
            let d16 = d16_hl + d16_de;
            let carry_from_11th_bit_h = if d16 & 0xF800 > 1 { 0x20 } else { 0x00 };
            let carry_from_15th_bit_c = if d16 & 0x8000 > 1 { 0x10 } else { 0x00 };
            let keep_z_flag = cpu.registers.r_f & 0x80;
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (keep_z_flag | carry_from_11th_bit_h | carry_from_15th_bit_c) as u8;
            2
        },
        /* ADD HL,HL */ 0x29 => {
            let d16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8) + 1;
            let d16 = d16_hl + d16_hl; // Explicitly 2 * d16_hl;
            let carry_from_11th_bit_h = if d16 & 0xF800 > 1 { 0x20 } else { 0x00 };
            let carry_from_15th_bit_c = if d16 & 0x8000 > 1 { 0x10 } else { 0x00 };
            let keep_z_flag = cpu.registers.r_f & 0x80;
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (keep_z_flag | carry_from_11th_bit_h | carry_from_15th_bit_c) as u8;
            2
        },
        /* ADD HL,SP */ 0x39 => {
            let d16_sp = cpu.registers.stack_pointer;
            let d16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8) + 1;
            let d16 = d16_hl + d16_sp;
            let carry_from_11th_bit_h = if d16 & 0xF800 > 1 { 0x20 } else { 0x00 };
            let carry_from_15th_bit_c = if d16 & 0x8000 > 1 { 0x10 } else { 0x00 };
            let keep_z_flag = cpu.registers.r_f & 0x80;
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (keep_z_flag | carry_from_11th_bit_h | carry_from_15th_bit_c) as u8;
            2
        },
        /* DEC BC */ 0x0B => {
            let msb = cpu.registers.r_b;
            let lsb = cpu.registers.r_c;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8) - 1;
            cpu.registers.r_b = bit_operations::msb(d16, 8);
            cpu.registers.r_c = bit_operations::lsb(d16, 8);
            2
        },
        /* DEC DE */ 0x1B => {
            let msb = cpu.registers.r_d;
            let lsb = cpu.registers.r_e;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8) - 1;
            cpu.registers.r_d = bit_operations::msb(d16, 8);
            cpu.registers.r_e = bit_operations::lsb(d16, 8);
            2
        },
        /* DEC HL */ 0x2B => {
            let msb = cpu.registers.r_h;
            let lsb = cpu.registers.r_l;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8) - 1;
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            2
        },
        /* DEC SP */ 0x3B => {
            cpu.registers.stack_pointer -= 1;
            2
        },
        /* ADD SP,r8 */ 0xE8 => {
            let r8 = cpu.fetch_operand(memory);
            let d16 = cpu.registers.stack_pointer + (r8 as u16);
            let carry_from_11th_bit_h = if d16 & 0xF800 > 1 { 0x20 } else { 0x00 };
            let carry_from_15th_bit_c = if d16 & 0x8000 > 1 { 0x10 } else { 0x00 };
            cpu.registers.stack_pointer = d16;
            cpu.registers.r_f = (carry_from_11th_bit_h | carry_from_15th_bit_c) as u8;
            4
        },
        _ => panic!("Opcode unknown: ${:02X}", opcode)
    }
}
