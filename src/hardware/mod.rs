#[allow(dead_code)]
const DEBUG_MODE_UNDEFINED: u8 = 0x01;
const DEBUG_MODE_DMG_ONLY: u8 = 0x02;

#[macro_use]
mod debug_macros;

pub mod cartridge_types;
pub mod memory_map;
pub mod memory;
pub mod flags;
pub mod arithmetic;
pub mod bit_operations;
pub mod instruction_subset;
pub mod instruction_set;
pub mod interrupt;
pub mod cpu;
pub mod ppu;
pub mod system;
pub mod disassembler;
pub mod video;
