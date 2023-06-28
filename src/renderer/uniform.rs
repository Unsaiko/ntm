use std::ffi::CString;
use glam::Mat4;

#[derive(Clone)]
pub struct Uniform {
    shader_program_id: u32
}

impl Uniform {
    pub fn new (shader_program: u32) -> Uniform {
        Uniform
        { 
            shader_program_id: shader_program
        }
    }
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let name_c_str = CString::new(name.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(self.shader_program_id,
                name_c_str.as_ptr());

            gl::ProgramUniform1f(self.shader_program_id, location, value);
        }
    }

    pub fn set_texture_order(&self, name: &str, texture: i32) {
        unsafe {
            let name_c_str = CString::new(name.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(self.shader_program_id,
                name_c_str.as_ptr());

            gl::ProgramUniform1i(self.shader_program_id, location, texture);
        }
    }

    pub fn set_matrix4f(&self, name: &str, mat4: Mat4) {
        unsafe {
            let name_c_str = CString::new(name.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(self.shader_program_id,
                name_c_str.as_ptr());

            gl::ProgramUniformMatrix4fv(self.shader_program_id, location, 1,
                gl::FALSE, mat4.to_cols_array().as_ptr());
        }
    }
}