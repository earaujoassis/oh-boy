use std::io::prelude::*;
use std::fs::File;
use std::env;

use super::memory_map;
use super::cartridge_types;

pub struct ROM {
    boot_rom_data: Vec<u8>,
    data: Vec<u8>,
    rom_cartridge_type: u8,
    boot_rom_enabled: bool,
}

pub struct RAM {
    data: Vec<u8>,
}

pub struct Memory {
    rom: ROM,
    ram: RAM,
    debug_mode: bool,
}

impl Memory {

    pub fn new(file_path: String) -> Memory {
        let mut boot_rom_file = File::open("./data/DMG_ROM.bin").expect("Could not find BOOT ROM file; aborting");
        let mut boot_rom_buffer: Vec<u8> = Vec::new();
        let mut rom_file;
        let mut rom_buffer: Vec<u8>;
        let ram_buffer: Vec<u8>;
        let rom_cartridge_type: u8;
        let memory_size: usize;
        let rom: ROM;
        let ram: RAM;

        let debug_mode: bool = debug_mode!();

        if std::path::Path::new(&file_path.to_owned()).exists() {
            rom_file = File::open(file_path.to_owned()).expect("Could not find ROM file; aborting");
            rom_buffer = Vec::new();
            rom_file.read_to_end(&mut rom_buffer).expect("Could not load ROM file; aborting");
            rom_cartridge_type = rom_buffer[memory_map::RTC as usize];
        } else {
            rom_buffer = vec![0; 16_384]; // 16kB of Empty ROM
            rom_buffer[memory_map::IROZ as usize] = 0x00; // NOP
            rom_buffer[(memory_map::IROZ + 0x0001) as usize] = 0x10; //
            rom_buffer[(memory_map::IROZ + 0x0002) as usize] = 0x00; // STOP 00
            rom_cartridge_type = cartridge_types::ROM_ONLY;
        }

        debug_system!(format!("Cartridge type: {:#04X}", rom_cartridge_type), debug_mode);

        boot_rom_file.read_to_end(&mut boot_rom_buffer).expect("Could not load BOOT ROM file; aborting");

        // TODO Should I check the ROM size? I don't think the real hardware does that
        // if rom_buffer[memory_map::OSIZ as usize] as usize != rom_buffer.len() {
        //     panic!("ROM Size Mismatch: ${:02X} & ${:02X}",
        //         rom_buffer[memory_map::OSIZ as usize],
        //         rom_buffer.len());
        // }
        memory_size = 0xFFFF as usize;
        ram_buffer = vec![0; memory_size];
        debug_system!(format!("RAM Size: {}", ram_buffer.len()), debug_mode);

        rom = ROM {
            data: rom_buffer,
            boot_rom_data: boot_rom_buffer,
            rom_cartridge_type: rom_cartridge_type,
            boot_rom_enabled: true,
        };

        ram = RAM {
            data: ram_buffer,
        };

        Memory {
            rom: rom,
            ram: ram,
            debug_mode: debug_mode,
        }
    }

    #[allow(unreachable_patterns)]
    pub fn fetch(&mut self, address: u16) -> u8 {
        debug_system!(format!("memory[{:#06X}]", address), self.debug_mode);
        match address {
            // Internal / BOOT ROM (if enabled; external ROM otherwise)
            memory_map::IROM..=memory_map::IROX => {
                match self.rom.boot_rom_enabled {
                    true => self.rom.boot_rom_data[address as usize],
                    _ =>    self.rom.data[address as usize],
                }
            },
            // Non-switchable ROM Bank #0 (always accessible)
            memory_map::IROZ..=memory_map::ROM9 => self.rom.data[address as usize],
            // Switchable / External ROM Bank
            memory_map::RB0 ..=memory_map::RB9  => 0xFF,
            // VRAM
            memory_map::VR0 ..=memory_map::VR9  => self.ram.data[address as usize],
            // Switchable / External RAM Bank
            memory_map::SWR0..=memory_map::SWR9 => 0xFF,
            // Internal (Work) RAM
            memory_map::WR0 ..=memory_map::WR9  => self.ram.data[address as usize],
            // ECHO of RAM
            memory_map::ER0 ..=memory_map::ER9  =>
                self.ram.data[(address - (memory_map::ER0 - memory_map::WR0)) as usize],
            // Object Attribute Memory (OAM)
            memory_map::OAM0..=memory_map::OAM9  => self.ram.data[address as usize],
            // Un-used High RAM Area
            memory_map::RAM0..=memory_map::URAM => 0xFF,
            // Usable High RAM Area
            memory_map::HRAM..=memory_map::RAM9 => self.ram.data[address as usize],
            // This is by definition unreachable, since the address (u16) maximum value is 0xFFFF
            _ => panic!("Unreachable area: ${:#02X}", address)
        }
    }

    #[allow(unreachable_patterns)]
    pub fn write(&mut self, address: u16, word: u8) {
        debug_system!(format!("memory[{:#06X}]={:#04X}", address, word), self.debug_mode);
        match address {
            // Internal / BOOT ROM (if enabled; external ROM otherwise)
            memory_map::IROM..=memory_map::IROX => {},
            // Non-switchable ROM Bank #0 (always accessible)
            memory_map::IROZ..=memory_map::ROM9 => {},
            // Switchable / External ROM Bank
            memory_map::RB0 ..=memory_map::RB9  => {},
            // VRAM
            memory_map::VR0 ..=memory_map::VR9  => self.ram.data[address as usize] = word,
            // Switchable / External RAM Bank
            memory_map::SWR0..=memory_map::SWR9 => {},
            // Internal (Work) RAM
            memory_map::WR0 ..=memory_map::WR9  => self.ram.data[address as usize] = word,
            // ECHO of RAM
            memory_map::ER0 ..=memory_map::ER9  => {
                self.ram.data[(address - (memory_map::ER0 - memory_map::WR0)) as usize] = word;
            },
            // Object Attribute Memory (OAM)
            memory_map::OAM0..=memory_map::OAM9  => self.ram.data[address as usize] = word,
            // Un-used High RAM Area
            memory_map::RAM0..=memory_map::URAM => {},
            // Usable High RAM Area
            memory_map::HRAM..=memory_map::RAM9 => self.ram.data[address as usize] = word,
            // This is by definition unreachable, since the address (u16) maximum value is 0xFFFF
            _ => panic!("Unreachable area: ${:#02X}", address)
        }
    }

}
