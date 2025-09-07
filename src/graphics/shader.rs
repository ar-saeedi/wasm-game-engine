use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};
use std::collections::HashMap;

use crate::math::mat4::Mat4;

pub struct Shader {
    id: WebGlShader,
}

impl Shader {
    pub fn new(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<Self, JsValue> {
        let shader = gl.create_shader(shader_type)
            .ok_or("Unable to create shader object")?;
        
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);
        
        if gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false) {
            Ok(Shader { id: shader })
        } else {
            let info = gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".into());
            Err(JsValue::from_str(&info))
        }
    }
    
    pub fn id(&self) -> &WebGlShader {
        &self.id
    }
}

pub struct ShaderProgram {
    program: WebGlProgram,
    uniform_locations: HashMap<String, WebGlUniformLocation>,
}

impl ShaderProgram {
    pub fn new(gl: &WebGl2RenderingContext, vertex_source: &str, fragment_source: &str) -> Result<Self, JsValue> {
        let vertex_shader = Shader::new(gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_source)?;
        let fragment_shader = Shader::new(gl, WebGl2RenderingContext::FRAGMENT_SHADER, fragment_source)?;
        
        let program = gl.create_program()
            .ok_or("Unable to create shader program")?;
        
        gl.attach_shader(&program, vertex_shader.id());
        gl.attach_shader(&program, fragment_shader.id());
        gl.link_program(&program);
        
        if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false) {
            
            // Clean up shaders (they're now linked into the program)
            gl.delete_shader(Some(vertex_shader.id()));
            gl.delete_shader(Some(fragment_shader.id()));
            
            Ok(ShaderProgram {
                program,
                uniform_locations: HashMap::new(),
            })
        } else {
            let info = gl.get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error linking program".into());
            Err(JsValue::from_str(&info))
        }
    }
    
    pub fn use_program(&self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));
    }
    
    fn get_uniform_location(&mut self, gl: &WebGl2RenderingContext, name: &str) -> Option<&WebGlUniformLocation> {
        if !self.uniform_locations.contains_key(name) {
            if let Some(location) = gl.get_uniform_location(&self.program, name) {
                self.uniform_locations.insert(name.to_string(), location);
            } else {
                return None;
            }
        }
        self.uniform_locations.get(name)
    }
    
    pub fn set_bool(&mut self, gl: &WebGl2RenderingContext, name: &str, value: bool) {
        if let Some(location) = self.get_uniform_location(gl, name) {
            gl.uniform1i(Some(location), if value { 1 } else { 0 });
        }
    }
    
    pub fn set_int(&mut self, gl: &WebGl2RenderingContext, name: &str, value: i32) {
        if let Some(location) = self.get_uniform_location(gl, name) {
            gl.uniform1i(Some(location), value);
        }
    }
    
    pub fn set_float(&mut self, gl: &WebGl2RenderingContext, name: &str, value: f32) {
        if let Some(location) = self.get_uniform_location(gl, name) {
            gl.uniform1f(Some(location), value);
        }
    }
    
    pub fn set_vec2(&mut self, gl: &WebGl2RenderingContext, name: &str, x: f32, y: f32) {
        if let Some(location) = self.get_uniform_location(gl, name) {
            gl.uniform2f(Some(location), x, y);
        }
    }
    
    pub fn set_vec3(&mut self, gl: &WebGl2RenderingContext, name: &str, x: f32, y: f32, z: f32) {
        if let Some(location) = self.get_uniform_location(gl, name) {
            gl.uniform3f(Some(location), x, y, z);
        }
    }
    
    pub fn set_vec4(&mut self, gl: &WebGl2RenderingContext, name: &str, x: f32, y: f32, z: f32, w: f32) {
        if let Some(location) = self.get_uniform_location(gl, name) {
            gl.uniform4f(Some(location), x, y, z, w);
        }
    }
    
    pub fn set_mat4(&mut self, gl: &WebGl2RenderingContext, name: &str, matrix: &Mat4) {
        if let Some(location) = self.get_uniform_location(gl, name) {
            gl.uniform_matrix4fv_with_f32_array(Some(location), false, matrix.as_slice());
        }
    }
}
