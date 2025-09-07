use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, Document, Window};
use std::collections::HashMap;

use crate::graphics::renderer::Renderer;
use crate::input::input_manager::InputManager;
use crate::physics::physics_world::PhysicsWorld;
use crate::audio::audio_manager::AudioManager;
use crate::core::ecs::{World, Entity};
use crate::core::time::TimeManager;

pub struct GameEngine {
    canvas: HtmlCanvasElement,
    renderer: Renderer,
    input_manager: InputManager,
    physics_world: PhysicsWorld,
    audio_manager: AudioManager,
    world: World,
    time_manager: TimeManager,
    sprites: HashMap<u32, Entity>,
    next_sprite_id: u32,
    canvas_width: u32,
    canvas_height: u32,
}

impl GameEngine {
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;
        
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();
        
        // Initialize all systems
        let renderer = Renderer::new(&canvas)?;
        let input_manager = InputManager::new();
        let physics_world = PhysicsWorld::new();
        let audio_manager = AudioManager::new()?;
        let world = World::new();
        let time_manager = TimeManager::new();
        
        Ok(GameEngine {
            canvas,
            renderer,
            input_manager,
            physics_world,
            audio_manager,
            world,
            time_manager,
            sprites: HashMap::new(),
            next_sprite_id: 1,
            canvas_width,
            canvas_height,
        })
    }
    
    pub fn update(&mut self, delta_time: f64) {
        self.time_manager.update(delta_time);
        
        // Update physics
        self.physics_world.update(delta_time as f32);
        
        // Update ECS world
        self.world.update(delta_time as f32);
        
        // Process input
        self.input_manager.update();
    }
    
    pub fn render(&mut self) {
        self.renderer.clear();
        
        // Render all sprites
        for (_, entity) in &self.sprites {
            self.renderer.render_sprite(*entity, &self.world);
        }
        
        self.renderer.present();
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.canvas_width = width;
        self.canvas_height = height;
        self.renderer.resize(width, height);
    }
    
    pub fn create_sprite(&mut self, x: f32, y: f32, width: f32, height: f32) -> u32 {
        let entity = self.world.create_sprite_entity(x, y, width, height);
        let sprite_id = self.next_sprite_id;
        self.sprites.insert(sprite_id, entity);
        self.next_sprite_id += 1;
        sprite_id
    }
    
    pub fn set_sprite_position(&mut self, sprite_id: u32, x: f32, y: f32) {
        if let Some(&entity) = self.sprites.get(&sprite_id) {
            self.world.set_position(entity, x, y);
        }
    }
    
    pub fn set_sprite_color(&mut self, sprite_id: u32, r: f32, g: f32, b: f32, a: f32) {
        if let Some(&entity) = self.sprites.get(&sprite_id) {
            self.world.set_color(entity, r, g, b, a);
        }
    }
    
    // Input handling methods
    pub fn handle_key_down(&mut self, key_code: u32) {
        self.input_manager.handle_key_down(key_code);
    }
    
    pub fn handle_key_up(&mut self, key_code: u32) {
        self.input_manager.handle_key_up(key_code);
    }
    
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        self.input_manager.handle_mouse_move(x, y);
    }
    
    pub fn handle_mouse_down(&mut self, button: u32, x: f32, y: f32) {
        self.input_manager.handle_mouse_down(button, x, y);
    }
    
    pub fn handle_mouse_up(&mut self, button: u32, x: f32, y: f32) {
        self.input_manager.handle_mouse_up(button, x, y);
    }
    
    pub fn get_canvas_size(&self) -> (u32, u32) {
        (self.canvas_width, self.canvas_height)
    }
}
