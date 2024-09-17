pub struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    pc: usize,
    stack: Vec<usize>,
    display: [[bool; 64]; 32],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            pc: 0x200,
            stack: Vec::new(),
            display: [[false; 64]; 32],
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

    pub fn execute(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => self.clear_display(),
                0x00EE => self.return_from_subroutine(),
                _ => println!("wtf is thsi?? opcode: {:X}", opcode),
            },
            0x1000 => self.jump(nnn),
            0x6000 => self.set_register(x, nn),
            0x7000 => self.add_to_register(x, nn),
            0xA000 => self.set_index(nnn),
            0xD000 => self.draw(x, y, n),
            _ => println!("wtf is thsi?? opcode: {:X}", opcode),
        }
    }

    fn clear_display(&mut self) {
        self.display = [[false; 64]; 32];
    }

    fn return_from_subroutine(&mut self) {
        self.pc = self.stack.pop().unwrap();
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr as usize;
    }

    fn set_register(&mut self, reg: usize, value: u8) {
        self.registers[reg] = value;
    }

    fn add_to_register(&mut self, reg: usize, value: u8) {
        self.registers[reg] = self.registers[reg].wrapping_add(value);
    }

    fn set_index(&mut self, value: u16) {
        println!("set index register to {:X}", value);
    }

    fn draw(&mut self, x: usize, y: usize, height: u8) {
        println!("draw at ({}, {}) with height {}", x, y, height);
    }

    pub fn print_state(&self) {
        println!("pc: 0x{:X}", self.pc);
        print!("registers: ");
        for (i, reg) in self.registers.iter().enumerate() {
            print!("V{:X}:{:02X} ", i, reg);
        }
        println!("\n");
    }
}