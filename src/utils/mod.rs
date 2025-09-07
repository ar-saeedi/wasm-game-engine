use wasm_bindgen::prelude::*;

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
//
// For more details see
// https://github.com/rustwasm/console_error_panic_hook#readme
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(not(feature = "console_error_panic_hook"))]
pub fn set_panic_hook() {
    // Fallback panic hook
    std::panic::set_hook(Box::new(|info| {
        let msg = match info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Unknown panic occurred",
            }
        };
        
        let location = if let Some(location) = info.location() {
            format!(" at {}:{}:{}", location.file(), location.line(), location.column())
        } else {
            String::new()
        };
        
        web_sys::console::error_1(&format!("Panic occurred: {}{}", msg, location).into());
    }));
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Performance measurement utilities
pub struct PerformanceTimer {
    start_time: Option<f64>,
    name: String,
}

impl PerformanceTimer {
    pub fn new(name: &str) -> Self {
        let start_time = web_sys::window()
            .and_then(|window| window.performance())
            .map(|perf| perf.now());
            
        Self {
            start_time,
            name: name.to_string(),
        }
    }
    
    pub fn end(&self) -> Option<f64> {
        if let (Some(start), Some(perf)) = (
            self.start_time, 
            web_sys::window()?.performance()
        ) {
            let elapsed = perf.now() - start;
            web_sys::console::log_1(&format!("[{}] took: {:.2}ms", self.name, elapsed).into());
            Some(elapsed)
        } else {
            None
        }
    }
}

// Random number utilities
pub struct Random {
    state: u64,
}

impl Random {
    pub fn new() -> Self {
        let seed = web_sys::window()
            .and_then(|window| window.performance())
            .map(|perf| perf.now() as u64)
            .unwrap_or(42);
            
        Self { state: seed }
    }
    
    pub fn from_seed(seed: u64) -> Self {
        Self { state: seed }
    }
    
    // Simple linear congruential generator
    pub fn next_f32(&mut self) -> f32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state as f32) / (u64::MAX as f32)
    }
    
    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
    
    pub fn next_i32(&mut self) -> i32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state as i32
    }
    
    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        min + (self.next_i32().abs() % (max - min))
    }
    
    pub fn next_bool(&mut self) -> bool {
        self.next_f32() > 0.5
    }
}

// Color utilities
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let b = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let a = (hex & 0xFF) as f32 / 255.0;
        Self { r, g, b, a }
    }
    
    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        let t = t.max(0.0).min(1.0);
        Color {
            r: a.r + (b.r - a.r) * t,
            g: a.g + (b.g - a.g) * t,
            b: a.b + (b.b - a.b) * t,
            a: a.a + (b.a - a.a) * t,
        }
    }
}
