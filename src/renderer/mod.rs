use crate::application::Window;

mod shader;
mod shader_program;
mod buffer;
mod vertex_array;
mod uniform;
mod camera;
mod texture;
mod framebuffer;

pub use {
    self::shader::Shader,
    self::shader_program::ShaderProgram,
    self::buffer::Buffer,
    self::vertex_array::VertexArray,
    self::uniform::Uniform,
    self::camera::Camera,
    self::texture::Texture2D,
    self::texture::Cubemap,
    self::framebuffer::Framebuffer,
};

pub struct Renderer;

impl Renderer
{
    pub fn load_func(window: &Window) {
        gl::load_with(|symbol| window.gl_get_proc_address(symbol) as *const _);
    }
    
    pub fn enable(bit: gl::types::GLenum) {
        unsafe {
            gl::Enable(bit)
        }
    }
    
    pub fn viewport(x:i32, y:i32, width:i32, height:i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }
    
    pub fn set_clear_color(r: f32, g:f32, b:f32, a:f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }
    
    pub fn clear(mask: gl::types::GLbitfield) {
        unsafe {
            gl::Clear(mask);
        }
    }
    
    pub fn draw_arrays(mode:gl::types::GLenum, first:i32, count:i32) {
        unsafe {
            gl::DrawArrays(mode, first, count);
        }
    }
    
    pub fn patch_parameter(name: gl::types::GLenum, value: i32) {
        unsafe {
            gl::PatchParameteri(name, value);
        }
    }
    
    pub fn draw_elements(mode: gl::types::GLenum, count: i32, type_:gl::types::GLenum) {
        unsafe {
            gl::DrawElements(mode, count, type_, std::ptr::null())
        }
    }
    
    pub fn tex_parametr(target: gl::types::GLenum, param: gl::types::GLenum, value: u32) {
        unsafe {
            gl::TexParameteri(target, param, value as i32);
        }
    }
    
    pub fn activate_texture(slot: gl::types::GLenum) {
        unsafe {
            gl::ActiveTexture(slot);
        }
    }
}