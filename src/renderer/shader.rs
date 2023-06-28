use std::fs::File;
use std::ffi::CString;
use std::io::Read;

#[derive(Clone)]
pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn create(path: &str, _type: u32) -> Result<Shader, String> {
        let tmp = Shader { id: unsafe { gl::CreateShader(_type) } };
        let mut shader_file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(e.to_string())
        };

        let mut shader_string = String::new();
        shader_file.read_to_string(&mut shader_string).expect("Failed write file to string"); 
        let shader_source = CString::new(shader_string.as_bytes()).expect("Failed convert shader source to CString");

        let mut result = 0;
        unsafe {
            gl::ShaderSource(tmp.id, 1, &shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(tmp.id);

            gl::GetShaderiv(tmp.id, gl::COMPILE_STATUS, &mut result);
        }

        if result != 1 {
            let mut error_len = 0;
            let mut _error = String::new();

            unsafe {
                gl::GetShaderiv(tmp.id, gl::INFO_LOG_LENGTH, &mut error_len);
                let mut message = Vec::<u8>::with_capacity(error_len as usize);
                gl::GetShaderInfoLog(tmp.id, error_len, &mut error_len,
                    message.as_mut_ptr() as *mut _);
                message.set_len(error_len as usize);
                _error = String::from_utf8(message).expect("Failed convert error message to string");
            }
            return Err(_error);
        }

        Ok(tmp)
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn delete(&self) {
        unsafe{ gl::DeleteShader(self.id)}
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.delete()
    }
}