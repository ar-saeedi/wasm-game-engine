use web_sys::Performance;

pub struct TimeManager {
    performance: Option<Performance>,
    last_time: f64,
    delta_time: f64,
    fps: f64,
    frame_count: u32,
    fps_timer: f64,
}

impl TimeManager {
    pub fn new() -> Self {
        let performance = web_sys::window()
            .and_then(|window| window.performance());
        
        let current_time = performance
            .as_ref()
            .map(|p| p.now())
            .unwrap_or(0.0);
        
        Self {
            performance,
            last_time: current_time,
            delta_time: 0.0,
            fps: 60.0,
            frame_count: 0,
            fps_timer: 0.0,
        }
    }
    
    pub fn update(&mut self, delta_time: f64) {
        self.delta_time = delta_time / 1000.0; // Convert ms to seconds
        
        // Update FPS calculation
        self.frame_count += 1;
        self.fps_timer += self.delta_time;
        
        if self.fps_timer >= 1.0 {
            self.fps = self.frame_count as f64 / self.fps_timer;
            self.frame_count = 0;
            self.fps_timer = 0.0;
        }
    }
    
    pub fn get_delta_time(&self) -> f64 {
        self.delta_time
    }
    
    pub fn get_fps(&self) -> f64 {
        self.fps
    }
    
    pub fn get_current_time(&self) -> f64 {
        self.performance
            .as_ref()
            .map(|p| p.now())
            .unwrap_or(0.0)
    }
}
