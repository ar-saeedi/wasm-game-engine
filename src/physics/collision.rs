#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl AABB {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn min_x(&self) -> f32 {
        self.x
    }
    
    pub fn min_y(&self) -> f32 {
        self.y
    }
    
    pub fn max_x(&self) -> f32 {
        self.x + self.width
    }
    
    pub fn max_y(&self) -> f32 {
        self.y + self.height
    }
    
    pub fn center_x(&self) -> f32 {
        self.x + self.width / 2.0
    }
    
    pub fn center_y(&self) -> f32 {
        self.y + self.height / 2.0
    }
    
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.min_x() && px <= self.max_x() &&
        py >= self.min_y() && py <= self.max_y()
    }
    
    pub fn intersects(&self, other: &AABB) -> bool {
        !(self.max_x() < other.min_x() || 
          self.min_x() > other.max_x() ||
          self.max_y() < other.min_y() ||
          self.min_y() > other.max_y())
    }
}

pub struct CollisionDetection {
    // Could contain spatial partitioning structures like quadtree in the future
}

impl CollisionDetection {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn aabb_vs_aabb(&self, a: &AABB, b: &AABB) -> bool {
        a.intersects(b)
    }
    
    pub fn point_in_aabb(&self, px: f32, py: f32, aabb: &AABB) -> bool {
        aabb.contains_point(px, py)
    }
    
    pub fn circle_vs_circle(&self, x1: f32, y1: f32, r1: f32, x2: f32, y2: f32, r2: f32) -> bool {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let distance_squared = dx * dx + dy * dy;
        let radius_sum = r1 + r2;
        distance_squared <= radius_sum * radius_sum
    }
    
    pub fn circle_vs_aabb(&self, cx: f32, cy: f32, radius: f32, aabb: &AABB) -> bool {
        // Find the closest point on the AABB to the circle center
        let closest_x = cx.max(aabb.min_x()).min(aabb.max_x());
        let closest_y = cy.max(aabb.min_y()).min(aabb.max_y());
        
        // Calculate distance from circle center to closest point
        let dx = cx - closest_x;
        let dy = cy - closest_y;
        let distance_squared = dx * dx + dy * dy;
        
        distance_squared <= radius * radius
    }
    
    // Ray-AABB intersection (useful for raycasting)
    pub fn ray_vs_aabb(&self, ray_x: f32, ray_y: f32, ray_dx: f32, ray_dy: f32, aabb: &AABB) -> Option<f32> {
        let inv_dx = 1.0 / ray_dx;
        let inv_dy = 1.0 / ray_dy;
        
        let t1 = (aabb.min_x() - ray_x) * inv_dx;
        let t2 = (aabb.max_x() - ray_x) * inv_dx;
        let t3 = (aabb.min_y() - ray_y) * inv_dy;
        let t4 = (aabb.max_y() - ray_y) * inv_dy;
        
        let tmin = t1.min(t2).max(t3.min(t4));
        let tmax = t1.max(t2).min(t3.max(t4));
        
        // If tmax < 0, ray is intersecting AABB but in opposite direction
        if tmax < 0.0 {
            return None;
        }
        
        // If tmin > tmax, ray doesn't intersect AABB
        if tmin > tmax {
            return None;
        }
        
        // Return the closest intersection distance
        Some(if tmin < 0.0 { tmax } else { tmin })
    }
}

// Collision response data
#[derive(Clone, Copy, Debug)]
pub struct CollisionInfo {
    pub penetration_x: f32,
    pub penetration_y: f32,
    pub normal_x: f32,
    pub normal_y: f32,
    pub contact_x: f32,
    pub contact_y: f32,
}

impl CollisionInfo {
    pub fn resolve_aabb_collision(a: &AABB, b: &AABB) -> Option<CollisionInfo> {
        if !a.intersects(b) {
            return None;
        }
        
        // Calculate overlap on both axes
        let x_overlap = (a.max_x() - b.min_x()).min(b.max_x() - a.min_x());
        let y_overlap = (a.max_y() - b.min_y()).min(b.max_y() - a.min_y());
        
        // Determine which axis has the smallest overlap
        if x_overlap < y_overlap {
            // Separate on X axis
            let normal_x = if a.center_x() < b.center_x() { -1.0 } else { 1.0 };
            Some(CollisionInfo {
                penetration_x: x_overlap * normal_x,
                penetration_y: 0.0,
                normal_x,
                normal_y: 0.0,
                contact_x: if normal_x > 0.0 { a.max_x() } else { a.min_x() },
                contact_y: a.center_y(),
            })
        } else {
            // Separate on Y axis
            let normal_y = if a.center_y() < b.center_y() { -1.0 } else { 1.0 };
            Some(CollisionInfo {
                penetration_x: 0.0,
                penetration_y: y_overlap * normal_y,
                normal_x: 0.0,
                normal_y,
                contact_x: a.center_x(),
                contact_y: if normal_y > 0.0 { a.max_y() } else { a.min_y() },
            })
        }
    }
}
