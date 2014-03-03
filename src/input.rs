use collections::hashmap::HashMap;
use sdl2::keycode::KeyCode;

pub struct Input {
    held_keys: HashMap<KeyCode, bool>,
    pressed_keys: HashMap<KeyCode, bool>,
    released_keys: HashMap<KeyCode, bool>
}

impl Input {
    pub fn new() -> Input {
        Input {
            held_keys: HashMap::<KeyCode, bool>::new(),
            pressed_keys: HashMap::<KeyCode, bool>::new(),
            released_keys: HashMap::<KeyCode, bool>::new()
        }
    }

    pub fn begin_new_frame(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
    }

    pub fn key_down_event(&mut self, k: KeyCode) {
        self.pressed_keys.insert(k, true);
        self.held_keys.insert(k, true);
    }

    pub fn key_up_event(&mut self, k: KeyCode) {
        self.released_keys.insert(k, true);
        self.held_keys.insert(k, false);
    }

    pub fn was_pressed(&self, k: KeyCode) -> bool {
        match self.pressed_keys.find(&k) {
            Some(is_pressed) => *is_pressed,
            _ => false
        }
    }

    pub fn was_released(&self, k: KeyCode) -> bool {
        match self.released_keys.find(&k) {
            Some(is_released) => *is_released,
            _ => false
        }
    }

    pub fn is_held(&self, k: KeyCode) -> bool {
        match self.held_keys.find(&k) {
            Some(is_held) => *is_held,
            _ => false
        }
    }
}
