use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::{Rect, Point};
use sdl2::Sdl;

use std::time::{Duration, Instant};
use std::thread;

use ::hardware::system::System;

const CLOCK_SPEED      : i32 = 4194304; // 4.194304 MHz
const FRAME_RATE       : i32 = 60; // 59.727500569606 Hz
const CYCLES_PER_FRAME : u32 = (CLOCK_SPEED / FRAME_RATE) as u32;

pub struct EmulatorContext {
    context: Sdl,
    canvas: WindowCanvas,
    hardware: System,
}

impl EmulatorContext {

    pub fn new(file_path: String) -> EmulatorContext {
        let title = "GameBoy Emulator".to_owned();
        let system = System::new(file_path.to_owned());
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window(&title, 160, 144)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        EmulatorContext {
            context: context,
            canvas: canvas,
            hardware: system,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.context.event_pump().unwrap();
        let frame_time = Duration::from_nanos(16_750_418);

        self.hardware.boot();
        'running: loop {
            let start_time = Instant::now();
            let mut emulated_cycles: u32 = 0;
            let elapsed_time;

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            while emulated_cycles < CYCLES_PER_FRAME {
                emulated_cycles += self.hardware.cycle() as u32;
                if self.hardware.has_stopped() {
                    break 'running
                }
            }

            elapsed_time = start_time.elapsed();
            if elapsed_time < frame_time {
                let remaining_time = frame_time - elapsed_time;
                thread::sleep(remaining_time);
            }

            self.update_canvas();
        }

        self.hardware.dump();
    }

    #[allow(unused_must_use)]
    pub fn update_canvas(&mut self) {
        let buffer: Vec<u8> = self.hardware.video_buffer();
        let capacity = buffer.capacity();
        let mut i = 0;

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        self.canvas.clear();

        while i < capacity {
            let color = match buffer[i] {
                0x01 => Color::RGB(255, 255, 255),
                0x02 => Color::RGB(198, 198, 198),
                0x04 => Color::RGB(127, 127, 127),
                0x08 => Color::RGB(127, 127, 127),
                _    => Color::RGBA(255, 255, 255, 0),
            };
            let x = i / 144;
            let y = i % 144;

            self.canvas.set_draw_color(color);
            self.canvas.fill_rect(Rect::from_center(Point::new(x as i32 + 4, y as i32 + 4), 8, 8));

            i += 1;
        }

        self.canvas.present();
    }

}
