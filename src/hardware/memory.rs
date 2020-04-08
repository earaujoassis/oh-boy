use std::io::prelude::*;
use std::fs::File;

use super::memory_map;

pub struct ROM {
    file_path: String,
    data: Vec<u8>,
    rom_cartridge_type: u8,
}

pub struct RAM {
    data: Vec<u8>,
}

pub struct Memory {
    rom: ROM,
    ram: RAM,
}

impl Memory {

    pub fn new(file_path: String) -> Memory {
        let mut rom_file = File::open(file_path.to_owned()).expect("Could not find ROM file; aborting");
        let mut rom_buffer: Vec<u8> = Vec::new();
        let rom: ROM;
        let ram: RAM;
        let rom_cartridge_type: u8;
        let memory_size: usize;

        rom_file.read_to_end(&mut rom_buffer).expect("Could not load ROM file; aborting");
        rom_cartridge_type = rom_buffer[memory_map::RTC as usize];
        if rom_cartridge_type != 0x00 {
            panic!("Unsupported ROM Cartridge Type: ${:02X}", rom_cartridge_type);
        }
        // TODO Should I check the ROM size? I don't think the real hardware does that
        // if rom_buffer[memory_map::OSIZ as usize] as usize != rom_buffer.len() {
        //     panic!("ROM Size Mismatch: ${:02X} & ${:02X}",
        //         rom_buffer[memory_map::OSIZ as usize],
        //         rom_buffer.len());
        // }
        memory_size = 0xFFFF - rom_buffer.len();

        rom = ROM {
            data: rom_buffer,
            file_path: file_path.to_owned(),
            rom_cartridge_type: rom_cartridge_type,
        };

        ram = RAM {
            data: Vec::with_capacity(memory_size)
        };

        Memory {
            rom: rom,
            ram: ram,
        }
    }

    pub fn fetch(&mut self, address: u16) -> u8 {
        match self.rom.rom_cartridge_type {
            0x00 => {
                match address {
                    memory_map::ROM0..=memory_map::ROM9 => self.rom.data[address as usize],
                    _ => self.ram.data[address as usize],
                }
            },
            _ => panic!("Unsupported ROM Cartridge Type: ${:02X}", self.rom.rom_cartridge_type)
        }
    }

    pub fn write(&mut self, address: u16, word: u8) {
        match self.rom.rom_cartridge_type {
            0x00 => {},
            _ => panic!("Unsupported ROM Cartridge Type: ${:02X}", self.rom.rom_cartridge_type)
        }
    }

}
