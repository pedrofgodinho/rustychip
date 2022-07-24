use std::fs;
use rustychip::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The rom file to open
    #[clap(value_parser)]
    rom: String,

    /// Whether to emulate the behaviour of the original chip8 and set vx register to vy and shift it instead of shifting vy in place. Will likely break some roms
    #[clap(short, long, value_parser, default_value_t = false)]
    shift_sets_vx: bool,

    /// Whether to emulate the bug present in CHIP-48 and SUPER-CHIP related to the jump with offset instruction. Will likely break some roms
    #[clap(short, long, value_parser, default_value_t = false)]
    jump_with_offset_bug_emulation: bool,


    /// Whether to emulate the behaviour of the original chip8 and increment the I register when storing or loading from memory. Will likely break some roms
    #[clap(short, long, value_parser, default_value_t = false)]
    increment_i_on_store_and_load: bool,
}

fn main() {
    let args = Args::parse();
    let rom = match fs::read(args.rom) {
        Ok(rom) => rom,
        Err(e) => {
            println!("Error reading rom: {}", e);
            return;
        }
    };
    let emu = Emulator::new(&rom, args.shift_sets_vx, args.jump_with_offset_bug_emulation, args.increment_i_on_store_and_load).unwrap();
    let interface = Interface::new(emu);
    interface.run();
}