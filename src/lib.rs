use std::fmt::{Display, Formatter};
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


#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("Program size is {0} bytes but cannot exceed 3584 bytes")]
    ProgramTooLarge(usize),
    #[error("The program counter reached the end of memory")]
    PcOutOfBounds(),
    #[error("A decoded instruction is invalid: {0}")]
    InvalidInstruction(Instruction),
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
    pc: usize,
    index: u16,
    stack: [u16; 128],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
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
        write!(f, "Op: {}  x: {}  y: {}  n: {}  nn: {}  nnn: {}", self.operation, self.x, self.y, self.n, self.nn, self.nnn)
    }
}

impl Emulator {
    pub fn new(program: &[u8]) -> Result<Emulator, EmulatorError> {
        if program.len() > 3584 {
            return Err(EmulatorError::ProgramTooLarge(program.len()));
        }

        let mut memory = [0; 0x1000];
        for (i, byte) in program.iter().enumerate() {
            memory[i + 0x200] = *byte;
        }
        for (i, byte) in FONT.iter().enumerate() {
            memory[i + 0x50] = *byte;
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
        })
    }

    pub fn step(&mut self) -> Result<bool, EmulatorError> {
        let opcode = self.fetch_opcode()?;
        self.execute_opcode(opcode)
    }

    fn fetch_opcode(&mut self) -> Result<u16, EmulatorError> {
        if self.pc >= self.memory.len() {
            return Err(EmulatorError::PcOutOfBounds());
        }
        let opcode = (self.memory[self.pc] as u16) << 8 |
            (self.memory[self.pc + 1] as u16);
        self.pc += 2;
        Ok(opcode)
    }

    fn execute_opcode(&mut self, opcode: u16) -> Result<bool, EmulatorError> {
        let instruction = Instruction::from_opcode(opcode);

        match instruction.operation {
            0x00 => Ok(self.operation_0(instruction)),
            0x01 => Ok(self.operation_1(instruction)),
            0x02 => Ok(self.operation_2(instruction)),
            0x03 => Ok(self.operation_3(instruction)),
            0x04 => Ok(self.operation_4(instruction)),
            0x05 => Ok(self.operation_5(instruction)),
            0x06 => Ok(self.operation_6(instruction)),
            0x07 => Ok(self.operation_7(instruction)),
            0x08 => Ok(self.operation_8(instruction)),
            0x09 => Ok(self.operation_9(instruction)),
            0x0A => Ok(self.operation_a(instruction)),
            0x0B => Ok(self.operation_b(instruction)),
            0x0C => Ok(self.operation_c(instruction)),
            0x0D => Ok(self.operation_d(instruction)),
            0x0E => Ok(self.operation_e(instruction)),
            0x0F => Ok(self.operation_f(instruction)),
            _ => Err(EmulatorError::InvalidInstruction(instruction)),
        }
    }

    fn operation_0(&mut self, instruction: Instruction) -> bool {
        match instruction.nnn {
            0x0E0 => { // Clear screen
                for row in self.display.iter_mut() {
                    for pixel in row.iter_mut() {
                        *pixel = false;
                    }
                }
            }
            0x0EE => { // Return
                unimplemented!()
            }
            _ => {
                unimplemented!()
            }
        }
        false
    }

    fn operation_1(&mut self, instruction: Instruction) -> bool {
        self.pc = instruction.nnn as usize;
        false
    }

    fn operation_2(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_3(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_4(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_5(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_6(&mut self, instruction: Instruction) -> bool {
        // Load value into register Vx
        self.registers[instruction.x as usize] = instruction.nn;
        false
    }

    fn operation_7(&mut self, instruction: Instruction) -> bool {
        // Add value to register Vx -- does not affect carry flag
        self.registers[instruction.x as usize] = self.registers[instruction.x as usize].wrapping_add(instruction.nn);
        false
    }

    fn operation_8(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_9(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_a(&mut self, instruction: Instruction) -> bool {
        // Load value into register I
        self.index = instruction.nnn;
        false
    }

    fn operation_b(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_c(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_d(&mut self, instruction: Instruction) -> bool {
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
        true
    }

    fn operation_e(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }

    fn operation_f(&mut self, instruction: Instruction) -> bool {
        unimplemented!()
    }
}