use wasm_bindgen::prelude::*;

// Import the `console.log` function from the `console` global object
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro for easier console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Use `wee_alloc` as the global allocator for smaller binary size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Core modules
pub mod core;
pub mod graphics;
pub mod physics;
pub mod audio;
pub mod input;
pub mod math;
pub mod utils;

use core::engine::GameEngine;

// Export the main GameEngine to JavaScript
#[wasm_bindgen]
pub struct WasmGameEngine {
    engine: GameEngine,
}

#[wasm_bindgen]
impl WasmGameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WasmGameEngine, JsValue> {
        utils::set_panic_hook();
        console_log!("🎮 Initializing WebAssembly Game Engine...");
        
        let engine = GameEngine::new(canvas_id)?;
        
        console_log!("✅ Game Engine initialized successfully!");
        
        Ok(WasmGameEngine { engine })
    }
    
    #[wasm_bindgen]
    pub fn update(&mut self, delta_time: f64) {
        self.engine.update(delta_time);
    }
    
    #[wasm_bindgen]
    pub fn render(&mut self) {
        self.engine.render();
    }
    
    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.engine.resize(width, height);
    }
    
    #[wasm_bindgen]
    pub fn create_sprite(&mut self, x: f32, y: f32, width: f32, height: f32) -> u32 {
        self.engine.create_sprite(x, y, width, height)
    }
    
    #[wasm_bindgen]
    pub fn set_sprite_position(&mut self, sprite_id: u32, x: f32, y: f32) {
        self.engine.set_sprite_position(sprite_id, x, y);
    }
    
    #[wasm_bindgen]
    pub fn set_sprite_color(&mut self, sprite_id: u32, r: f32, g: f32, b: f32, a: f32) {
        self.engine.set_sprite_color(sprite_id, r, g, b, a);
    }
    
    #[wasm_bindgen]
    pub fn handle_key_down(&mut self, key_code: u32) {
        self.engine.handle_key_down(key_code);
    }
    
    #[wasm_bindgen]
    pub fn handle_key_up(&mut self, key_code: u32) {
        self.engine.handle_key_up(key_code);
    }
    
    #[wasm_bindgen]
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        self.engine.handle_mouse_move(x, y);
    }
    
    #[wasm_bindgen]
    pub fn handle_mouse_down(&mut self, button: u32, x: f32, y: f32) {
        self.engine.handle_mouse_down(button, x, y);
    }
    
    #[wasm_bindgen]
    pub fn handle_mouse_up(&mut self, button: u32, x: f32, y: f32) {
        self.engine.handle_mouse_up(button, x, y);
    }
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
    console_log!("🦀 WebAssembly Game Engine loaded!");
}
