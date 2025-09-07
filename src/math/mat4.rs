use std::ops::{Mul, MulAssign};

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    data: [f32; 16],
}

impl Mat4 {
    pub fn new() -> Self {
        Self {
            data: [0.0; 16],
        }
    }
    
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                x,   y,   z,   1.0,
            ],
        }
    }
    
    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                x,   0.0, 0.0, 0.0,
                0.0, y,   0.0, 0.0,
                0.0, 0.0, z,   0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    
    pub fn rotation_z(angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        Self {
            data: [
                cos_a, sin_a, 0.0, 0.0,
                -sin_a, cos_a, 0.0, 0.0,
                0.0,    0.0,   1.0, 0.0,
                0.0,    0.0,   0.0, 1.0,
            ],
        }
    }
    
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        
        Self {
            data: [
                2.0 / width, 0.0,          0.0,         0.0,
                0.0,         2.0 / height, 0.0,         0.0,
                0.0,         0.0,          -2.0 / depth, 0.0,
                -(right + left) / width, -(top + bottom) / height, -(far + near) / depth, 1.0,
            ],
        }
    }
    
    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fovy / 2.0).tan();
        let depth = near - far;
        
        Self {
            data: [
                f / aspect, 0.0, 0.0,                    0.0,
                0.0,        f,   0.0,                    0.0,
                0.0,        0.0, (far + near) / depth,  -1.0,
                0.0,        0.0, (2.0 * far * near) / depth, 0.0,
            ],
        }
    }
    
    pub fn look_at(eye_x: f32, eye_y: f32, eye_z: f32,
                   center_x: f32, center_y: f32, center_z: f32,
                   up_x: f32, up_y: f32, up_z: f32) -> Self {
        let f_x = center_x - eye_x;
        let f_y = center_y - eye_y;
        let f_z = center_z - eye_z;
        let f_len = (f_x * f_x + f_y * f_y + f_z * f_z).sqrt();
        let f_x = f_x / f_len;
        let f_y = f_y / f_len;
        let f_z = f_z / f_len;
        
        let s_x = f_y * up_z - f_z * up_y;
        let s_y = f_z * up_x - f_x * up_z;
        let s_z = f_x * up_y - f_y * up_x;
        let s_len = (s_x * s_x + s_y * s_y + s_z * s_z).sqrt();
        let s_x = s_x / s_len;
        let s_y = s_y / s_len;
        let s_z = s_z / s_len;
        
        let u_x = s_y * f_z - s_z * f_y;
        let u_y = s_z * f_x - s_x * f_z;
        let u_z = s_x * f_y - s_y * f_x;
        
        Self {
            data: [
                s_x,  u_x,  -f_x, 0.0,
                s_y,  u_y,  -f_y, 0.0,
                s_z,  u_z,  -f_z, 0.0,
                -(s_x * eye_x + s_y * eye_y + s_z * eye_z),
                -(u_x * eye_x + u_y * eye_y + u_z * eye_z),
                f_x * eye_x + f_y * eye_y + f_z * eye_z, 1.0,
            ],
        }
    }
    
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * 4 + col]
    }
    
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row * 4 + col] = value;
    }
    
    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }
    
    pub fn transpose(&self) -> Self {
        Self {
            data: [
                self.data[0], self.data[4], self.data[8],  self.data[12],
                self.data[1], self.data[5], self.data[9],  self.data[13],
                self.data[2], self.data[6], self.data[10], self.data[14],
                self.data[3], self.data[7], self.data[11], self.data[15],
            ],
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    
    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::new();
        
        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.get(row, k) * other.get(k, col);
                }
                result.set(row, col, sum);
            }
        }
        
        result
    }
}

impl MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, other: Mat4) {
        *self = *self * other;
    }
}
