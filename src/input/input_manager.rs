use std::collections::HashSet;

pub struct InputManager {
    keys_pressed: HashSet<u32>,
    keys_just_pressed: HashSet<u32>,
    keys_just_released: HashSet<u32>,
    mouse_position: (f32, f32),
    mouse_buttons_pressed: HashSet<u32>,
    mouse_buttons_just_pressed: HashSet<u32>,
    mouse_buttons_just_released: HashSet<u32>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_just_pressed: HashSet::new(),
            keys_just_released: HashSet::new(),
            mouse_position: (0.0, 0.0),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_just_pressed: HashSet::new(),
            mouse_buttons_just_released: HashSet::new(),
        }
    }
    
    pub fn update(&mut self) {
        // Clear "just pressed/released" states for next frame
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
        self.mouse_buttons_just_pressed.clear();
        self.mouse_buttons_just_released.clear();
    }
    
    // Keyboard input
    pub fn handle_key_down(&mut self, key_code: u32) {
        if !self.keys_pressed.contains(&key_code) {
            self.keys_just_pressed.insert(key_code);
        }
        self.keys_pressed.insert(key_code);
    }
    
    pub fn handle_key_up(&mut self, key_code: u32) {
        if self.keys_pressed.contains(&key_code) {
            self.keys_just_released.insert(key_code);
        }
        self.keys_pressed.remove(&key_code);
    }
    
    pub fn is_key_pressed(&self, key_code: u32) -> bool {
        self.keys_pressed.contains(&key_code)
    }
    
    pub fn is_key_just_pressed(&self, key_code: u32) -> bool {
        self.keys_just_pressed.contains(&key_code)
    }
    
    pub fn is_key_just_released(&self, key_code: u32) -> bool {
        self.keys_just_released.contains(&key_code)
    }
    
    // Mouse input
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }
    
    pub fn handle_mouse_down(&mut self, button: u32, _x: f32, _y: f32) {
        if !self.mouse_buttons_pressed.contains(&button) {
            self.mouse_buttons_just_pressed.insert(button);
        }
        self.mouse_buttons_pressed.insert(button);
    }
    
    pub fn handle_mouse_up(&mut self, button: u32, _x: f32, _y: f32) {
        if self.mouse_buttons_pressed.contains(&button) {
            self.mouse_buttons_just_released.insert(button);
        }
        self.mouse_buttons_pressed.remove(&button);
    }
    
    pub fn get_mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }
    
    pub fn is_mouse_button_pressed(&self, button: u32) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }
    
    pub fn is_mouse_button_just_pressed(&self, button: u32) -> bool {
        self.mouse_buttons_just_pressed.contains(&button)
    }
    
    pub fn is_mouse_button_just_released(&self, button: u32) -> bool {
        self.mouse_buttons_just_released.contains(&button)
    }
}

// Common key codes
pub const KEY_W: u32 = 87;
pub const KEY_A: u32 = 65;
pub const KEY_S: u32 = 83;
pub const KEY_D: u32 = 68;
pub const KEY_SPACE: u32 = 32;
pub const KEY_ESCAPE: u32 = 27;
pub const KEY_ENTER: u32 = 13;
pub const KEY_LEFT_ARROW: u32 = 37;
pub const KEY_UP_ARROW: u32 = 38;
pub const KEY_RIGHT_ARROW: u32 = 39;
pub const KEY_DOWN_ARROW: u32 = 40;

// Mouse button constants
pub const MOUSE_LEFT: u32 = 0;
pub const MOUSE_MIDDLE: u32 = 1;
pub const MOUSE_RIGHT: u32 = 2;
