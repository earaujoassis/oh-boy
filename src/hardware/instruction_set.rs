use super::cpu::CPU;
use super::memory::Memory;
use super::bit_operations;
use super::flags;
use super::arithmetic;
use super::instruction_subset;

/// This function represents the instruction set executor within the CPU.
/// It receives an 8bit/1byte opcode and checks if argument bytes/bits
/// are necessary (and request it if necessary). Each opcode returns
/// the duration in machine cycles.
///
/// This is a Little-endian CPU -> Memory system (LSB, MSB)
///
pub fn execute(cpu: &mut CPU, memory: &mut Memory, opcode: u8) -> usize {
    match opcode {
        /* NOP  */ 0x00 => { 1 },
        /* STOP */ 0x10 => { cpu.fetch_operand(memory); cpu.stopped = true; 1 },
        /* HALT */ 0x76 => { cpu.halted = true; 1 },
        /* DI   */ 0xF3 => { cpu.interruption_enabled = false; 1 },
        /* EI   */ 0xFB => { cpu.interruption_enabled = true; 1 },
        /* LD BC,d16 */ 0x01 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.r_b = bit_operations::msb(d16, 8);
            cpu.registers.r_c = bit_operations::lsb(d16, 8);
            3
        },
        /* LD (a16),SP */ 0x08 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.write_data(memory, a16, bit_operations::lsb(a16, 8));
            cpu.write_data(memory, a16.wrapping_add(1), bit_operations::msb(a16, 8));
            5
        },
        /* LD DE,d16 */ 0x11 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.r_d = bit_operations::msb(d16, 8);
            cpu.registers.r_e = bit_operations::lsb(d16, 8);
            3
        },
        /* LD HL,d16 */ 0x21 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            3
        },
        /* LD SP,d16 */ 0x31 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let d16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.stack_pointer = d16;
            3
        },
        /* POP BC */ 0xC1 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            cpu.registers.r_b = msb;
            cpu.registers.r_c = lsb;
            3
        },
        /* POP DE */ 0xD1 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            cpu.registers.r_d = msb;
            cpu.registers.r_e = lsb;
            3
        },
        /* POP HL */ 0xE1 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            cpu.registers.r_h = msb;
            cpu.registers.r_l = lsb;
            3
        },
        /* POP AF */ 0xF1 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            cpu.registers.r_a = msb;
            cpu.registers.r_f = lsb;
            3
        },
        /* PUSH BC */ 0xC5 => {
            let msb = cpu.registers.r_b;
            let lsb = cpu.registers.r_c;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, lsb);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, msb);
            4
        },
        /* PUSH DE */ 0xD5 => {
            let msb = cpu.registers.r_d;
            let lsb = cpu.registers.r_e;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, lsb);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, msb);
            4
        },
        /* PUSH HL */ 0xE5 => {
            let msb = cpu.registers.r_h;
            let lsb = cpu.registers.r_l;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, lsb);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, msb);
            4
        },
        /* PUSH AF */ 0xF5 => {
            let msb = cpu.registers.r_a;
            let lsb = cpu.registers.r_f;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, lsb);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, msb);
            4
        },
        /* LD HL,SP+r8 */ 0xF8 => {
            let r8 = cpu.fetch_operand(memory);
            let sp = cpu.registers.stack_pointer;
            let d16 = (sp as i16).wrapping_add(r8 as i16) as u16;
            // let zero_flag = flags::RESET; -> this is implied
            // let subtract_flag = flags::RESET; -> this is implied
            let half_carry_flag = if (d16 & 0xF) < (sp & 0xF) { flags::HALF_CARRY } else { flags::RESET };
            let carry_flag = if (d16 & 0xFF) < (sp & 0xFF) { flags::CARRY } else { flags::RESET };
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (half_carry_flag | carry_flag) as u8;
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
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8).wrapping_add(1);
            cpu.registers.r_b = bit_operations::msb(d16, 8);
            cpu.registers.r_c = bit_operations::lsb(d16, 8);
            2
        },
        /* INC DE */ 0x13 => {
            let msb = cpu.registers.r_d;
            let lsb = cpu.registers.r_e;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8).wrapping_add(1);
            cpu.registers.r_d = bit_operations::msb(d16, 8);
            cpu.registers.r_e = bit_operations::lsb(d16, 8);
            2
        },
        /* INC HL */ 0x23 => {
            let msb = cpu.registers.r_h;
            let lsb = cpu.registers.r_l;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8).wrapping_add(1);
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            2
        },
        /* INC SP */ 0x33 => {
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            2
        },
        /* ADD HL,BC */ 0x09 => {
            let d16_bc = bit_operations::join_words(cpu.registers.r_b as u16, cpu.registers.r_c as u16, 8);
            let d16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d16 = d16_hl.wrapping_add(d16_bc);
            let zero_flag = cpu.registers.r_f & flags::ZERO; // keep its value
            // let subtract_flag = flags::RESET; -> this is implied
            let half_carry_flag = if (((d16_hl & 0xFFF) + (d16_bc & 0xFFF)) & 0x1000) != 0 { flags::HALF_CARRY } else { flags::RESET };
            let carry_flag = if d16_hl > 0xFFFF - d16_bc { flags::CARRY } else { flags::RESET };
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (zero_flag | half_carry_flag | carry_flag) as u8;
            2
        },
        /* ADD HL,DE */ 0x19 => {
            let d16_de = bit_operations::join_words(cpu.registers.r_d as u16, cpu.registers.r_e as u16, 8);
            let d16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d16 = d16_hl.wrapping_add(d16_de);
            let zero_flag = cpu.registers.r_f & flags::ZERO; // keep its value
            // let subtract_flag = flags::RESET; -> this is implied
            let half_carry_flag = if (((d16_hl & 0xFFF) + (d16_de & 0xFFF)) & 0x1000) != 0 { flags::HALF_CARRY } else { flags::RESET };
            let carry_flag = if d16_hl > 0xFFFF - d16_de { flags::CARRY } else { flags::RESET };
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (zero_flag | half_carry_flag | carry_flag) as u8;
            2
        },
        /* ADD HL,HL */ 0x29 => {
            let d16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d16 = d16_hl.wrapping_add(d16_hl); // Explicitly 2 * d16_hl;
            let zero_flag = cpu.registers.r_f & flags::ZERO; // keep its value
            // let subtract_flag = flags::RESET; -> this is implied
            let half_carry_flag = if (((d16_hl & 0xFFF) + (d16_hl & 0xFFF)) & 0x1000) != 0 { flags::HALF_CARRY } else { flags::RESET };
            let carry_flag = if d16_hl > 0xFFFF - d16_hl { flags::CARRY } else { flags::RESET };
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (zero_flag | half_carry_flag | carry_flag) as u8;
            2
        },
        /* ADD HL,SP */ 0x39 => {
            let d16_sp = cpu.registers.stack_pointer;
            let d16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d16 = d16_hl.wrapping_add(d16_sp);
            let zero_flag = cpu.registers.r_f & flags::ZERO; // keep its value
            // let subtract_flag = flags::RESET; -> this is implied
            let half_carry_flag = if (((d16_hl & 0xFFF) + (d16_sp & 0xFFF)) & 0x1000) != 0 { flags::HALF_CARRY } else { flags::RESET };
            let carry_flag = if d16_hl > 0xFFFF - d16_sp { flags::CARRY } else { flags::RESET };
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            cpu.registers.r_f = (zero_flag | half_carry_flag | carry_flag) as u8;
            2
        },
        /* DEC BC */ 0x0B => {
            let msb = cpu.registers.r_b;
            let lsb = cpu.registers.r_c;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8).wrapping_sub(1);
            cpu.registers.r_b = bit_operations::msb(d16, 8);
            cpu.registers.r_c = bit_operations::lsb(d16, 8);
            2
        },
        /* DEC DE */ 0x1B => {
            let msb = cpu.registers.r_d;
            let lsb = cpu.registers.r_e;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8).wrapping_sub(1);
            cpu.registers.r_d = bit_operations::msb(d16, 8);
            cpu.registers.r_e = bit_operations::lsb(d16, 8);
            2
        },
        /* DEC HL */ 0x2B => {
            let msb = cpu.registers.r_h;
            let lsb = cpu.registers.r_l;
            let d16 = bit_operations::join_words(msb as u16, lsb as u16, 8).wrapping_sub(1);
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            2
        },
        /* DEC SP */ 0x3B => {
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            2
        },
        /* ADD SP,r8 */ 0xE8 => {
            let r8 = cpu.fetch_operand(memory);
            let sp = cpu.registers.stack_pointer;
            let d16 = sp.wrapping_add(r8 as u16);
            // let zero_flag = flags::RESET; -> this is implied
            // let subtract_flag = flags::RESET; -> this is implied
            let half_carry_flag = if (d16 & 0xF) < (sp & 0xF) { flags::HALF_CARRY } else { flags::RESET };
            let carry_flag = if (d16 & 0xFF) < (sp & 0xFF) { flags::CARRY } else { flags::RESET };
            cpu.registers.stack_pointer = d16;
            cpu.registers.r_f = (half_carry_flag | carry_flag) as u8;
            4
        },
        /* JR Z,r8 */ 0x28 => {
            // Rust represents integer numbers using the two's complement represenation,
            // so we don't have to change that ourselves since DMG uses the same representation
            let r8 = cpu.fetch_operand(memory) as i8;
            // It is executed if Z == 1
            // Check if Z is 1
            if (cpu.registers.r_f & flags::ZERO) > 0 {
                cpu.registers.program_counter = (cpu.registers.program_counter as i16).wrapping_add(r8 as i16) as u16;
                3
            } else {
                2
            }
        },
        /* JR C,r8 */ 0x38 => {
            // Rust represents integer numbers using the two's complement represenation,
            // so we don't have to change that ourselves since DMG uses the same representation
            let r8 = cpu.fetch_operand(memory) as i8;
            // It is executed if CY == 1
            // Check if CY is 1
            if (cpu.registers.r_f & flags::CARRY) > 0 {
                cpu.registers.program_counter = (cpu.registers.program_counter as i16).wrapping_add(r8 as i16) as u16;
                3
            } else {
                2
            }
        },
        /* JR NZ,r8 */ 0x20 => {
            // Rust represents integer numbers using the two's complement represenation,
            // so we don't have to change that ourselves since DMG uses the same representation
            let r8 = cpu.fetch_operand(memory) as i8;
            // It is executed if Z == 0
            // Check if Z is 0
            if (cpu.registers.r_f & flags::ZERO) == 0x00 {
                cpu.registers.program_counter = (cpu.registers.program_counter as i16).wrapping_add(r8 as i16) as u16;
                3
            } else {
                2
            }
        },
        /* JR NC,r8 */ 0x30 => {
            // Rust represents integer numbers using the two's complement represenation,
            // so we don't have to change that ourselves since DMG uses the same representation
            let r8 = cpu.fetch_operand(memory) as i8;
            // It is executed if CY == 0
            // Check if CY is 0
            if (cpu.registers.r_f & flags::CARRY) == 0x00 {
                cpu.registers.program_counter = (cpu.registers.program_counter as i16).wrapping_add(r8 as i16) as u16;
                3
            } else {
                2
            }
        },
        /* JR r8 */ 0x18 => {
            // Rust represents integer numbers using the two's complement represenation,
            // so we don't have to change that ourselves since DMG uses the same representation
            let r8 = cpu.fetch_operand(memory) as i8;
            cpu.registers.program_counter = (cpu.registers.program_counter as i16).wrapping_add(r8 as i16) as u16;
            3
        },
        /* JP Z,a16 */ 0xCA => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            // It is executed if Z == 1
            // Check if Z is 1
            if (cpu.registers.r_f & flags::ZERO) > 0 {
                cpu.registers.program_counter = a16;
                4
            } else {
                3
            }
        },
        /* JP C,a16 */ 0xDA => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            // It is executed if CY == 1
            // Check if CY is 1
            if (cpu.registers.r_f & flags::CARRY) > 0 {
                cpu.registers.program_counter = a16;
                4
            } else {
                3
            }
        },
        /* JP NZ,a16 */ 0xC2 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            // It is executed if Z == 0
            // Check if Z is 0
            if (cpu.registers.r_f & flags::ZERO) == 0x00 {
                cpu.registers.program_counter = a16;
                4
            } else {
                3
            }
        },
        /* JP NC,a16 */ 0xD2 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            // It is executed if CY == 0
            // Check if CY is 0
            if (cpu.registers.r_f & flags::CARRY) == 0x00 {
                cpu.registers.program_counter = a16;
                4
            } else {
                3
            }
        },
        /* JP a16 */ 0xC3 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.program_counter = a16;
            4
        },
        /* JP (HL) */ 0xE9 => {
            let msb = cpu.registers.r_h;
            let lsb = cpu.registers.r_l;
            let a16 = bit_operations::join_words(msb as u16, lsb as u16, 8);
            cpu.registers.program_counter = a16;
            1
        },
        /* CALL Z,a16 */ 0xCC => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            // It is executed if Z == 1
            // Check if Z is 1
            if (cpu.registers.r_f & flags::ZERO) > 0 {
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
                cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
                cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
                cpu.registers.program_counter = a16;
                6
            } else {
                3
            }
        },
        /* CALL C,a16 */ 0xDC => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            // It is executed if CY == 1
            // Check if CY is 1
            if (cpu.registers.r_f & flags::CARRY) > 0 {
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
                cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
                cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
                cpu.registers.program_counter = a16;
                6
            } else {
                3
            }
        },
        /* CALL NZ,a16 */ 0xC4 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            // It is executed if Z == 0
            // Check if Z is 0
            if (cpu.registers.r_f & flags::ZERO) == 0x00 {
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
                cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
                cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
                cpu.registers.program_counter = a16;
                6
            } else {
                3
            }
        },
        /* CALL NC,a16 */ 0xD4 => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            // It is executed if CY == 0
            // Check if CY is 0
            if (cpu.registers.r_f & flags::CARRY) == 0x00 {
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
                cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
                cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
                cpu.registers.program_counter = a16;
                6
            } else {
                3
            }
        },
        /* CALL a16 */ 0xCD => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            6
        },
        /* RET Z */ 0xC8 => {
            // It is executed if Z == 1
            // Check if Z is 1
            if (cpu.registers.r_f & flags::ZERO) > 0 {
                let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
                let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
                let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
                cpu.registers.program_counter = a16;
                5
            } else {
                2
            }
        },
        /* RET C */ 0xD8 => {
            // It is executed if CY == 1
            // Check if CY is 1
            if (cpu.registers.r_f & flags::CARRY) > 0 {
                let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
                let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
                let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
                cpu.registers.program_counter = a16;
                5
            } else {
                2
            }
        },
        /* RET NZ */ 0xC0 => {
            // It is executed if Z == 0
            // Check if Z is 0
            if (cpu.registers.r_f & flags::ZERO) == 0x00 {
                let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
                let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
                let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
                cpu.registers.program_counter = a16;
                5
            } else {
                2
            }
        },
        /* RET NC */ 0xD0 => {
            // It is executed if CY == 0
            // Check if CY is 0
            if (cpu.registers.r_f & flags::CARRY) == 0x00 {
                let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
                let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
                cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
                let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
                cpu.registers.program_counter = a16;
                5
            } else {
                2
            }
        },
        /* RET */ 0xC9 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.program_counter = a16;
            4
        },
        /* RETI */ 0xD9 => {
            let lsb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            let msb = cpu.fetch_data(memory, cpu.registers.stack_pointer);
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.program_counter = a16;
            4
        },
        /* RST 00H */ 0xC7 => {
            let a16: u16 = 0x0000;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            4
        },
        /* RST 10H */ 0xD7 => {
            let a16: u16 = 0x0010;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            4
        },
        /* RST 20H */ 0xE7 => {
            let a16: u16 = 0x0020;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            4
        },
        /* RST 30H */ 0xF7 => {
            let a16: u16 = 0x0030;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            4
        },
        /* RST 08H */ 0xCF => {
            let a16: u16 = 0x0008;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            4
        },
        /* RST 18H */ 0xDF => {
            let a16: u16 = 0x0018;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            4
        },
        /* RST 28H */ 0xEF => {
            let a16: u16 = 0x0028;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            4
        },
        /* RST 38H */ 0xFF => {
            let a16: u16 = 0x0038;
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
            cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
            cpu.registers.program_counter = a16;
            4
        },
        /* LDH (a8),A */ 0xE0 => {
            let a8 = cpu.fetch_operand(memory);
            let a16 = (a8 as u16).wrapping_add(0xFF00) as u16;
            cpu.write_data(memory, a16, cpu.registers.r_a);
            3
        },
        /* LDH A,(a8) */ 0xF0 => {
            let a8 = cpu.fetch_operand(memory);
            let a16 = (a8 as u16).wrapping_add(0xFF00) as u16;
            cpu.registers.r_a = cpu.fetch_data(memory, a16);
            3
        },
        /* LD (C),A */ 0xE2 => {
            let a16 = (cpu.registers.r_c as u16).wrapping_add(0xFF00);
            cpu.write_data(memory, a16, cpu.registers.r_a);
            2
        },
        /* LD A,(C) */ 0xF2 => {
            let a16 = (cpu.registers.r_c as u16).wrapping_add(0xFF00);
            cpu.registers.r_a = cpu.fetch_data(memory, a16);
            2
        },
        /* LD (a16),A */ 0xEA => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.write_data(memory, a16, cpu.registers.r_a);
            4
        },
        /* LD A,(a16) */ 0xFA => {
            let lsb = cpu.fetch_operand(memory);
            let msb = cpu.fetch_operand(memory);
            let a16 = bit_operations::endianess(lsb as u16, msb as u16, 8);
            cpu.registers.r_a = cpu.fetch_data(memory, a16);
            4
        },
        /* LD B,B */ 0x40 => { cpu.registers.r_b = cpu.registers.r_b; 1 },
        /* LD B,C */ 0x41 => { cpu.registers.r_b = cpu.registers.r_c; 1 },
        /* LD B,D */ 0x42 => { cpu.registers.r_b = cpu.registers.r_d; 1 },
        /* LD B,E */ 0x43 => { cpu.registers.r_b = cpu.registers.r_e; 1 },
        /* LD B,H */ 0x44 => { cpu.registers.r_b = cpu.registers.r_h; 1 },
        /* LD B,L */ 0x45 => { cpu.registers.r_b = cpu.registers.r_l; 1 },
        /* LD B,(HL) */ 0x46 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_b = d8;
            2
        },
        /* LD B,A */ 0x47 => { cpu.registers.r_b = cpu.registers.r_a; 1 },
        /* LD C,B */ 0x48 => { cpu.registers.r_c = cpu.registers.r_b; 1 },
        /* LD C,C */ 0x49 => { cpu.registers.r_c = cpu.registers.r_c; 1 },
        /* LD C,D */ 0x4A => { cpu.registers.r_c = cpu.registers.r_d; 1 },
        /* LD C,E */ 0x4B => { cpu.registers.r_c = cpu.registers.r_e; 1 },
        /* LD C,H */ 0x4C => { cpu.registers.r_c = cpu.registers.r_h; 1 },
        /* LD C,L */ 0x4D => { cpu.registers.r_c = cpu.registers.r_l; 1 },
        /* LD C,(HL) */ 0x4E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_c = d8;
            2
        },
        /* LD C,A */ 0x4F => { cpu.registers.r_c = cpu.registers.r_a; 1 },
        /* LD D,B */ 0x50 => { cpu.registers.r_d = cpu.registers.r_b; 1 },
        /* LD D,C */ 0x51 => { cpu.registers.r_d = cpu.registers.r_c; 1 },
        /* LD D,D */ 0x52 => { cpu.registers.r_d = cpu.registers.r_d; 1 },
        /* LD D,E */ 0x53 => { cpu.registers.r_d = cpu.registers.r_e; 1 },
        /* LD D,H */ 0x54 => { cpu.registers.r_d = cpu.registers.r_h; 1 },
        /* LD D,L */ 0x55 => { cpu.registers.r_d = cpu.registers.r_l; 1 },
        /* LD D,(HL) */ 0x56 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_d = d8;
            2
        },
        /* LD D,A */ 0x57 => { cpu.registers.r_d = cpu.registers.r_a; 1 },
        /* LD E,B */ 0x58 => { cpu.registers.r_e = cpu.registers.r_b; 1 },
        /* LD E,C */ 0x59 => { cpu.registers.r_e = cpu.registers.r_c; 1 },
        /* LD E,D */ 0x5A => { cpu.registers.r_e = cpu.registers.r_d; 1 },
        /* LD E,E */ 0x5B => { cpu.registers.r_e = cpu.registers.r_e; 1 },
        /* LD E,H */ 0x5C => { cpu.registers.r_e = cpu.registers.r_h; 1 },
        /* LD E,L */ 0x5D => { cpu.registers.r_e = cpu.registers.r_l; 1 },
        /* LD E,(HL) */ 0x5E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_e = d8;
            2
        },
        /* LD E,A */ 0x5F => { cpu.registers.r_e = cpu.registers.r_a; 1 },
        /* LD H,B */ 0x60 => { cpu.registers.r_h = cpu.registers.r_b; 1 },
        /* LD H,C */ 0x61 => { cpu.registers.r_h = cpu.registers.r_c; 1 },
        /* LD H,D */ 0x62 => { cpu.registers.r_h = cpu.registers.r_d; 1 },
        /* LD H,E */ 0x63 => { cpu.registers.r_h = cpu.registers.r_e; 1 },
        /* LD H,H */ 0x64 => { cpu.registers.r_h = cpu.registers.r_h; 1 },
        /* LD H,L */ 0x65 => { cpu.registers.r_h = cpu.registers.r_l; 1 },
        /* LD H,(HL) */ 0x66 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_h = d8;
            2
        },
        /* LD H,A */ 0x67 => { cpu.registers.r_h = cpu.registers.r_a; 1 },
        /* LD L,B */ 0x68 => { cpu.registers.r_l = cpu.registers.r_b; 1 },
        /* LD L,C */ 0x69 => { cpu.registers.r_l = cpu.registers.r_c; 1 },
        /* LD L,D */ 0x6A => { cpu.registers.r_l = cpu.registers.r_d; 1 },
        /* LD L,E */ 0x6B => { cpu.registers.r_l = cpu.registers.r_e; 1 },
        /* LD L,H */ 0x6C => { cpu.registers.r_l = cpu.registers.r_h; 1 },
        /* LD L,L */ 0x6D => { cpu.registers.r_l = cpu.registers.r_l; 1 },
        /* LD L,(HL) */ 0x6E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_l = d8;
            2
        },
        /* LD L,A */ 0x6F => { cpu.registers.r_l = cpu.registers.r_a; 1 },
        /* LD (HL),B */ 0x70 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_b);
            2
        },
        /* LD (HL),C */ 0x71 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_c);
            2
        },
        /* LD (HL),D */ 0x72 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_d);
            2
        },
        /* LD (HL),E */ 0x73 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_e);
            2
        },
        /* LD (HL),H */ 0x74 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_h);
            2
        },
        /* LD (HL),L */ 0x75 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_l);
            2
        },
        /* LD (HL),A */ 0x77 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_a);
            2
        },
        /* LD A,B */ 0x78 => { cpu.registers.r_a = cpu.registers.r_b; 1 },
        /* LD A,C */ 0x79 => { cpu.registers.r_a = cpu.registers.r_c; 1 },
        /* LD A,D */ 0x7A => { cpu.registers.r_a = cpu.registers.r_d; 1 },
        /* LD A,E */ 0x7B => { cpu.registers.r_a = cpu.registers.r_e; 1 },
        /* LD A,H */ 0x7C => { cpu.registers.r_a = cpu.registers.r_h; 1 },
        /* LD A,L */ 0x7D => { cpu.registers.r_a = cpu.registers.r_l; 1 },
        /* LD A,(HL) */ 0x7E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_a = d8;
            2
        },
        /* LD A,A */ 0x7F => { cpu.registers.r_a = cpu.registers.r_a; 1 },
        /* LD (BC),A */ 0x02 => {
            let a16_bc = bit_operations::join_words(cpu.registers.r_b as u16, cpu.registers.r_c as u16, 8);
            cpu.write_data(memory, a16_bc, cpu.registers.r_a);
            2
        },
        /* LD (DE),A */ 0x12 => {
            let a16_de = bit_operations::join_words(cpu.registers.r_d as u16, cpu.registers.r_e as u16, 8);
            cpu.write_data(memory, a16_de, cpu.registers.r_a);
            2
        },
        /* LD (HL+),A */ 0x22 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_a);
            let d16 = a16_hl.wrapping_add(1);
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            2
        },
        /* LD (HL-),A */ 0x32 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, cpu.registers.r_a);
            let d16 = a16_hl.wrapping_sub(1);
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            2
        },
        /* LD B,d8 */ 0x06 => {
            let d8 = cpu.fetch_operand(memory);
            cpu.registers.r_b = d8;
            2
        },
        /* LD D,d8 */ 0x16 => {
            let d8 = cpu.fetch_operand(memory);
            cpu.registers.r_d = d8;
            2
        },
        /* LD H,d8 */ 0x26 => {
            let d8 = cpu.fetch_operand(memory);
            cpu.registers.r_h = d8;
            2
        },
        /* LD (HL),d8 */ 0x36 => {
            let d8 = cpu.fetch_operand(memory);
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            cpu.write_data(memory, a16_hl, d8);
            3
        },
        /* LD A,(BC) */ 0x0A => {
            let a16_bc = bit_operations::join_words(cpu.registers.r_b as u16, cpu.registers.r_c as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_bc);
            cpu.registers.r_a = d8;
            2
        },
        /* LD A,(DE) */ 0x1A => {
            let a16_de = bit_operations::join_words(cpu.registers.r_d as u16, cpu.registers.r_e as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_de);
            cpu.registers.r_a = d8;
            2
        },
        /* LD A,(HL+) */ 0x2A => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_a = d8;
            let d16 = a16_hl.wrapping_add(1);
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            2
        },
        /* LD A,(HL-) */ 0x3A => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            cpu.registers.r_a = d8;
            let d16 = a16_hl.wrapping_sub(1);
            cpu.registers.r_h = bit_operations::msb(d16, 8);
            cpu.registers.r_l = bit_operations::lsb(d16, 8);
            2
        },
        /* LD C,d8 */ 0x0E => {
            let d8 = cpu.fetch_operand(memory);
            cpu.registers.r_c = d8;
            2
        },
        /* LD E,d8 */ 0x1E => {
            let d8 = cpu.fetch_operand(memory);
            cpu.registers.r_e = d8;
            2
        },
        /* LD L,d8 */ 0x2E => {
            let d8 = cpu.fetch_operand(memory);
            cpu.registers.r_l = d8;
            2
        },
        /* LD A,d8 */ 0x3E => {
            let d8 = cpu.fetch_operand(memory);
            cpu.registers.r_a = d8;
            2
        },
        /* INC B */ 0x04 => { cpu.registers.r_b = arithmetic::increment(cpu, cpu.registers.r_b); 1 },
        /* INC C */ 0x0C => { cpu.registers.r_c = arithmetic::increment(cpu, cpu.registers.r_c); 1 },
        /* INC D */ 0x14 => { cpu.registers.r_d = arithmetic::increment(cpu, cpu.registers.r_d); 1 },
        /* INC E */ 0x1C => { cpu.registers.r_e = arithmetic::increment(cpu, cpu.registers.r_e); 1 },
        /* INC H */ 0x24 => { cpu.registers.r_h = arithmetic::increment(cpu, cpu.registers.r_h); 1 },
        /* INC L */ 0x2C => { cpu.registers.r_l = arithmetic::increment(cpu, cpu.registers.r_l); 1 },
        /* INC (HL) */ 0x34 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let result_d8 = arithmetic::increment(cpu, d8);
            cpu.write_data(memory, a16_hl, result_d8);
            3
        },
        /* INC A */ 0x3C => { cpu.registers.r_a = arithmetic::increment(cpu, cpu.registers.r_a); 1 },
        /* DEC B */ 0x05 => { cpu.registers.r_b = arithmetic::decrement(cpu, cpu.registers.r_b); 1 },
        /* DEC C */ 0x0D => { cpu.registers.r_c = arithmetic::decrement(cpu, cpu.registers.r_c); 1 },
        /* DEC D */ 0x15 => { cpu.registers.r_d = arithmetic::decrement(cpu, cpu.registers.r_d); 1 },
        /* DEC E */ 0x1D => { cpu.registers.r_e = arithmetic::decrement(cpu, cpu.registers.r_e); 1 },
        /* DEC H */ 0x25 => { cpu.registers.r_h = arithmetic::decrement(cpu, cpu.registers.r_h); 1 },
        /* DEC L */ 0x2D => { cpu.registers.r_l = arithmetic::decrement(cpu, cpu.registers.r_l); 1 },
        /* DEC (HL) */ 0x35 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            let result_d8 = arithmetic::decrement(cpu, d8);
            cpu.write_data(memory, a16_hl, result_d8);
            3
        },
        /* ADD A,B */ 0x80 => { let d8 = cpu.registers.r_b; arithmetic::add(cpu, d8); 2 },
        /* ADD A,C */ 0x81 => { let d8 = cpu.registers.r_c; arithmetic::add(cpu, d8); 2 },
        /* ADD A,D */ 0x82 => { let d8 = cpu.registers.r_d; arithmetic::add(cpu, d8); 2 },
        /* ADD A,E */ 0x83 => { let d8 = cpu.registers.r_e; arithmetic::add(cpu, d8); 2 },
        /* ADD A,H */ 0x84 => { let d8 = cpu.registers.r_h; arithmetic::add(cpu, d8); 2 },
        /* ADD A,L */ 0x85 => { let d8 = cpu.registers.r_l; arithmetic::add(cpu, d8); 2 },
        /* ADD A,(HL) */ 0x86 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            arithmetic::add(cpu, d8);
            2
        },
        /* ADD A,A */ 0x87 => { let d8 = cpu.registers.r_a; arithmetic::add(cpu, d8); 2 },
        /* ADC A,B */ 0x88 => { let d8 = cpu.registers.r_b; arithmetic::add_carry(cpu, d8); 2 },
        /* ADC A,C */ 0x89 => { let d8 = cpu.registers.r_c; arithmetic::add_carry(cpu, d8); 2 },
        /* ADC A,D */ 0x8A => { let d8 = cpu.registers.r_d; arithmetic::add_carry(cpu, d8); 2 },
        /* ADC A,E */ 0x8B => { let d8 = cpu.registers.r_e; arithmetic::add_carry(cpu, d8); 2 },
        /* ADC A,H */ 0x8C => { let d8 = cpu.registers.r_h; arithmetic::add_carry(cpu, d8); 2 },
        /* ADC A,L */ 0x8D => { let d8 = cpu.registers.r_l; arithmetic::add_carry(cpu, d8); 2 },
        /* ADC A,(HL) */ 0x8E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            arithmetic::add_carry(cpu, d8);
            2
        },
        /* ADC A,A */ 0x8F => { let d8 = cpu.registers.r_a; arithmetic::add_carry(cpu, d8); 2 },
        /* SUB B */ 0x90 => { let d8 = cpu.registers.r_b; arithmetic::sub(cpu, d8); 2 },
        /* SUB C */ 0x91 => { let d8 = cpu.registers.r_c; arithmetic::sub(cpu, d8); 2 },
        /* SUB D */ 0x92 => { let d8 = cpu.registers.r_d; arithmetic::sub(cpu, d8); 2 },
        /* SUB E */ 0x93 => { let d8 = cpu.registers.r_e; arithmetic::sub(cpu, d8); 2 },
        /* SUB H */ 0x94 => { let d8 = cpu.registers.r_h; arithmetic::sub(cpu, d8); 2 },
        /* SUB L */ 0x95 => { let d8 = cpu.registers.r_l; arithmetic::sub(cpu, d8); 2 },
        /* SUB (HL) */ 0x96 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            arithmetic::sub(cpu, d8);
            2
        },
        /* SUB A */ 0x97 => { let d8 = cpu.registers.r_a; arithmetic::sub(cpu, d8); 2 },
        /* SBC A,B */ 0x98 => { let d8 = cpu.registers.r_b; arithmetic::sub_carry(cpu, d8); 2 },
        /* SBC A,C */ 0x99 => { let d8 = cpu.registers.r_c; arithmetic::sub_carry(cpu, d8); 2 },
        /* SBC A,D */ 0x9A => { let d8 = cpu.registers.r_d; arithmetic::sub_carry(cpu, d8); 2 },
        /* SBC A,E */ 0x9B => { let d8 = cpu.registers.r_e; arithmetic::sub_carry(cpu, d8); 2 },
        /* SBC A,H */ 0x9C => { let d8 = cpu.registers.r_h; arithmetic::sub_carry(cpu, d8); 2 },
        /* SBC A,L */ 0x9D => { let d8 = cpu.registers.r_l; arithmetic::sub_carry(cpu, d8); 2 },
        /* SBC A,(HL) */ 0x9E => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            arithmetic::sub_carry(cpu, d8);
            2
        },
        /* SBC A,A */ 0x9F => { let d8 = cpu.registers.r_a; arithmetic::sub_carry(cpu, d8); 2 },
        /* AND B */ 0xA0 => { let d8 = cpu.registers.r_b; arithmetic::and(cpu, d8); 2 },
        /* AND C */ 0xA1 => { let d8 = cpu.registers.r_c; arithmetic::and(cpu, d8); 2 },
        /* AND D */ 0xA2 => { let d8 = cpu.registers.r_d; arithmetic::and(cpu, d8); 2 },
        /* AND E */ 0xA3 => { let d8 = cpu.registers.r_e; arithmetic::and(cpu, d8); 2 },
        /* AND H */ 0xA4 => { let d8 = cpu.registers.r_h; arithmetic::and(cpu, d8); 2 },
        /* AND L */ 0xA5 => { let d8 = cpu.registers.r_l; arithmetic::and(cpu, d8); 2 },
        /* AND (HL) */ 0xA6 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            arithmetic::and(cpu, d8);
            2
        },
        /* AND A */ 0xA7 => { let d8 = cpu.registers.r_a; arithmetic::and(cpu, d8); 2 },
        /* XOR B */ 0xA8 => { let d8 = cpu.registers.r_b; arithmetic::xor(cpu, d8); 2 },
        /* XOR C */ 0xA9 => { let d8 = cpu.registers.r_c; arithmetic::xor(cpu, d8); 2 },
        /* XOR D */ 0xAA => { let d8 = cpu.registers.r_d; arithmetic::xor(cpu, d8); 2 },
        /* XOR E */ 0xAB => { let d8 = cpu.registers.r_e; arithmetic::xor(cpu, d8); 2 },
        /* XOR H */ 0xAC => { let d8 = cpu.registers.r_h; arithmetic::xor(cpu, d8); 2 },
        /* XOR L */ 0xAD => { let d8 = cpu.registers.r_l; arithmetic::xor(cpu, d8); 2 },
        /* XOR (HL) */ 0xAE => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            arithmetic::xor(cpu, d8);
            2
        },
        /* XOR A */ 0xAF => { let d8 = cpu.registers.r_a; arithmetic::xor(cpu, d8); 2 },
        /* OR B */ 0xB0 => { let d8 = cpu.registers.r_b; arithmetic::or(cpu, d8); 2 },
        /* OR C */ 0xB1 => { let d8 = cpu.registers.r_c; arithmetic::or(cpu, d8); 2 },
        /* OR D */ 0xB2 => { let d8 = cpu.registers.r_d; arithmetic::or(cpu, d8); 2 },
        /* OR E */ 0xB3 => { let d8 = cpu.registers.r_e; arithmetic::or(cpu, d8); 2 },
        /* OR H */ 0xB4 => { let d8 = cpu.registers.r_h; arithmetic::or(cpu, d8); 2 },
        /* OR L */ 0xB5 => { let d8 = cpu.registers.r_l; arithmetic::or(cpu, d8); 2 },
        /* OR (HL) */ 0xB6 => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            arithmetic::or(cpu, d8);
            2
        },
        /* OR A */ 0xB7 => { let d8 = cpu.registers.r_a; arithmetic::or(cpu, d8); 2 },
        /* CP B */ 0xB8 => { let d8 = cpu.registers.r_b; arithmetic::compare(cpu, d8); 2 },
        /* CP C */ 0xB9 => { let d8 = cpu.registers.r_c; arithmetic::compare(cpu, d8); 2 },
        /* CP D */ 0xBA => { let d8 = cpu.registers.r_d; arithmetic::compare(cpu, d8); 2 },
        /* CP E */ 0xBB => { let d8 = cpu.registers.r_e; arithmetic::compare(cpu, d8); 2 },
        /* CP H */ 0xBC => { let d8 = cpu.registers.r_h; arithmetic::compare(cpu, d8); 2 },
        /* CP L */ 0xBD => { let d8 = cpu.registers.r_l; arithmetic::compare(cpu, d8); 2 },
        /* CP (HL) */ 0xBE => {
            let a16_hl = bit_operations::join_words(cpu.registers.r_h as u16, cpu.registers.r_l as u16, 8);
            let d8 = cpu.fetch_data(memory, a16_hl);
            arithmetic::compare(cpu, d8);
            2
        },
        /* CP A */ 0xBF => { let d8 = cpu.registers.r_a; arithmetic::compare(cpu, d8); 2 },
        /* DEC A */ 0x3D => { cpu.registers.r_a = arithmetic::decrement(cpu, cpu.registers.r_a); 1 },
        /* ADD A,d8 */ 0xC6 => { let d8 = cpu.fetch_operand(memory); arithmetic::add(cpu, d8); 2 },
        /* ADC A,d8 */ 0xCE => { let d8 = cpu.fetch_operand(memory); arithmetic::add_carry(cpu, d8); 2 },
        /* SUB d8 */ 0xD6 => { let d8 = cpu.fetch_operand(memory); arithmetic::sub(cpu, d8); 2 },
        /* SBC A,d8 */ 0xDE => { let d8 = cpu.fetch_operand(memory); arithmetic::sub_carry(cpu, d8); 2 },
        /* AND d8 */ 0xE6 => { let d8 = cpu.fetch_operand(memory); arithmetic::and(cpu, d8); 2 },
        /* XOR d8 */ 0xEE => { let d8 = cpu.fetch_operand(memory); arithmetic::xor(cpu, d8); 2 },
        /* OR d8 */ 0xF6 => { let d8 = cpu.fetch_operand(memory); arithmetic::or(cpu, d8); 2 },
        /* CP d8 */ 0xFE => { let d8 = cpu.fetch_operand(memory); arithmetic::compare(cpu, d8); 2 },
        /* DAA */ 0x27 => {
            // credit to rboy and github.com/mattbruv
            // this instruction is really poorly described throughout the docs I've obtained so far
            // the best doc for this is the "GAME BOY Programming Manual Version 1.1" by Nintendo, Inc.
            let mut register_data = cpu.registers.r_a;
            let mut adjust = if ((cpu.registers.r_f & flags::CARRY) >> 4) == 1 { 0x60 } else { 0x00 };
            if ((cpu.registers.r_f & flags::HALF_CARRY) >> 5) == 1 {
                adjust = adjust | 0x06;
            }
            if ((cpu.registers.r_f & flags::SUBTRACT) >> 6) != 1 {
                if register_data & 0x0F > 0x09 {
                    adjust = adjust | 0x06;
                }
                if register_data > 0x99 {
                    adjust = adjust | 0x60;
                }
                register_data = register_data.wrapping_add(adjust);
            } else {
                register_data = register_data.wrapping_sub(adjust);
            }
            let zero_flag = if register_data == 0 { flags::ZERO } else { flags::RESET };
            let subtract_flag = cpu.registers.r_f & flags::SUBTRACT; // keep its value
            // let half_carry_flag = flags::RESET; -> this is implied; reset
            let carry_flag = if adjust >= 0x60 { flags::CARRY } else { flags::RESET };
            cpu.registers.r_f = (zero_flag | subtract_flag | carry_flag) as u8;
            cpu.registers.r_a = register_data;
            1
        },
        /* CPL */ 0x2F => {
            cpu.registers.r_a = !cpu.registers.r_a;
            let zero_flag = cpu.registers.r_f & flags::ZERO; // keep its value
            let subtract_flag = flags::SUBTRACT; // -> this is set
            let half_carry_flag = flags::HALF_CARRY; // -> this is set
            let carry_flag = cpu.registers.r_f & flags::CARRY; // keep its value
            cpu.registers.r_f = (zero_flag | subtract_flag | half_carry_flag | carry_flag) as u8;
            1
        },
        /* SCF */ 0x37 => {
            let zero_flag = cpu.registers.r_f & flags::ZERO; // keep its value
            // let subtract_flag = flags::RESET; -> this is implied; reset
            // let half_carry_flag = flags::RESET; -> this is implied; reset
            let carry_flag = flags::CARRY; // -> this is set
            cpu.registers.r_f = (zero_flag | carry_flag) as u8;
            1
        },
        /* CCF */ 0x3F => {
            let zero_flag = cpu.registers.r_f & flags::ZERO; // keep its value
            // let subtract_flag = flags::RESET; -> this is implied; reset
            // let half_carry_flag = flags::RESET; -> this is implied; reset
            let carry_flag = if ((cpu.registers.r_f & flags::CARRY) >> 4) == 1 { flags::RESET } else { flags::CARRY };
            cpu.registers.r_f = (zero_flag | carry_flag) as u8;
            1
        },
        /* RLCA */ 0x07 => {
            let (register_data, register_flags) = bit_operations::rotate_left_carry(cpu.registers.r_a, cpu.registers.r_f);
            cpu.registers.r_f = (register_flags & flags::CARRY) as u8;
            cpu.registers.r_a = register_data;
            1
        },
        /* RLA */ 0x17 => {
            let (register_data, register_flags) = bit_operations::rotate_left(cpu.registers.r_a);
            cpu.registers.r_f = (register_flags & flags::CARRY) as u8;
            cpu.registers.r_a = register_data;
            1
        },
        /* RRCA */ 0x0F => {
            let (register_data, register_flags) = bit_operations::rotate_right_carry(cpu.registers.r_a, cpu.registers.r_f);
            cpu.registers.r_f = (register_flags & flags::CARRY) as u8;
            cpu.registers.r_a = register_data;
            1
        },
        /* RRA */ 0x1F => {
            let (register_data, register_flags) = bit_operations::rotate_right(cpu.registers.r_a);
            cpu.registers.r_f = (register_flags & flags::CARRY) as u8;
            cpu.registers.r_a = register_data;
            1
        },
        /* PREFIX CB */ 0xCB => { let opcode = cpu.fetch_operand(memory); instruction_subset::execute(cpu, memory, opcode) },
        _ => panic!("Opcode unknown: ${:02X}", opcode)
    }
}
