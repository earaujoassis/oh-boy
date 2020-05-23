use super::memory::Memory;
use super::memory_map;
use super::bit_operations;

pub const FRAME_WIDTH             : usize = 160;
pub const FRAME_HEIGHT            : usize = 144;
pub const FRAME_HEIGHT_FULL       : usize = 154;
pub const BG_DIMENSION            : usize = 256; // background dimension
pub const BG_BLOCK_SIZE           : usize = 32;
pub const TILE_DIMENSION          : usize = 8;

pub struct Video {
    pub frame_buffer: Vec<u8>,
    pub scanline_pixels_rendered: usize,
}

#[allow(non_camel_case_types)]
pub enum Mode {
    HBLANK     = 0x00,
    VBLANK     = 0x01,
    SEARCH_OAM  = 0x02,
    SCANLINE   = 0x03,
}

#[allow(non_camel_case_types)]
pub enum Frame {
    WIDTH       = FRAME_WIDTH as isize,
    HEIGHT      = FRAME_HEIGHT as isize,
    HEIGHT_FULL = FRAME_HEIGHT_FULL as isize,
}

impl Video {

    pub fn new() -> Video {
        let frame_buffer = vec![0; FRAME_WIDTH * FRAME_HEIGHT];

        Video {
            frame_buffer: frame_buffer,
            scanline_pixels_rendered: 0,
        }
    }

    pub fn update_scanline(&mut self, memory: &mut Memory) {
        let current_lcdc = memory.fetch(memory_map::LCDC);
        let updates: u8 = current_lcdc & 0x01;

        let current_ly = memory.fetch(memory_map::LY);
        memory.write(memory_map::LY, current_ly + 1);

        match updates {
            0x01 => self.update_background_actor(memory),
            _ => {},
        }
    }

    fn update_background_actor(&mut self, memory: &mut Memory) {
        // IMPLEMENT
    }

}

fn pixel_color(palette: u8, lsb: u8, msb: u8, bit: u8) -> u8 {
    let shade_0 = palette & 0x03;
    let shade_1 = (palette >> 2) & 0x03;
    let shade_2 = (palette >> 4) & 0x03;
    let shade_3 = palette >> 6;

    let bit0 = if bit_operations::simple_bit(lsb, bit as usize) { 0x01 } else { 0x00 } as u8;
    let bit1 = if bit_operations::simple_bit(msb, bit as usize) { 0x02 } else { 0x00 } as u8;

    match bit0 | bit1 {
        0x00 => matching_palette(shade_0),
        0x01 => matching_palette(shade_1),
        0x02 => matching_palette(shade_2),
        0x03 => matching_palette(shade_3),
        _    => 0xFF,
    }
}

fn matching_palette(color: u8) -> u8 {
    match color {
        0x00 => 0x01,
        0x01 => 0x02,
        0x02 => 0x04,
        0x03 => 0x08,
        _    => 0xFF,
    }
}
