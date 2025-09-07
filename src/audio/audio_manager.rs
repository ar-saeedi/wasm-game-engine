use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AudioBuffer, AudioBufferSourceNode, GainNode};
use std::collections::HashMap;

pub struct AudioManager {
    audio_context: Option<AudioContext>,
    master_gain: Option<GainNode>,
    sound_buffers: HashMap<String, AudioBuffer>,
    sound_sources: Vec<AudioBufferSourceNode>,
}

impl AudioManager {
    pub fn new() -> Result<Self, JsValue> {
        let audio_context = AudioContext::new().ok();
        let master_gain = if let Some(ref ctx) = audio_context {
            let gain = ctx.create_gain()?;
            gain.connect_with_audio_node(&ctx.destination())?;
            gain.gain().set_value(1.0);
            Some(gain)
        } else {
            None
        };
        
        Ok(Self {
            audio_context,
            master_gain,
            sound_buffers: HashMap::new(),
            sound_sources: Vec::new(),
        })
    }
    
    pub fn set_master_volume(&self, volume: f32) -> Result<(), JsValue> {
        if let Some(ref gain) = self.master_gain {
            gain.gain().set_value(volume.max(0.0).min(1.0));
        }
        Ok(())
    }
    
    pub fn load_sound(&mut self, name: &str, audio_data: &[u8]) -> Result<(), JsValue> {
        if let Some(ref ctx) = self.audio_context {
            let array_buffer = js_sys::ArrayBuffer::new(audio_data.len() as u32);
            let uint8_array = js_sys::Uint8Array::new(&array_buffer);
            uint8_array.copy_from(audio_data);
            
            // Note: In a real implementation, we'd need to decode the audio data
            // For now, this is a placeholder structure
            console_log!("Audio loading not fully implemented yet - placeholder");
        }
        Ok(())
    }
    
    pub fn play_sound(&mut self, name: &str) -> Result<(), JsValue> {
        self.play_sound_with_volume(name, 1.0)
    }
    
    pub fn play_sound_with_volume(&mut self, name: &str, volume: f32) -> Result<(), JsValue> {
        if let (Some(ref ctx), Some(ref master_gain)) = (&self.audio_context, &self.master_gain) {
            if let Some(buffer) = self.sound_buffers.get(name) {
                let source = ctx.create_buffer_source()?;
                let gain = ctx.create_gain()?;
                
                source.set_buffer(Some(buffer));
                gain.gain().set_value(volume.max(0.0).min(1.0));
                
                source.connect_with_audio_node(&gain)?;
                gain.connect_with_audio_node(master_gain)?;
                
                source.start()?;
                
                // Store reference to prevent cleanup
                self.sound_sources.push(source);
            }
        }
        Ok(())
    }
    
    pub fn play_sound_looped(&mut self, name: &str) -> Result<(), JsValue> {
        if let (Some(ref ctx), Some(ref master_gain)) = (&self.audio_context, &self.master_gain) {
            if let Some(buffer) = self.sound_buffers.get(name) {
                let source = ctx.create_buffer_source()?;
                source.set_buffer(Some(buffer));
                source.set_loop(true);
                source.connect_with_audio_node(master_gain)?;
                source.start()?;
                
                self.sound_sources.push(source);
            }
        }
        Ok(())
    }
    
    pub fn stop_all_sounds(&mut self) {
        for source in &self.sound_sources {
            let _ = source.stop();
        }
        self.sound_sources.clear();
    }
    
    pub fn create_oscillator(&self, frequency: f32, wave_type: &str) -> Result<(), JsValue> {
        if let (Some(ref ctx), Some(ref master_gain)) = (&self.audio_context, &self.master_gain) {
            let oscillator = ctx.create_oscillator()?;
            let gain = ctx.create_gain()?;
            
            oscillator.frequency().set_value(frequency);
            oscillator.set_type(&wave_type.parse().unwrap_or(web_sys::OscillatorType::Sine));
            
            gain.gain().set_value(0.1); // Lower volume for oscillator
            
            oscillator.connect_with_audio_node(&gain)?;
            gain.connect_with_audio_node(master_gain)?;
            
            oscillator.start()?;
            
            // Auto-stop after 0.5 seconds
            oscillator.stop_with_when(ctx.current_time() + 0.5)?;
        }
        Ok(())
    }
    
    pub fn beep(&self) -> Result<(), JsValue> {
        self.create_oscillator(440.0, "sine")
    }
    
    pub fn is_audio_available(&self) -> bool {
        self.audio_context.is_some()
    }
}
