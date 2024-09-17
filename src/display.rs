pub struct Display {
    screen: [[bool; 64]; 32],
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [[false; 64]; 32],
        }
    }

    pub fn clear(&mut self) {
        self.screen = [[false; 64]; 32];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) -> bool {
        let x = x % 64;
        let y = y % 32;
        let collision = self.screen[y][x] & on;
        self.screen[y][x] ^= on;
        collision
    }

    pub fn render(&self) {
        for row in &self.screen {
            for &pixel in row {
                print!("{}", if pixel { "█" } else { " " });
            }
            println!();
        }
    }
}