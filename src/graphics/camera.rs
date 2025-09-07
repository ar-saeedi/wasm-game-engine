use crate::math::{Mat4, Vec2, Vec3};

pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    viewport_size: Vec2,
    is_orthographic: bool,
    // Orthographic parameters
    ortho_size: f32,
    ortho_near: f32,
    ortho_far: f32,
    // Perspective parameters
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
    dirty: bool,
}

impl Camera {
    pub fn new_orthographic(viewport_width: f32, viewport_height: f32) -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            target: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            projection_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            viewport_size: Vec2::new(viewport_width, viewport_height),
            is_orthographic: true,
            ortho_size: 1.0,
            ortho_near: -1.0,
            ortho_far: 1.0,
            fov: 45.0,
            aspect_ratio: viewport_width / viewport_height,
            near_plane: 0.1,
            far_plane: 100.0,
            dirty: true,
        };
        
        camera.update_projection();
        camera.update_view();
        camera
    }
    
    pub fn new_perspective(viewport_width: f32, viewport_height: f32, fov: f32) -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            target: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            projection_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            viewport_size: Vec2::new(viewport_width, viewport_height),
            is_orthographic: false,
            ortho_size: 1.0,
            ortho_near: -1.0,
            ortho_far: 1.0,
            fov,
            aspect_ratio: viewport_width / viewport_height,
            near_plane: 0.1,
            far_plane: 100.0,
            dirty: true,
        };
        
        camera.update_projection();
        camera.update_view();
        camera
    }
    
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.dirty = true;
    }
    
    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
        self.dirty = true;
    }
    
    pub fn look_at(&mut self, eye: Vec3, center: Vec3, up: Vec3) {
        self.position = eye;
        self.target = center;
        self.up = up;
        self.dirty = true;
    }
    
    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
        self.target += delta;
        self.dirty = true;
    }
    
    pub fn set_viewport_size(&mut self, width: f32, height: f32) {
        self.viewport_size = Vec2::new(width, height);
        self.aspect_ratio = width / height;
        self.update_projection();
    }
    
    pub fn set_orthographic_size(&mut self, size: f32) {
        self.ortho_size = size;
        if self.is_orthographic {
            self.update_projection();
        }
    }
    
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        if !self.is_orthographic {
            self.update_projection();
        }
    }
    
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> Vec2 {
        if self.is_orthographic {
            // Simple orthographic screen-to-world conversion
            let world_x = (screen_x - self.viewport_size.x * 0.5) + self.position.x;
            let world_y = (screen_y - self.viewport_size.y * 0.5) + self.position.y;
            Vec2::new(world_x, world_y)
        } else {
            // For perspective, this would require more complex ray casting
            // For now, return screen coordinates
            Vec2::new(screen_x, screen_y)
        }
    }
    
    pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> Vec2 {
        if self.is_orthographic {
            let screen_x = (world_x - self.position.x) + self.viewport_size.x * 0.5;
            let screen_y = (world_y - self.position.y) + self.viewport_size.y * 0.5;
            Vec2::new(screen_x, screen_y)
        } else {
            // For perspective, this would require projection
            Vec2::new(world_x, world_y)
        }
    }
    
    pub fn get_view_matrix(&mut self) -> &Mat4 {
        if self.dirty {
            self.update_view();
        }
        &self.view_matrix
    }
    
    pub fn get_projection_matrix(&self) -> &Mat4 {
        &self.projection_matrix
    }
    
    pub fn get_position(&self) -> Vec3 {
        self.position
    }
    
    pub fn get_target(&self) -> Vec3 {
        self.target
    }
    
    fn update_projection(&mut self) {
        if self.is_orthographic {
            let half_width = self.viewport_size.x * 0.5 * self.ortho_size;
            let half_height = self.viewport_size.y * 0.5 * self.ortho_size;
            
            self.projection_matrix = Mat4::orthographic(
                -half_width, half_width,
                -half_height, half_height,
                self.ortho_near, self.ortho_far
            );
        } else {
            self.projection_matrix = Mat4::perspective(
                self.fov.to_radians(),
                self.aspect_ratio,
                self.near_plane,
                self.far_plane
            );
        }
    }
    
    fn update_view(&mut self) {
        self.view_matrix = Mat4::look_at(
            self.position.x, self.position.y, self.position.z,
            self.target.x, self.target.y, self.target.z,
            self.up.x, self.up.y, self.up.z
        );
        self.dirty = false;
    }
}
