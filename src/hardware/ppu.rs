/// The Pixel (or Picture) Processing Unit

use super::memory::Memory;
use super::memory_map;
use super::video::{Video, Mode, Frame};
use super::interrupt::{Flag as InterruptFlag};
use super::bit_operations;

const MODE00_THRESHOLD: usize = 51;   // 204 div 4  -> HBLANK threshold
const MODE01_THRESHOLD: usize = 1140; // 4560 div 4 -> VBLANK threshold
const MODE10_THRESHOLD: usize = 20;   // 80 div 4   -> SEARCH_OAM threshold
const MODE11_THRESHOLD: usize = 43;   // 172 div 4  -> SCANLINE threshold

const VBLANK_LINES:           usize = 10;
const VBLANK_CYCLES_PER_LINE: usize = MODE01_THRESHOLD / VBLANK_LINES;

pub struct PPURegisters {
    // Memory Address Register (MAR)
    address_register: u16,
    // Memory Data Register (MDR)
    data_register: u8,
}

pub struct PPU {
    pub registers: PPURegisters,
    pub video: Video,
    pub accumulated_cycles: usize,
    pub update_buffer: bool,
    pub debug_mode: bool,
}

impl PPU {

    pub fn new() -> PPU {
        let registers = PPURegisters {
            address_register: 0x0000,
            data_register: 0x00,
        };
        let video = Video::new();
        let debug_mode: bool = debug_mode!();

        PPU {
            registers: registers,
            video: video,
            accumulated_cycles: 0,
            update_buffer: true,
            debug_mode: debug_mode,
        }
    }

    // The `cycle` function uses the CPU cycles to select the right Mode.
    // Basically, it cycles through the following Modes
    //     ((10 -> 11 -> 00)+ -> 11 ->)+
    // It attempts to mimic the following diagram between Modes:
    //
    //    Mode 10  x_____x_____x_____x_____x_____x_________________  ->  Mode::SEARCH_OAM
    //    Mode 11  _xx____xx____xx____xx____xx____xx_______________  ->  Mode::SCANLINE
    //    Mode 00  ___xxx___xxx___xxx___xxx___xxx___xxx____________  ->  Mode::HBLANK
    //    Mode 01  ____________________________________xxxxxxxxxxxx  ->  Mode::VBLANK
    //
    pub fn cycle(&mut self, memory: &mut Memory, cycles: usize) {
        // Check if the display is enabled from the LCDC flag (at RAM)
        if (self.fetch_data(memory, memory_map::LCDC) & 0x80) == 0x00 {
            return;
        }

        let current_stat: u8 = self.fetch_data(memory, memory_map::STAT);
        let clean_stat: u8 = 0xFC & current_stat; // 0b11111100 & current_stat
        let mut current_ly: u8 = self.fetch_data(memory, memory_map::LY);
        let current_lyc: u8 = self.fetch_data(memory, memory_map::LYC);
        let current_mode: u8 = current_stat & 0x03; // last 2 bits

        let mut next_mode: u8 = current_mode;
        let mut next_stat: u8 = current_stat;
        let mut must_request_interrupt: bool = false;

        self.accumulated_cycles += cycles;

        match current_mode {
            // This handle Mode 01 | Mode VBLANK
            current_mode if current_mode == Mode::VBLANK as u8 => {
                if self.accumulated_cycles > MODE01_THRESHOLD || current_ly >= Frame::HEIGHT_FULL as u8 {
                    // Request SEARCH_OAM
                    next_mode = Mode::SEARCH_OAM as u8;
                    next_stat = (clean_stat | (Mode::SEARCH_OAM as u8)) & 0xFF;
                    self.accumulated_cycles = 0;
                    memory.write(memory_map::LY, 0x00);
                } else {
                    // We must update the LY inside the VBLANK according to the VBLANK step
                    let ly_checker = (self.accumulated_cycles / VBLANK_CYCLES_PER_LINE) as u8;
                    current_ly = Frame::HEIGHT as u8 + ly_checker;
                    memory.write(memory_map::LY, current_ly);
                }
            },
            // This handle Mode 10 | Mode SEARCH_OAM
            current_mode if current_mode == Mode::SEARCH_OAM as u8 => {
                if self.accumulated_cycles > MODE10_THRESHOLD {
                    // Request SCANLINE
                    next_mode = Mode::SCANLINE as u8;
                    next_stat = (clean_stat | (Mode::SCANLINE as u8)) & 0xFF;
                    self.accumulated_cycles = 0;
                }
            },
            // This handle Mode 11 | Mode SCANLINE
            current_mode if current_mode == Mode::SCANLINE as u8 => {
                if self.accumulated_cycles > MODE11_THRESHOLD {
                    // Request HBLANK
                    next_mode = Mode::HBLANK as u8;
                    next_stat = (clean_stat | (Mode::HBLANK as u8)) & 0xFF;
                    self.accumulated_cycles = 0;
                    // It is the end of the scanline, so we can request to update the whole line
                    self.video.update_scanline(memory);
                    current_ly = self.fetch_data(memory, memory_map::LY);
                    // If bit #3 at STAT is set, request interrupt
                    must_request_interrupt = bit_operations::simple_bit(next_stat, 3);
                }
            },
            // This handle Mode 00 | Mode HBLANK
            current_mode if current_mode == Mode::HBLANK as u8 => {
                if self.accumulated_cycles > MODE00_THRESHOLD {
                    if current_ly >= Frame::HEIGHT as u8 {
                        // Request VBLANK
                        next_mode = Mode::VBLANK as u8;
                        next_stat = (clean_stat | (Mode::VBLANK as u8)) & 0xFF;
                        // If bit #4 at STAT is set, request interrupt
                        must_request_interrupt = bit_operations::simple_bit(next_stat, 4);
                        request_interrupt(self, memory, InterruptFlag::VBLANK);
                    } else {
                        // Request SEARCH_OAM
                        next_mode = Mode::SEARCH_OAM as u8;
                        next_stat = (clean_stat | (Mode::SEARCH_OAM as u8)) & 0xFF;
                        // If bit #5 at STAT is set, request interrupt
                        must_request_interrupt = bit_operations::simple_bit(next_stat, 5);
                    }
                    self.accumulated_cycles = 0;
                }
            },
            _ => panic!("Oops!... there's a bug at the PPU; mode: {:#04X}", current_mode),
        }

        debug_system!(format!("PPU   : MODE={:#04X} NEXT-MODE={:#04X} LY={:#04X}\n",
            current_mode, next_mode, current_ly),
                self.debug_mode);

        if must_request_interrupt && (next_mode != current_mode) {
            request_interrupt(self, memory, InterruptFlag::LCDC);
        }

        if current_ly == current_lyc {
            next_stat = next_stat | 0x04; // Set Match Flag (LYC = LCDCLY)
            if bit_operations::simple_bit(next_stat, 6) {
                request_interrupt(self, memory, InterruptFlag::LCDC);
            }
        } else {
            next_stat = next_stat & !(0x40 as u8); // Reset Match Flag (LYC != LCDCLY)
        }

        self.write_data(memory, memory_map::STAT, next_stat);
    }

    pub fn fetch_data(&mut self, memory: &mut Memory, address: u16) -> u8 {
        self.registers.address_register = address;
        self.registers.data_register = memory.fetch(self.registers.address_register);
        self.registers.data_register
    }

    pub fn write_data(&mut self, memory: &mut Memory, address: u16, word: u8) {
        self.registers.address_register = address;
        self.registers.data_register = word;
        memory.write(address, word);
    }

}

fn request_interrupt(ppu: &mut PPU, memory: &mut Memory, flag: InterruptFlag) {
    let interrupt_request: u8 = ppu.fetch_data(memory, memory_map::IF) as u8 | (flag as u8);
    ppu.write_data(memory, memory_map::IF, interrupt_request);
}
