use std::collections::HashMap;
use winit::keyboard::KeyCode;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum KeyState {
    Up,
    Pressed,
    Hold,
    Released,
}

#[derive(Default)]
pub struct Input {
    key_state: HashMap<KeyCode, KeyState>,
}

impl Input {
    pub fn lookup(&self, key: &KeyCode) -> KeyState {
        self.key_state.get(key).cloned().unwrap_or(KeyState::Up)
    }

    pub fn record(&mut self, key: KeyCode, state: KeyState) {
        self.key_state.insert(key, state);
    }
}
