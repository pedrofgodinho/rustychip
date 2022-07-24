use std::fs;
use rustychip::prelude::*;

fn main() {
    //let rom = fs::read("roms/test_opcode.ch8").unwrap();
    let rom = fs::read("roms/Bowling [Gooitzen van der Wal].ch8").unwrap();
    let emu = Emulator::new(&rom, false, false, false).unwrap();
    let interface = Interface::new(emu);
    interface.run();
}