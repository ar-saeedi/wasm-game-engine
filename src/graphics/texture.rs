use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlTexture, HtmlImageElement};

pub struct Texture {
    id: WebGlTexture,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, JsValue> {
        let texture = gl.create_texture()
            .ok_or("Failed to create texture")?;
        
        Ok(Self {
            id: texture,
            width: 0,
            height: 0,
        })
    }
    
    pub fn from_image(gl: &WebGl2RenderingContext, image: &HtmlImageElement) -> Result<Self, JsValue> {
        let texture = gl.create_texture()
            .ok_or("Failed to create texture")?;
            
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        
        // Set texture parameters
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32
        );
        
        // Upload image data
        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            image
        )?;
        
        let width = image.width();
        let height = image.height();
        
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        
        Ok(Self {
            id: texture,
            width,
            height,
        })
    }
    
    pub fn from_data(gl: &WebGl2RenderingContext, data: &[u8], width: u32, height: u32) -> Result<Self, JsValue> {
        let texture = gl.create_texture()
            .ok_or("Failed to create texture")?;
            
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        
        // Set texture parameters
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32
        );
        
        unsafe {
            let data_array = js_sys::Uint8Array::view(data);
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                width as i32,
                height as i32,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(&data_array)
            )?;
        }
        
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        
        Ok(Self {
            id: texture,
            width,
            height,
        })
    }
    
    pub fn create_white_pixel(gl: &WebGl2RenderingContext) -> Result<Self, JsValue> {
        let data = [255u8, 255, 255, 255]; // White pixel
        Self::from_data(gl, &data, 1, 1)
    }
    
    pub fn bind(&self, gl: &WebGl2RenderingContext, slot: u32) {
        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + slot);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.id));
    }
    
    pub fn unbind(&self, gl: &WebGl2RenderingContext) {
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn id(&self) -> &WebGlTexture {
        &self.id
    }
}
