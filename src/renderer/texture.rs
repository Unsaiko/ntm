use std::ffi::c_void;
use image::EncodableLayout;

use super::Renderer;

pub struct Texture2D {
    id: u32,
    width: u32,
    height: u32,
}

impl Texture2D {
    pub fn new() -> Texture2D{
        let mut tmp = Texture2D {id: 0, width: 0, height: 0};
        unsafe {
            gl::GenTextures(1,  &mut tmp.id);
        }

        tmp
    }

    pub fn get_id(&self) -> u32{
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn from_memory(width: u32, height: u32, _format: gl::types::GLuint, data: &[u8]) -> Texture2D {
        let mut tmp = Texture2D::new();
        tmp.width = width;
        tmp.height = height;

        tmp.bind();
        unsafe{
            gl::TexImage2D(gl::TEXTURE_2D, 0, _format as i32, width as i32, height as i32, 0,
                _format, gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
                gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        tmp
    }

    pub fn load_from_file(&mut self, filepath: &str) {
        let image = image::open(filepath)
            .expect("Failed to load image file!").flipv()
            .into_rgba8();

        self.width = image.width();
        self.height = image.height();

        self.bind();
        
        unsafe{
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.width as i32, self.height as i32, 0,
                 gl::RGBA, gl::UNSIGNED_BYTE, image.as_bytes().as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) }
    }
}

pub struct Cubemap {
    id: u32
}

impl Cubemap {
    pub fn new() -> Cubemap {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Cubemap { id: id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id);
        }
    }
    
    pub fn load(&self, images: [&str; 6]) {
        self.bind();
        let image = image::open(images[0])
        .expect("Failed to load image file!").flipv()
        .into_rgb8();
        let width = image.width();
        let height = image.height();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_CUBE_MAP_NEGATIVE_X, 0, gl::RGB as i32, width as i32, height as i32, 0,
                gl::RGB, gl::UNSIGNED_BYTE, image.as_bytes().as_ptr() as *const c_void);
        }
        let image = image::open(images[1])
        .expect("Failed to load image file!").flipv()
        .into_rgb8();
        let width = image.width();
        let height = image.height();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_CUBE_MAP_NEGATIVE_Y, 0, gl::RGB as i32, width as i32, height as i32, 0,
                gl::RGB, gl::UNSIGNED_BYTE, image.as_bytes().as_ptr() as *const c_void);
        }
        let image = image::open(images[2])
        .expect("Failed to load image file!").flipv()
        .into_rgb8();
        let width = image.width();
        let height = image.height();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_CUBE_MAP_NEGATIVE_Z, 0, gl::RGB as i32, width as i32, height as i32, 0,
                gl::RGB, gl::UNSIGNED_BYTE, image.as_bytes().as_ptr() as *const c_void);
        }
        let image = image::open(images[3])
        .expect("Failed to load image file!").flipv()
        .into_rgb8();
        let width = image.width();
        let height = image.height();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X, 0, gl::RGB as i32, width as i32, height as i32, 0,
                gl::RGB, gl::UNSIGNED_BYTE, image.as_bytes().as_ptr() as *const c_void);
        }
        let image = image::open(images[4])
        .expect("Failed to load image file!").flipv()
        .into_rgb8();
        let width = image.width();
        let height = image.height();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_Y, 0, gl::RGB as i32, width as i32, height as i32, 0,
                gl::RGB, gl::UNSIGNED_BYTE, image.as_bytes().as_ptr() as *const c_void);
        }
        let image = image::open(images[5])
        .expect("Failed to load image file!").flipv()
        .into_rgb8();
        let width = image.width();
        let height = image.height();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_Z, 0, gl::RGB as i32, width as i32, height as i32, 0,
                gl::RGB, gl::UNSIGNED_BYTE, image.as_bytes().as_ptr() as *const c_void);
        }

        Renderer::tex_parametr(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        Renderer::tex_parametr(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        Renderer::tex_parametr(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
        Renderer::tex_parametr(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
        Renderer::tex_parametr(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE);
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}