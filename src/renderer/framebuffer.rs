use super::Renderer;

#[derive(Clone, Copy)]
pub struct Framebuffer {
    width: u32,
    height: u32,
    color_texture: u32,
    depth_texture: u32,
    id: u32
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Framebuffer {
        let mut ret = Framebuffer {width: width, height: height, color_texture: 0, depth_texture: 0, id: 0};
        unsafe {
            gl::CreateFramebuffers(1, &mut ret.id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, ret.id);

            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut ret.color_texture);
            gl::BindTexture(gl::TEXTURE_2D, ret.color_texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, ret.width as i32, ret.height as i32,
                0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
            Renderer::tex_parametr(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            Renderer::tex_parametr(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D,
            ret.color_texture, 0);

            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut ret.depth_texture);
            gl::BindTexture(gl::TEXTURE_2D, ret.depth_texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH24_STENCIL8 as i32, ret.width as i32, ret.height as i32,
                0, gl::DEPTH_STENCIL, gl::UNSIGNED_INT_24_8, std::ptr::null());
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D,
                ret.depth_texture, 0);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        ret
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.depth_texture);
            gl::DeleteTextures(1, &self.color_texture);
            gl::DeleteFramebuffers(1, &self.id);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.id > 0 {
            self.delete();
        }
        
        let tmp = Framebuffer::new(width, height);
        self.id = tmp.id;
        self.width = tmp.width;
        self.height = tmp.height;
        self.color_texture = tmp.color_texture;
        self.depth_texture = tmp.depth_texture;
    }

    pub fn get_width(&self) -> f32 {
        self.width as f32
    }

    pub fn get_height(&self) -> f32 {
        self.height as f32
    }

    pub fn get_color_texture(&self) -> u32 {
        self.color_texture.clone()
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}