use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlBuffer, WebGlVertexArrayObject};

use crate::core::ecs::{World, Entity, Transform, Sprite};
use crate::graphics::shader::ShaderProgram;
use crate::math::mat4::Mat4;

pub struct Renderer {
    gl: WebGl2RenderingContext,
    sprite_shader: ShaderProgram,
    quad_vao: Option<WebGlVertexArrayObject>,
    quad_vbo: Option<WebGlBuffer>,
    quad_ebo: Option<WebGlBuffer>,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    canvas_width: u32,
    canvas_height: u32,
}

impl Renderer {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;
        
        // Enable blending for transparency
        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        
        // Create sprite shader
        let sprite_shader = ShaderProgram::new(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
        
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();
        
        // Create projection matrix (orthographic)
        let projection_matrix = Mat4::orthographic(0.0, canvas_width as f32, canvas_height as f32, 0.0, -1.0, 1.0);
        let view_matrix = Mat4::identity();
        
        let mut renderer = Self {
            gl,
            sprite_shader,
            quad_vao: None,
            quad_vbo: None,
            quad_ebo: None,
            projection_matrix,
            view_matrix,
            canvas_width,
            canvas_height,
        };
        
        renderer.setup_quad_geometry()?;
        
        Ok(renderer)
    }
    
    fn setup_quad_geometry(&mut self) -> Result<(), JsValue> {
        // Quad vertices (position + texture coordinates)
        let vertices: [f32; 20] = [
            // positions   // texture coords
             0.0,  1.0,     0.0, 1.0,  // top left
             1.0,  1.0,     1.0, 1.0,  // top right
             1.0,  0.0,     1.0, 0.0,  // bottom right
             0.0,  0.0,     0.0, 0.0,  // bottom left
        ];
        
        let indices: [u16; 6] = [
            0, 1, 2,  // first triangle
            2, 3, 0   // second triangle
        ];
        
        // Create VAO
        let vao = self.gl.create_vertex_array()
            .ok_or("Failed to create VAO")?;
        self.gl.bind_vertex_array(Some(&vao));
        
        // Create VBO
        let vbo = self.gl.create_buffer()
            .ok_or("Failed to create VBO")?;
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
        
        unsafe {
            let vertex_array = js_sys::Float32Array::view(&vertices);
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertex_array,
                WebGl2RenderingContext::STATIC_DRAW
            );
        }
        
        // Create EBO
        let ebo = self.gl.create_buffer()
            .ok_or("Failed to create EBO")?;
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        
        unsafe {
            let index_array = js_sys::Uint16Array::view(&indices);
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &index_array,
                WebGl2RenderingContext::STATIC_DRAW
            );
        }
        
        // Setup vertex attributes
        // Position attribute (location = 0)
        self.gl.vertex_attrib_pointer_with_i32(
            0,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            4 * 4, // stride: 4 floats * 4 bytes
            0,     // offset
        );
        self.gl.enable_vertex_attrib_array(0);
        
        // Texture coordinate attribute (location = 1)
        self.gl.vertex_attrib_pointer_with_i32(
            1,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            4 * 4, // stride: 4 floats * 4 bytes
            2 * 4, // offset: 2 floats * 4 bytes
        );
        self.gl.enable_vertex_attrib_array(1);
        
        self.quad_vao = Some(vao);
        self.quad_vbo = Some(vbo);
        self.quad_ebo = Some(ebo);
        
        // Unbind
        self.gl.bind_vertex_array(None);
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        
        Ok(())
    }
    
    pub fn clear(&self) {
        self.gl.viewport(0, 0, self.canvas_width as i32, self.canvas_height as i32);
        self.gl.clear_color(0.2, 0.3, 0.3, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
    
    pub fn present(&self) {
        // WebGL automatically presents the frame
    }
    
    pub fn render_sprite(&self, entity: Entity, world: &World) {
        let transform = world.get_component::<Transform>(entity);
        let sprite = world.get_component::<Sprite>(entity);
        
        if let (Some(transform), Some(sprite)) = (transform, sprite) {
            // Use sprite shader
            self.sprite_shader.use_program(&self.gl);
            
            // Calculate model matrix
            let model_matrix = Mat4::translation(transform.x, transform.y, 0.0)
                * Mat4::rotation_z(transform.rotation)
                * Mat4::scaling(sprite.width * transform.scale_x, sprite.height * transform.scale_y, 1.0);
            
            // Set uniforms
            let mvp_matrix = self.projection_matrix * self.view_matrix * model_matrix;
            self.sprite_shader.set_mat4(&self.gl, "u_mvp", &mvp_matrix);
            self.sprite_shader.set_vec4(&self.gl, "u_color", sprite.color_r, sprite.color_g, sprite.color_b, sprite.color_a);
            
            // Bind VAO and draw
            self.gl.bind_vertex_array(self.quad_vao.as_ref());
            self.gl.draw_elements_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                6,
                WebGl2RenderingContext::UNSIGNED_SHORT,
                0
            );
        }
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.canvas_width = width;
        self.canvas_height = height;
        self.projection_matrix = Mat4::orthographic(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
    }
}

const VERTEX_SHADER_SOURCE: &str = r#"#version 300 es
layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 u_mvp;

out vec2 vTexCoord;

void main() {
    gl_Position = u_mvp * vec4(aPosition, 0.0, 1.0);
    vTexCoord = aTexCoord;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"#version 300 es
precision mediump float;

in vec2 vTexCoord;
uniform vec4 u_color;

out vec4 fragColor;

void main() {
    fragColor = u_color;
}
"#;
