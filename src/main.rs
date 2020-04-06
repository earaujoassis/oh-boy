extern crate sdl2;

use std::env;

fn main() {
    let mut args = env::args();
    let cartridge_path = args.nth(1).expect("No cartridge provided; aborting");

    println!("{}", cartridge_path);
}
