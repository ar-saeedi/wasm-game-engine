use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };
    pub const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };
    pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
    pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    pub fn normalize(&self) -> Vec2 {
        let len = self.length();
        if len > 0.0 {
            Vec2 { x: self.x / len, y: self.y / len }
        } else {
            Vec2::ZERO
        }
    }
    
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    
    pub fn distance(&self, other: Vec2) -> f32 {
        (*self - other).length()
    }
    
    pub fn distance_squared(&self, other: Vec2) -> f32 {
        (*self - other).length_squared()
    }
    
    pub fn lerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
        let t = t.max(0.0).min(1.0);
        Vec2 {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
        }
    }
    
    pub fn rotate(&self, angle: f32) -> Vec2 {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Vec2 {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
        }
    }
    
    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }
    
    pub fn reflect(&self, normal: Vec2) -> Vec2 {
        *self - normal * (2.0 * self.dot(normal))
    }
}

// Operator implementations
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f32) -> Vec2 {
        Vec2 { x: self.x / scalar, y: self.y / scalar }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}
