use crate::display::Display;
use crate::keyboard::Keyboard;
use rand::{random, Rng};

pub struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    pc: usize,
    i: u16,
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            pc: 0x200,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0x200..(0x200 + rom.len())].copy_from_slice(rom);
    }

    pub fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc] as u16;
        let lo = self.memory[self.pc + 1] as u16;
        self.pc += 2;
        (hi << 8) | lo
    }

    pub fn execute(&mut self, opcode: u16, display: &mut Display, keyboard: &mut Keyboard) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => display.clear(),
                0x00EE => self.return_from_subroutine(),
                _ => println!("this opcode is sus: 0x{:04X}", opcode),
            },
            0x1000 => self.jump(nnn),
            0x2000 => self.call_subroutine(nnn),
            0x3000 => self.skip_if_equal(x, nn),
            0x4000 => self.skip_if_not_equal(x, nn),
            0x5000 => self.skip_if_registers_equal(x, y),
            0x6000 => self.set_register(x, nn),
            0x7000 => self.add_to_register(x, nn),
            0x8000 => self.arithmetic_operations(x, y, n),
            0x9000 => self.skip_if_registers_not_equal(x, y),
            0xA000 => self.set_index(nnn),
            0xB000 => self.jump_with_offset(nnn),
            0xC000 => self.random(x, nn),
            0xD000 => self.draw(x, y, n, display),
            0xE000 => self.keyboard_operations(x, nn, keyboard),
            0xF000 => self.misc_operations(x, nn),
            _ => println!("l opcode: 0x{:04X}", opcode),
        }
    }

    fn return_from_subroutine(&mut self) {
        self.pc = self.stack.pop().unwrap();
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr as usize;
    }

    fn call_subroutine(&mut self, addr: u16) {
        self.stack.push(self.pc);
        self.pc = addr as usize;
    }

    fn skip_if_equal(&mut self, x: usize, nn: u8) {
        if self.registers[x] == nn {
            self.pc += 2;
        }
    }

    fn skip_if_not_equal(&mut self, x: usize, nn: u8) {
        if self.registers[x] != nn {
            self.pc += 2;
        }
    }

    fn skip_if_registers_equal(&mut self, x: usize, y: usize) {
        if self.registers[x] == self.registers[y] {
            self.pc += 2;
        }
    }

    fn set_register(&mut self, reg: usize, value: u8) {
        self.registers[reg] = value;
    }

    fn add_to_register(&mut self, reg: usize, value: u8) {
        self.registers[reg] = self.registers[reg].wrapping_add(value);
    }

    fn arithmetic_operations(&mut self, x: usize, y: usize, n: u8) {
        match n {
            0x0 => self.registers[x] = self.registers[y],
            0x1 => self.registers[x] |= self.registers[y],
            0x2 => self.registers[x] &= self.registers[y],
            0x3 => self.registers[x] ^= self.registers[y],
            0x4 => {
                let (val, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = val;
                self.registers[0xF] = if overflow { 1 } else { 0 };
            },
            0x5 => {
                let (val, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = val;
                self.registers[0xF] = if borrow { 0 } else { 1 };
            },
            0x6 => {
                self.registers[0xF] = self.registers[x] & 1;
                self.registers[x] >>= 1;
            },
            0x7 => {
                let (val, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = val;
                self.registers[0xF] = if borrow { 0 } else { 1 };
            },
            0xE => {
                self.registers[0xF] = (self.registers[x] & 0x80) >> 7;
                self.registers[x] <<= 1;
            },
            _ => println!("wtf math: 0x{:X}", n),
        }
    }

    fn skip_if_registers_not_equal(&mut self, x: usize, y: usize) {
        if self.registers[x] != self.registers[y] {
            self.pc += 2;
        }
    }

    fn set_index(&mut self, value: u16) {
        self.i = value;
    }

    fn jump_with_offset(&mut self, addr: u16) {
        self.pc = (addr + self.registers[0] as u16) as usize;
    }

    fn random(&mut self, x: usize, nn: u8) {
        let random_byte: u8 = random();
        self.registers[x] = random_byte & nn;
    }

    fn draw(&mut self, x: usize, y: usize, height: u8, display: &mut Display) {
        let x_coord = self.registers[x] as usize;
        let y_coord = self.registers[y] as usize;
        let mut flipped = false;

        for row in 0..height {
            let sprite_byte = self.memory[self.i as usize + row as usize];
            for col in 0..8 {
                if (sprite_byte & (0x80 >> col)) != 0 {
                    if display.set_pixel(x_coord + col, y_coord + row as usize, true) {
                        flipped = true;
                    }
                }
            }
        }

        self.registers[0xF] = if flipped { 1 } else { 0 };
    }

    fn keyboard_operations(&mut self, x: usize, nn: u8, keyboard: &mut Keyboard) {
        match nn {
            0x9E => if keyboard.is_key_pressed(self.registers[x] as usize) { self.pc += 2; },
            0xA1 => if !keyboard.is_key_pressed(self.registers[x] as usize) { self.pc += 2; },
            _ => println!("bad keyboard: 0x{:X}", nn),
        }
    }

    fn misc_operations(&mut self, x: usize, nn: u8) {
        match nn {
            0x07 => self.registers[x] = self.delay_timer,
            0x0A => {
                println!("waiiiting");
            },
            0x15 => self.delay_timer = self.registers[x],
            0x18 => self.sound_timer = self.registers[x],
            0x1E => self.i += self.registers[x] as u16,
            0x29 => self.i = (self.registers[x] as u16) * 5,
            0x33 => {
                self.memory[self.i as usize] = self.registers[x] / 100;
                self.memory[self.i as usize + 1] = (self.registers[x] / 10) % 10;
                self.memory[self.i as usize + 2] = self.registers[x] % 10;
            },
            0x55 => {
                for i in 0..=x {
                    self.memory[self.i as usize + i] = self.registers[i];
                }
            },
            0x65 => {
                for i in 0..=x {
                    self.registers[i] = self.memory[self.i as usize + i];
                }
            },
            _ => println!("unknown misc: 0x{:X}", nn),
        }
    }

    pub fn print_state(&self) {
        println!("pc: 0x{:04X}, i: 0x{:04X}", self.pc, self.i);
        print!("registers: ");
        for (i, reg) in self.registers.iter().enumerate() {
            print!("v{:X}:{:02X} ", i, reg);
        }
        println!("\ndelay timer: {}, sound timer: {}", self.delay_timer, self.sound_timer);
        println!();
    }
}