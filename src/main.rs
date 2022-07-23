use std::fs;

use rustychip::*;

fn main() {

    let rom = fs::read("roms/IBM Logo.ch8").unwrap();
    let mut emu = Emulator::new(&rom).unwrap();
    loop {
        if emu.step().unwrap() {
            print_display(&emu.display);
        }
    }
}

fn print_display(display: &[[bool; 64]; 32]) {
    for row in display {
        for col in row {
            print!("{}", if *col { '#' } else { ' ' });
        }
        println!();
    }
}