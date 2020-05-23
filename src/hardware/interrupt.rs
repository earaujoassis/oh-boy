/// Interruption handler

use super::cpu::CPU;
use super::memory::Memory;
use super::memory_map;
use super::bit_operations;

pub enum Flag {
    VBLANK = 0x01,
    LCDC   = 0x02,
    TIMER  = 0x04,
    SERIAL = 0x08,
    P10P13 = 0x10,
}

enum HandlerAddress {
    VBLANK = 0x0040,
    LCDC   = 0x0048,
    TIMER  = 0x0050,
    SERIAL = 0x0058,
    P10P13 = 0x0060,
}

pub fn handler(cpu: &mut CPU, memory: &mut Memory) {
    let any_flags = (Flag::VBLANK as u8 | Flag::LCDC as u8 | Flag::TIMER as u8 | Flag::SERIAL as u8 | Flag::P10P13 as u8) as u8;
    let interrupt_enable: u8 = cpu.fetch_data(memory, memory_map::IE);
    let interrupt_request: u8 = cpu.fetch_data(memory, memory_map::IF);

    if cpu.interruption_enabled && (interrupt_enable & any_flags) > 0 {
        // TODO check if the interrupt is enabled (IE at 0xFFFF) and if the interrupt is available (IF at 0xFF0F)
        let handler_address: HandlerAddress = match interrupt_request {
            interrupt_request if (Flag::VBLANK as u8 & interrupt_request) > 0 => {  // priority 0
                let handle_request = (!(Flag::VBLANK as u8) & interrupt_request) & 0xFF;
                cpu.write_data(memory, memory_map::IF, handle_request);
                HandlerAddress::VBLANK
            },
            interrupt_request if (Flag::LCDC as u8   & interrupt_request) > 0 => {  // priority 1
                let handle_request = (!(Flag::LCDC as u8) & interrupt_request) & 0xFF;
                cpu.write_data(memory, memory_map::IF, handle_request);
                HandlerAddress::LCDC
            },
            interrupt_request if (Flag::TIMER as u8  & interrupt_request) > 0 => {  // priority 2
                let handle_request = (!(Flag::TIMER as u8) & interrupt_request) & 0xFF;
                cpu.write_data(memory, memory_map::IF, handle_request);
                HandlerAddress::TIMER
            },
            interrupt_request if (Flag::SERIAL as u8 & interrupt_request) > 0 => {  // priority 3
                let handle_request = (!(Flag::SERIAL as u8) & interrupt_request) & 0xFF;
                cpu.write_data(memory, memory_map::IF, handle_request);
                HandlerAddress::SERIAL
            },
            interrupt_request if (Flag::P10P13 as u8 & interrupt_request) > 0 => {  // priority 4
                let handle_request = (!(Flag::P10P13 as u8) & interrupt_request) & 0xFF;
                cpu.write_data(memory, memory_map::IF, handle_request);
                HandlerAddress::P10P13
            },
            _ => panic!("Could not infer interruption flag type"),
        };
        cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
        cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::msb(cpu.registers.program_counter, 8));
        cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
        cpu.write_data(memory, cpu.registers.stack_pointer, bit_operations::lsb(cpu.registers.program_counter, 8));
        cpu.registers.program_counter = handler_address as u16;
        cpu.halted = false;
        cpu.interruption_enabled = false;
    }
}
