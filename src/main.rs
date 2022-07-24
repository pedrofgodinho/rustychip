use std::fs;
use rustychip::prelude::*;

fn main() {
    //let rom = fs::read("roms/test_opcode.ch8").unwrap();
    let rom = fs::read("roms/c8_test.c8").unwrap();
    let emu = Emulator::new(&rom, false, false, false).unwrap();
    let mut interface = Interface::new(emu);
    interface.run();
}


/*

use rustychip::prelude::*;

fn main() {

    let rom = fs::read("roms/test_opcode.ch8").unwrap();
    let mut emu = Emulator::new(&rom, false, false, false).unwrap();

    loop {
        if emu.step().unwrap() {
            print_display(&emu.display);
        }
    }
}

fn print_display(display: &[[bool; 64]; 32]) {
    for row in display {
        for col in row {
            print!("{}", if *col { '█' } else { ' ' });
        }
        println!();
    }
}

 */