
# RustyChip

This is an emulator for the CHIP-8 assembly language written in Rust.
It was meant as a learning tool for emulator development.

## Running
The emulator needs a ROM file to run.

    cargo run -- [rom]

Other flags are available to emulate the behaviour of other implementations of the CHIP-8.
Check `cargo run -- --help`

The 16 buttons of the CHIP-8 are mapped in the following way:
```
    1 2 3 C      1 2 3 4
    4 5 6 D  ->  q w e r
    7 8 9 E      a s d f
    A 0 B F      z x c v
```

## Known Issues
The current version seems to segfault on my linux machine, although it works fine on my windows machine. Not yet sure why.

Note also that the included `c8_test.c8` will fail if the emulator delay is set too low. 
The emulator will execute the instructions too fast and the test will fail before the timer it's testing changes.
This is not a bug in the emulator. 