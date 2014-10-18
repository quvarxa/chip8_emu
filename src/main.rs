#![feature(if_let)]

extern crate native;
extern crate sdl2;

use std::os;
use std::io::File;

mod chip8;
mod timer;
mod client;

fn main() {
    let args = os::args();
    let mut file = match File::open(&Path::new(args[1].as_slice())) {
        Ok(f) => f,
        Err(err) => fail!("Failed to open input program: {}", err)
    };

    let mut emulator = chip8::Emulator::new();
    match file.read(emulator.mem.ram) {
        Ok(n) => println!("Loaded program of size: {}", n),
        Err(err) => fail!("Failed to read file: {}", err)
    }

    client::run(emulator);
}
