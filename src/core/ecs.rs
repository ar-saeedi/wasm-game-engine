use std::collections::HashMap;

pub type Entity = u32;

pub trait Component {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub trait System {
    fn update(&mut self, world: &mut World, delta_time: f32);
}

// Transform component for position, rotation, scale
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

impl Component for Transform {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Sprite component for rendering
#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub width: f32,
    pub height: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,
    pub texture_id: Option<u32>,
}

impl Sprite {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            color_r: 1.0,
            color_g: 1.0,
            color_b: 1.0,
            color_a: 1.0,
            texture_id: None,
        }
    }
    
    pub fn with_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color_r = r;
        self.color_g = g;
        self.color_b = b;
        self.color_a = a;
        self
    }
}

impl Component for Sprite {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// Velocity component for physics
#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for Velocity {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// The main ECS world
pub struct World {
    next_entity_id: Entity,
    entities: Vec<Entity>,
    components: HashMap<Entity, HashMap<std::any::TypeId, Box<dyn Component>>>,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 1,
            entities: Vec::new(),
            components: HashMap::new(),
            systems: Vec::new(),
        }
    }
    
    pub fn create_entity(&mut self) -> Entity {
        let entity = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
        self.components.insert(entity, HashMap::new());
        entity
    }
    
    pub fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        if let Some(entity_components) = self.components.get_mut(&entity) {
            entity_components.insert(std::any::TypeId::of::<T>(), Box::new(component));
        }
    }
    
    pub fn get_component<T: Component + 'static>(&self, entity: Entity) -> Option<&T> {
        self.components.get(&entity)?
            .get(&std::any::TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<T>()
    }
    
    pub fn get_component_mut<T: Component + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(&entity)?
            .get_mut(&std::any::TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<T>()
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Update all systems
        for i in 0..self.systems.len() {
            // We need to split the mutable borrow here
            let (systems_before, systems_after) = self.systems.split_at_mut(i);
            let (current_system, systems_after) = systems_after.split_at_mut(1);
            
            if let Some(system) = current_system.get_mut(0) {
                system.update(self, delta_time);
            }
        }
    }
    
    // Helper methods for common operations
    pub fn create_sprite_entity(&mut self, x: f32, y: f32, width: f32, height: f32) -> Entity {
        let entity = self.create_entity();
        self.add_component(entity, Transform::new(x, y));
        self.add_component(entity, Sprite::new(width, height));
        entity
    }
    
    pub fn set_position(&mut self, entity: Entity, x: f32, y: f32) {
        if let Some(transform) = self.get_component_mut::<Transform>(entity) {
            transform.x = x;
            transform.y = y;
        }
    }
    
    pub fn set_color(&mut self, entity: Entity, r: f32, g: f32, b: f32, a: f32) {
        if let Some(sprite) = self.get_component_mut::<Sprite>(entity) {
            sprite.color_r = r;
            sprite.color_g = g;
            sprite.color_b = b;
            sprite.color_a = a;
        }
    }
    
    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
