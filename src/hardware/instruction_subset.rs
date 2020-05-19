use super::cpu::CPU;
use super::memory::Memory;
use super::bit_operations;

/// This function represents the instruction subset executor within the 0xCB prefix.
pub fn execute(cpu: &mut CPU, memory: &mut Memory, opcode: u8) -> usize {
    match opcode {
        _ => panic!("Opcode unknown within prefix 0xCB: ${:02X}", opcode)
    }
}
