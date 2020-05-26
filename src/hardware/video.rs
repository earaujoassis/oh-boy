use super::memory::Memory;
use super::memory_map;
use super::bit_operations;

pub const FRAME_WIDTH        : usize = 160;
pub const FRAME_HEIGHT       : usize = 144;
pub const FRAME_HEIGHT_FULL  : usize = 154;

#[allow(dead_code)]
const BG_DIMENSION       : u16 = 256; // background dimension
const BG_BLOCKS_PER_AXIS : u16 = 32;
const TILE_DIMENSION     : u8 = 8;

const CODE_AREA0         : u16 = 0x9800; // 9800h-9BFFh
const CODE_AREA1         : u16 = 0x9C00; // 9C00h-9FFFh

const CHARACTER_DATA0    : u16 = 0x8800; // 8800h-97FFh
const CHARACTER_DATA1    : u16 = 0x8000; // 8000h-8FFFh

const TILE_ROW_PIXELS    : u16 = 16; // (or 2 bytes of data for each tile row)
const TILE_ROW_OFFSET    : u16 = 2;

pub struct Video {
    pub frame_buffer: Vec<u8>,
    pub scanline_pixels_rendered: usize,
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
        let current_lcdc: u8 = memory.fetch(memory_map::LCDC);
        let updates: u8 = current_lcdc & 0x01;

        let current_ly = memory.fetch(memory_map::LY);

        match updates {
            0x01 => self.update_background_actor(memory),
            _ => {},
        };

        memory.write(memory_map::LY, current_ly + 1)
    }

    fn update_background_actor(&mut self, memory: &mut Memory) {
        let ly: u8 = memory.fetch(memory_map::LY);
        let lcdc: u8 = memory.fetch(memory_map::LCDC);
        let scy: u8 = memory.fetch(memory_map::SCY);
        let scx: u8 = memory.fetch(memory_map::SCX);
        let palette: u8 = memory.fetch(memory_map::BGP);

        let mut pixels_rendered: usize = 0;

        // We walk through BG blocks instead of each single pixel
        let y: u8 = (scy + ly) / TILE_DIMENSION;
        for x in (scx / TILE_DIMENSION)..(scx / TILE_DIMENSION + 20) {
            // Since the BG may slide beyond its edges, we must sanitize the x,y values
            let local_x = if x > BG_BLOCKS_PER_AXIS as u8 { x - BG_BLOCKS_PER_AXIS as u8 } else { x };
            let local_y = if y > BG_BLOCKS_PER_AXIS as u8 { y - BG_BLOCKS_PER_AXIS as u8 } else { y };
            let block_id: u16 = ((local_y as u16 * BG_BLOCKS_PER_AXIS as u16) + local_x as u16) as u16;
            // Check the "BG Code Area Selection Flag" and obtain the BG Character Code
            let character_code: u8 = match bit_operations::simple_bit(lcdc, 3) {
                true  => memory.fetch(CODE_AREA1 + block_id),
                false => memory.fetch(CODE_AREA0 + block_id),
            };

            // Obtain the tile row (y-axis) to render
            let tile_row = (scy + ly) % TILE_DIMENSION;
            // Each BG Character Data (8x8 pixels Tile) is stored in 16 bytes, each 2 bytes representing a row
            // (the `TILE_ROW_PIXELS`). The "BG Character Data Selection Flag" establish the Character Data
            // Bank to obtain each word (each byte).
            let character_data_offset: u16 = match bit_operations::simple_bit(lcdc, 4) {
                true  => (CHARACTER_DATA1 + (character_code as u16 * TILE_ROW_PIXELS)),
                false => ((CHARACTER_DATA0 + 0x800) as i32 + (character_code as i32 * TILE_ROW_PIXELS as i32)) as u16,
            };
            let lsb: u8 = memory.fetch(character_data_offset + (tile_row as u16 * TILE_ROW_OFFSET) as u16);
            let msb: u8 = memory.fetch(character_data_offset + (tile_row as u16 * TILE_ROW_OFFSET) as u16 + 1);

            let tile_column: u8 = (pixels_rendered as u8 + scx) % TILE_DIMENSION;
            // Render all the bits for the specific tile (x-axis) based on the previously selected row (y-axis)
            for column in (tile_column as u8)..8_u8 {
                let color = pixel_color(palette, lsb, msb, 7 - column);
                self.frame_buffer[(ly as usize * FRAME_WIDTH + pixels_rendered)] = color;
                pixels_rendered += 1;
            }
        }
    }

}

fn pixel_color(palette: u8, lsb: u8, msb: u8, pixel_x: u8) -> u8 {
    let shade_0 = palette & 0x03;
    let shade_1 = (palette >> 2) & 0x03;
    let shade_2 = (palette >> 4) & 0x03;
    let shade_3 = palette >> 6;

    let bit0 = if bit_operations::simple_bit(lsb, pixel_x as usize) { 0x01 } else { 0x00 } as u8;
    let bit1 = if bit_operations::simple_bit(msb, pixel_x as usize) { 0x02 } else { 0x00 } as u8;

    match bit0 | bit1 {
        0x00 => shade_0,
        0x01 => shade_1,
        0x02 => shade_2,
        0x03 => shade_3,
        _    => 0xFF,
    }
}
