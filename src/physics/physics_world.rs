use crate::physics::collision::{AABB, CollisionDetection};

pub struct PhysicsWorld {
    gravity: f32,
    collision_detector: CollisionDetection,
    time_step: f32,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            gravity: -9.8, // Standard gravity (negative for downward)
            collision_detector: CollisionDetection::new(),
            time_step: 1.0 / 60.0, // 60 FPS physics step
        }
    }
    
    pub fn new_with_gravity(gravity: f32) -> Self {
        Self {
            gravity,
            collision_detector: CollisionDetection::new(),
            time_step: 1.0 / 60.0,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Fixed timestep physics simulation
        let mut accumulator = 0.0;
        let mut current_time = delta_time;
        
        accumulator += current_time;
        
        while accumulator >= self.time_step {
            // Perform physics step
            self.physics_step(self.time_step);
            accumulator -= self.time_step;
        }
    }
    
    fn physics_step(&mut self, dt: f32) {
        // This is where we would update all physics bodies
        // For now, it's a placeholder for the physics integration
        
        // Update velocities based on forces (gravity, etc.)
        // Integrate positions
        // Detect and resolve collisions
    }
    
    pub fn set_gravity(&mut self, gravity: f32) {
        self.gravity = gravity;
    }
    
    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }
    
    pub fn check_collision(&self, aabb1: &AABB, aabb2: &AABB) -> bool {
        self.collision_detector.aabb_vs_aabb(aabb1, aabb2)
    }
    
    pub fn point_in_aabb(&self, point_x: f32, point_y: f32, aabb: &AABB) -> bool {
        self.collision_detector.point_in_aabb(point_x, point_y, aabb)
    }
}

// Physics body component (could be added to ECS)
#[derive(Clone, Copy, Debug)]
pub struct RigidBody {
    pub mass: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub is_static: bool,
    pub bounciness: f32,
    pub friction: f32,
}

impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            velocity_x: 0.0,
            velocity_y: 0.0,
            acceleration_x: 0.0,
            acceleration_y: 0.0,
            is_static: false,
            bounciness: 0.8,
            friction: 0.1,
        }
    }
    
    pub fn static_body() -> Self {
        Self {
            mass: f32::INFINITY,
            velocity_x: 0.0,
            velocity_y: 0.0,
            acceleration_x: 0.0,
            acceleration_y: 0.0,
            is_static: true,
            bounciness: 0.0,
            friction: 1.0,
        }
    }
    
    pub fn apply_force(&mut self, force_x: f32, force_y: f32) {
        if !self.is_static && self.mass > 0.0 {
            self.acceleration_x += force_x / self.mass;
            self.acceleration_y += force_y / self.mass;
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        if !self.is_static {
            // Update velocity from acceleration
            self.velocity_x += self.acceleration_x * dt;
            self.velocity_y += self.acceleration_y * dt;
            
            // Apply friction
            self.velocity_x *= 1.0 - self.friction * dt;
            self.velocity_y *= 1.0 - self.friction * dt;
            
            // Reset acceleration (forces are applied each frame)
            self.acceleration_x = 0.0;
            self.acceleration_y = 0.0;
        }
    }
}
