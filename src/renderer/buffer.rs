#[derive(Clone)]
pub struct Buffer {
    id: u32,
    size: usize,
    target_type: gl::types::GLenum,
}

impl Buffer {
    pub fn new(target_type: gl::types::GLenum) -> Buffer {
        let mut ret = Buffer { id: 0, size: 0, target_type: target_type};
        unsafe { gl::GenBuffers(1, &mut ret.id)};

        ret
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.target_type, self.id) }
    }

    pub fn set_data<T>(&mut self, data: &[T], usage: gl::types::GLenum) {
        self.size = data.len();
        self.bind();
        unsafe {
            gl::BufferData(self.target_type,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const std::ffi::c_void, usage);
        }
    }

    pub fn get_size(&self) -> usize {
        self.size.clone()
    }
}
