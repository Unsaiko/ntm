use std::ffi::c_void;


#[derive(Clone)]
pub struct VertexArray {
    id: u32
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut ret = VertexArray{id: 0};
        unsafe
        {
            gl::GenVertexArrays(1, &mut ret.id);
        }
        ret
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn disable_vertex_attrib_array(index: u32) {
        unsafe{ gl::DisableVertexAttribArray(index) }
    }

    pub fn setup_vertex_attrib(index: u32, size: i32, type_: gl::types::GLenum,
        normalized: gl::types::GLboolean, stride: gl::types::GLsizei, offset: usize) {
            unsafe {
                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(index, size, type_, normalized, stride, offset as *const c_void);
            }
        }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id)
        }
    }
}