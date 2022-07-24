use std::fmt::{Display, Formatter};
use rand::prelude::*;
use thiserror::Error;


const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

const CODE_BASE_ADDRESS: u16 = 0x200;
const FONT_BASE_ADDRESS: u16 = 0x50;


#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("Program size is {0} bytes but cannot exceed 3584 bytes")]
    ProgramTooLarge(usize),
    #[error("The program counter reached the end of memory")]
    PcOutOfBounds(),
    #[error("A decoded instruction is invalid: {0}")]
    InvalidInstruction(Instruction),
    #[error("Tried to pop an empty stack")]
    PoppedEmptyStack(),
    #[error("Tried to push a value to a full stack")]
    StackOverflow,
}

#[derive(Debug)]
pub struct Instruction {
    operation: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16,
}

pub struct Emulator {
    memory: [u8; 0x1000],
    pub display: [[bool; 64]; 32],
    pc: u16,
    index: u16,
    stack: [u16; 128],
    sp: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    registers: [u8; 16],
    pub keypad: [bool; 16],
    shift_sets_vx: bool,
    jump_with_offset_bug_emulation: bool,
    increment_i_on_store_and_load: bool,
}


impl Instruction {
    fn from_opcode(opcode: u16) -> Instruction {
        let operation = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        Instruction {
            operation,
            x,
            y,
            n,
            nn,
            nnn,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Op: {:#01x}  x: {:#01x}  y: {:#01x}  n: {:#01x}  nn: {:#02x}  nnn: {:#03x}", self.operation, self.x, self.y, self.n, self.nn, self.nnn)
    }
}

impl Emulator {
    pub fn new(program: &[u8], shift_sets_vx: bool, jump_with_offset_bug_emulation: bool, increment_i_on_store_and_load: bool) -> Result<Emulator, EmulatorError> {
        if program.len() > 0x1000 - CODE_BASE_ADDRESS as usize {
            return Err(EmulatorError::ProgramTooLarge(program.len()));
        }

        let mut memory = [0; 0x1000];
        for (i, byte) in program.iter().enumerate() {
            memory[i + CODE_BASE_ADDRESS as usize] = *byte;
        }
        for (i, byte) in FONT.iter().enumerate() {
            memory[i + FONT_BASE_ADDRESS as usize] = *byte;
        }

        Ok(Emulator {
            memory,
            display: [[false; 64]; 32],
            pc: 0x200,
            index: 0,
            stack: [0; 128],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            keypad: [false; 16],
            shift_sets_vx,
            jump_with_offset_bug_emulation,
            increment_i_on_store_and_load,
        })
    }

    pub fn tick_clock(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn step(&mut self) -> Result<bool, EmulatorError> {
        let opcode = self.fetch_opcode()?;
        self.execute_opcode(opcode)
    }

    fn fetch_opcode(&mut self) -> Result<u16, EmulatorError> {
        if self.pc as usize >= self.memory.len() {
            return Err(EmulatorError::PcOutOfBounds());
        }
        let opcode = (self.memory[self.pc as usize] as u16) << 8 |
            (self.memory[self.pc as usize + 1] as u16);
        self.pc += 2;
        Ok(opcode)
    }

    fn execute_opcode(&mut self, opcode: u16) -> Result<bool, EmulatorError> {
        let instruction = Instruction::from_opcode(opcode);

        match instruction.operation {
            0x00 => self.operation_0(instruction),
            0x01 => self.operation_1(instruction),
            0x02 => self.operation_2(instruction),
            0x03 => self.operation_3(instruction),
            0x04 => self.operation_4(instruction),
            0x05 => self.operation_5(instruction),
            0x06 => self.operation_6(instruction),
            0x07 => self.operation_7(instruction),
            0x08 => self.operation_8(instruction),
            0x09 => self.operation_9(instruction),
            0x0A => self.operation_a(instruction),
            0x0B => self.operation_b(instruction),
            0x0C => self.operation_c(instruction),
            0x0D => self.operation_d(instruction),
            0x0E => self.operation_e(instruction),
            0x0F => self.operation_f(instruction),
            _ => Err(EmulatorError::InvalidInstruction(instruction)),
        }
    }

    fn operation_0(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        match instruction.nnn {
            0x0E0 => { // Clear screen
                for row in self.display.iter_mut() {
                    for pixel in row.iter_mut() {
                        *pixel = false;
                    }
                }
            }
            0x0EE => { // Return
                if self.sp == 0 {
                    return Err(EmulatorError::PoppedEmptyStack());
                }
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            }
            _ => {
                return Err(EmulatorError::InvalidInstruction(instruction));
            }
        }
        Ok(false)
    }

    fn operation_1(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Jump to address NNN
        self.pc = instruction.nnn;
        Ok(false)
    }

    fn operation_2(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Call subroutine at NNN
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        if self.sp == self.stack.len() {
            return Err(EmulatorError::StackOverflow);
        }
        self.pc = instruction.nnn;
        Ok(false)
    }

    fn operation_3(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Skip next instruction if VX == NN
        if self.registers[instruction.x as usize] == instruction.nn {
            self.pc += 2;
        }
        Ok(false)
    }

    fn operation_4(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Skip next instruction if VX != NN
        if self.registers[instruction.x as usize] != instruction.nn {
            self.pc += 2;
        }
        Ok(false)
    }

    fn operation_5(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Skip next instruction if VX == VY
        if self.registers[instruction.x as usize] == self.registers[instruction.y as usize] {
            self.pc += 2;
        }
        Ok(false)
    }

    fn operation_6(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Load value into register Vx
        self.registers[instruction.x as usize] = instruction.nn;
        Ok(false)
    }

    fn operation_7(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Add value to register Vx -- does not affect carry flag
        self.registers[instruction.x as usize] = self.registers[instruction.x as usize].wrapping_add(instruction.nn);
        Ok(false)
    }

    fn operation_8(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        match instruction.n {
            0x0 => { // Set register Vx to value of register Vy
                self.registers[instruction.x as usize] = self.registers[instruction.y as usize];
            }
            0x1 => { // Or Vx with Vy
                self.registers[instruction.x as usize] |= self.registers[instruction.y as usize];
            }
            0x2 => { // And Vx with Vy
                self.registers[instruction.x as usize] &= self.registers[instruction.y as usize];
            }
            0x3 => { // Xor Vx with Vy
                self.registers[instruction.x as usize] ^= self.registers[instruction.y as usize];
            }
            0x4 => { // Add Vx to Vy
                let (result, carry) = self.registers[instruction.x as usize].overflowing_add(self.registers[instruction.y as usize]);
                self.registers[instruction.x as usize] = result;
                self.registers[0xF] = carry as u8;
            }
            0x5 => { // Subtract Vy from Vx
                let (result, borrow) = self.registers[instruction.x as usize].overflowing_sub(self.registers[instruction.y as usize]);
                self.registers[instruction.x as usize] = result;
                self.registers[0xF] = !borrow as u8;
            }
            0x6 => { // Shift Vx right by 1
                if self.shift_sets_vx {
                    self.registers[instruction.x as usize] = self.registers[instruction.y as usize];
                }
                self.registers[0xF] = self.registers[instruction.x as usize] & 0x1;
                self.registers[instruction.x as usize] >>= 1;
            }
            0x7 => { // Subtract Vy from Vx (Vx = Vy - Vx)
                let (result, borrow) = self.registers[instruction.y as usize].overflowing_sub(self.registers[instruction.x as usize]);
                self.registers[instruction.x as usize] = result;
                self.registers[0xF] = !borrow as u8;
            }
            0xE => { // Shift Vx left by 1
                if self.shift_sets_vx {
                    self.registers[instruction.x as usize] = self.registers[instruction.y as usize];
                }
                self.registers[0xF] = (self.registers[instruction.x as usize] >> 7) & 0x1;
                self.registers[instruction.x as usize] <<= 1;
            }
            _ => {
                return Err(EmulatorError::InvalidInstruction(instruction));
            }
        }
        Ok(false)
    }

    fn operation_9(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Skip next instruction if Vx != Vy
        if self.registers[instruction.x as usize] != self.registers[instruction.y as usize] {
            self.pc += 2;
        }
        Ok(false)
    }

    fn operation_a(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Load value into register I
        self.index = instruction.nnn;
        Ok(false)
    }

    fn operation_b(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Jump to address NNN + V0 unless the chip-48 bug is being emulated
        if self.jump_with_offset_bug_emulation {
            self.pc = instruction.nnn + self.registers[instruction.x as usize] as u16;
        } else {
            self.pc = instruction.nnn + self.registers[0] as u16;
        }
        Ok(false)
    }

    fn operation_c(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Load random number into register Vx
        self.registers[instruction.x as usize] = random::<u8>() & instruction.nn;
        Ok(false)
    }

    fn operation_d(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

        // Clear VF
        self.registers[0xF] = 0;

        let mut y = self.registers[instruction.y as usize] % self.display.len() as u8;
        // For each row of sprite
        for row in 0..instruction.n {
            let mut x = self.registers[instruction.x as usize] % self.display[0].len() as u8;
            let sprite_row = self.memory[(self.index + row as u16) as usize];
            // For each pixel in row
            for col in 0..8 {
                let sprite_pixel = (sprite_row >> (7 - col)) & 1 == 1;
                let display_pixel = self.display[y as usize][x as usize];
                // Check for collision
                if sprite_pixel && display_pixel {
                    self.registers[0xF] = 1;
                }
                // Xor display pixel
                self.display[y as usize][x as usize] ^= sprite_pixel;
                x += 1;
                if x >= self.display[0].len() as u8 {
                    break;
                }
            }
            y += 1;
            if y >= self.display.len() as u8 {
                break;
            }
        }
        Ok(true)
    }

    fn operation_e(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        match instruction.nn {
            0x9E => { // Skip next instruction if key with value Vx is pressed
                if self.registers[instruction.x as usize] > 16 {
                    return Err(EmulatorError::InvalidInstruction(instruction));
                }
                if self.keypad[self.registers[instruction.x as usize] as usize] {
                    self.pc += 2;
                }
            }
            0xA1 => { // Skip next instruction if key with value Vx is not pressed
                if self.registers[instruction.x as usize] > 16 {
                    return Err(EmulatorError::InvalidInstruction(instruction));
                }
                if !self.keypad[self.registers[instruction.x as usize] as usize] {
                    self.pc += 2;
                }
            }
            _ => {
                return Err(EmulatorError::InvalidInstruction(instruction));
            }
        }
        Ok(false)
    }

    fn operation_f(&mut self, instruction: Instruction) -> Result<bool, EmulatorError> {
        match instruction.nn {
            0x07 => { // Load Vx with delay timer value
                self.registers[instruction.x as usize] = self.delay_timer;
            }
            0x15 => { // Set delay timer to Vx
                self.delay_timer = self.registers[instruction.x as usize];
            }
            0x18 => { // Set sound timer to Vx
                self.sound_timer = self.registers[instruction.x as usize];
            }
            0x1e => { // Add Vx to I
                self.index += self.registers[instruction.x as usize] as u16;
                // Check for overflow. Original cosmac emulator does not check for overflow
                // however some interpreters do and at least one game is known to rely on this.
                // No known games relies on this not happening, so we check for overflow
                if self.index > 0xFFF {
                    self.registers[0xF] = 1;
                    self.index &= 0xFFF;
                } else {
                    self.registers[0xF] = 0;
                }
            }
            0x0A => {
                // Wait for a key press and store the value of the key in Vx
                let mut key_pressed = false;
                for (index, state) in self.keypad.iter().enumerate() {
                    if *state {
                        self.registers[instruction.x as usize] = index as u8;
                        key_pressed = true;
                        break;
                    }
                }
                if !key_pressed {
                    self.pc -= 2;
                }
            }
            0x29 => {
                // Load location of sprite for digit Vx into I
                self.index = ((self.registers[instruction.x as usize] as u16 & 0xF) * 5) + FONT_BASE_ADDRESS;
            }
            0x33 => {
                // Store BCD representation of Vx in memory locations I, I+1, and I+2
                let mut value = self.registers[instruction.x as usize];
                self.memory[self.index as usize] = value / 100;
                value %= 100;
                self.memory[(self.index + 1) as usize] = value / 10;
                value %= 10;
                self.memory[(self.index + 2) as usize] = value;
            }
            0x55 => {
                // Store registers V0 through Vx in memory starting at location I
                for i in 0..instruction.x + 1 {
                    self.memory[(self.index + i as u16) as usize] = self.registers[i as usize];
                }
                if self.increment_i_on_store_and_load {
                    self.index += instruction.x as u16 + 1;
                }
            }
            0x65 => {
                // Load registers V0 through Vx from memory starting at location I
                for i in 0..instruction.x + 1 {
                    self.registers[i as usize] = self.memory[(self.index + i as u16) as usize];
                }
                if self.increment_i_on_store_and_load {
                    self.index += instruction.x as u16 + 1;
                }
            }
            _ => {
                return Err(EmulatorError::InvalidInstruction(instruction));
            }
        }
        Ok(false)
    }
}