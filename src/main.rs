extern crate sdl2;

mod emulator;

use emulator::context::EmulatorContext;

pub fn main() {
    let mut emulator = EmulatorContext::new();
    emulator.run();
}
