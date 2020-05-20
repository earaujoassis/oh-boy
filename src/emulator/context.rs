use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::time::{Duration, Instant};
use std::thread;

use ::hardware::system::System;

const CLOCK_SPEED      : i32 = 4194304; // 4.194304 MHz
const FRAME_RATE       : i32 = 59; // 59.727500569606 Hz
const CYCLES_PER_FRAME : u16 = (CLOCK_SPEED / FRAME_RATE) as u16;

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
        let window = video_subsystem.window(&title, 320, 288)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
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
        let mut i = 0;
        let frame_time = Duration::new(0, 16600000);

        self.hardware.boot();
        'running: loop {
            let start_time = Instant::now();
            let mut emulated_cycles: u16 = 0;
            while emulated_cycles < CYCLES_PER_FRAME {
                emulated_cycles += self.hardware.cycle() as u16;
            }
            let elapsed_time = start_time.elapsed();
            if elapsed_time < frame_time {
                let remaining_time = frame_time - elapsed_time;
                thread::sleep(remaining_time);
            }

            i = (i + 1) % 255;
            self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            self.canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            // Emulator loop goes here...

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

}
