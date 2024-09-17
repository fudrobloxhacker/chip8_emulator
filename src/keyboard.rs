pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { keys: [false; 16] }
    }

    pub fn is_key_pressed(&self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
    }

    pub fn wait_for_key_press(&self) -> Option<usize> {
        self.keys.iter().position(|&key| key)
    }
}