use crate::renderer::Shader;

#[derive(Clone)]
pub struct ShaderProgram {
    id: u32
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram { id: unsafe{ gl::CreateProgram() } }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn detach_shader(&self, shader: Shader) {
        unsafe { gl::DetachShader(self.id, shader.get_id()) };
    }

    pub fn attach_shader(&self, shader: Shader) {
        unsafe { gl::AttachShader(self.id, shader.get_id()) }
    }
    
    pub fn link_program(&self) -> Option<String> {
        let mut _err_message = String::new();
        unsafe {
            gl::LinkProgram(self.id);
            let mut result = 0;
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut result);
            if result != 1 {
                let mut err_length = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut err_length);
                let mut buffer = Vec::with_capacity(err_length as usize);
                gl::GetProgramInfoLog(self.id, err_length, &mut err_length, buffer.as_mut_ptr() as *mut _);
                buffer.set_len(err_length as usize);
                _err_message = String::from_utf8(buffer).unwrap();
                return Some(_err_message)
            }
        }
        None
    }

    pub fn get_id(&self) -> u32 {
        self.id.clone()
    }
}