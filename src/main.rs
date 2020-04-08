#![allow(dead_code)]

extern crate sdl2;

mod emulator;
mod hardware;

use std::env;
use emulator::context::EmulatorContext;

pub fn main() {
    let mut args = env::args();
	let rom_file_path = args.nth(1).expect("ROM filepath is not available; aborting");
    let mut emulator = EmulatorContext::new(rom_file_path.to_owned());
    emulator.run();
}
