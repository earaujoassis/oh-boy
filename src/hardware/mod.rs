macro_rules! debug_system {
    ($formatted_data:expr, $debug_mode:expr) => {
        if ($debug_mode) {
            println!("{}", $formatted_data);
        }
    };
}

macro_rules! debug_mode {
    () => {
        match env::var("DEBUG") {
            Ok(_value) => true,
            Err(_error) => false,
        };
    }
}

pub mod cartridge_types;
pub mod memory_map;
pub mod memory;
pub mod flags;
pub mod arithmetic;
pub mod bit_operations;
pub mod instruction_subset;
pub mod instruction_set;
pub mod cpu;
pub mod ppu;
pub mod system;
pub mod disassembler;
