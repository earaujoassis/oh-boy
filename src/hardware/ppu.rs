/// The Pixel (or Picture) Processing Unit

use super::memory::Memory;
use super::memory_map;
use super::video::{Video, Mode, Frame};
use super::interrupt::{Flag as InterruptFlag};

const MODE0_THRESHOLD: usize = 51;   // cycles per mode; 204 div 4
const MODE1_THRESHOLD: usize = 1140; // 4560 div 4
const MODE2_THRESHOLD: usize = 20;   // 80 div 4
const MODE3_THRESHOLD: usize = 43;   // 172 div 4

const VBLANK_LINES:           usize = 10;
const VBLANK_CYCLES_PER_LINE: usize = MODE1_THRESHOLD / VBLANK_LINES;

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
    //    Mode 10  x_____x_____x_____x_____x_____x_________________
    //    Mode 11  _xx____xx____xx____xx____xx____xx_______________
    //    Mode 00  ___xxx___xxx___xxx___xxx___xxx___xxx____________
    //    Mode 01  ____________________________________xxxxxxxxxxxx
    //
    pub fn cycle(&mut self, memory: &mut Memory, cycles: usize) {
        // Check if the display is enabled from the LCDC flag (at RAM)
        if (self.fetch_data(memory, memory_map::LCDC) & 0x80) == 0x00 {
            return;
        }

        let current_stat = self.fetch_data(memory, memory_map::STAT);
        let mut current_ly = self.fetch_data(memory, memory_map::LY);
        let current_lyc = self.fetch_data(memory, memory_map::LYC);
        let current_mode = current_stat & 0x03; // last 2 bits

        let mut next_mode: u8 = current_mode;
        let mut next_stat: u8 = current_stat;
        let mut must_request_interrupt: bool = false;

        self.accumulated_cycles += cycles;

        if current_ly >= Frame::HEIGHT as u8 && current_ly <= Frame::HEIGHT_FULL as u8 {
            // This is Mode 01
            next_mode = Mode::VBLANK as u8;     // 1
            let clean_stat = (!(Mode::SEARCH_OAM as u8 | Mode::HBLANK as u8 | Mode::VBLANK as u8) & current_stat) & 0xFF;
            next_stat = (clean_stat | (Mode::VBLANK as u8)) & 0xFF;
            // Check if we're at VBLANK for the first time
            if current_mode != next_mode {
                self.accumulated_cycles = 0;
            }
            // If bit #4 at STAT is set, request interrupt
            must_request_interrupt = (current_stat & 0x10) > 0;
            let ly_checker = (self.accumulated_cycles / VBLANK_CYCLES_PER_LINE) as u8;
            if current_ly - Frame::HEIGHT as u8 != ly_checker {
                current_ly = Frame::HEIGHT as u8 + ly_checker;
                memory.write(memory_map::LY, current_ly);
            }
        } else if self.accumulated_cycles > MODE1_THRESHOLD {
            // Request Mode 10 next time
            current_ly = 0x00;
            memory.write(memory_map::LY, current_ly);
        }

        if self.accumulated_cycles < MODE2_THRESHOLD
                && current_ly < Frame::HEIGHT as u8 {
            // This is Mode 10
            next_mode = Mode::SEARCH_OAM as u8; // 2
            next_stat = ((!(Mode::VBLANK as u8 | Mode::SCANLINE as u8) & current_stat) | (Mode::SEARCH_OAM as u8)) & 0xFF;
            // If bit #5 at STAT is set, request interrupt
            must_request_interrupt = (current_stat & 0x20) > 0;
        } else if self.accumulated_cycles > MODE2_THRESHOLD
                && self.accumulated_cycles < MODE2_THRESHOLD + MODE3_THRESHOLD
                && current_ly < Frame::HEIGHT as u8 {
            // This is Mode 11
            next_mode = Mode::SCANLINE as u8;   // 3
            next_stat = (current_stat | (Mode::SCANLINE as u8)) & 0xFF;
        } else if self.accumulated_cycles > MODE2_THRESHOLD + MODE3_THRESHOLD
                && self.accumulated_cycles < MODE0_THRESHOLD + MODE2_THRESHOLD + MODE3_THRESHOLD
                && current_ly < Frame::HEIGHT as u8 {
            // This is Mode 00
            next_mode = Mode::HBLANK as u8;     // 0
            // Check if we're at HBLANK for the first time
            if next_mode != current_mode {
                // At HBLANK all transfers (the whole scanline) have been completed;
                // so it is a good moment for the emulator to update buffer
                self.video.update_scanline(memory);
                current_ly = self.fetch_data(memory, memory_map::LY);
            }
            next_stat = (!(Mode::SCANLINE as u8) & current_stat) & 0xFF;
            // If bit #3 at STAT is set, request interrupt
            must_request_interrupt = (current_stat & 0x08) > 0;
        } else if self.accumulated_cycles >  MODE0_THRESHOLD + MODE2_THRESHOLD + MODE3_THRESHOLD
                && current_ly < Frame::HEIGHT as u8 {
            // Request Mode 10 next time
            self.accumulated_cycles = 0;
        }

        if must_request_interrupt && (next_mode != current_mode) {
            request_interrupt(self, memory, InterruptFlag::LCDC);
        }

        if current_ly == current_lyc {
            next_stat = next_stat | 0x04;
            if (next_stat & 0x40) > 0 {
                request_interrupt(self, memory, InterruptFlag::LCDC);
            }
        } else {
            next_stat = next_stat & !(0x40 as u8);
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
