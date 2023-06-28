use std::{mem};

use imgui::{TextureId};
use sdl2::{event::Event, video::GLContext};
use crate::{renderer::*, generator::{Terrain, Heightmap}, ui::{UI, controls::Combo, Widget}, modifiers::{HeightmapModifier, PerlinModifier}};

use super::{ Window };

pub struct Application {
    window: Window,
    should_close: bool,
    _gl_context: GLContext,
    heightmap: Heightmap,
    heightmap_texture: Texture2D,
    framebuffer: Framebuffer,
    terrain: Terrain,
    terrain_vao: VertexArray,
    terrain_program: ShaderProgram,
    terrain_uniform: Uniform,
    terrain_height: f32,
    camera: Camera,
    viewport_selected: bool,
    ui: UI,
    modifier: Box<dyn HeightmapModifier>,
    combo: Combo<f32>,
}

impl Application {
    pub fn create(title: &str, width: u32, height: u32) -> Application {
        let window = Window::create(title, width, height);
        let gl_context = window.create_gl_context();
        Renderer::load_func(&window);
        let shader_program = ShaderProgram::new();
        let ui = UI::init(&window);

        Application { window: window, should_close: false, _gl_context: gl_context,
            heightmap: Heightmap::new(), terrain_vao: VertexArray::new(),
            terrain_uniform: Uniform::new(shader_program.get_id()), terrain_program: shader_program,
            camera: Camera::new(1.77), framebuffer: Framebuffer::new(width, height), ui: ui,
            terrain_height: 20.0, heightmap_texture: Texture2D::new(), terrain: Terrain::new(32, 32), viewport_selected: true,
            modifier:  Box::new(PerlinModifier::new(128f32, 10)),
            combo: Combo::new("Test", vec![32.1, 21.5, 33.3, 15.2])
        }
    }
    
    pub fn init(&mut self) {
        self.window.enable_vsync(true);
        Renderer::viewport(0, 0, self.window.get_width() as i32, self.window.get_height() as i32);
        Renderer::set_clear_color(0.3, 0.0, 0.8, 1.0);
        Renderer::enable(gl::DEPTH_TEST);

        Renderer::tex_parametr(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
        Renderer::tex_parametr(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);
        Renderer::tex_parametr(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR);
        Renderer::tex_parametr(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
    
        let vertex_shader = match Shader::create("shaders/terrain.vert", gl::VERTEX_SHADER) {
            Ok(shader) => shader,
            Err(e) => panic!("Vertex Shader {} ", e)
        };
        let tesc_shader = match Shader::create("shaders/terrain.tesc", gl::TESS_CONTROL_SHADER) {
            Ok(shader) => shader,
            Err(e) => panic!("Tess Control Shader {} ", e)
        };
        let tesev_shader = match Shader::create("shaders/terrain.tesv", gl::TESS_EVALUATION_SHADER) { 
            Ok(shader) => shader, 
            Err(e) => panic!("Tess Eval Shader {} ", e)
        };
        let fragment_shader = match Shader::create("shaders/terrain.frag", gl::FRAGMENT_SHADER) {
            Ok(shader) => shader,
            Err(e) => panic!("Fragmet shader {} ", e)
        };
     
        self.terrain_program.attach_shader(vertex_shader);
        self.terrain_program.attach_shader(tesc_shader);
        self.terrain_program.attach_shader(tesev_shader);
        self.terrain_program.attach_shader(fragment_shader);
        
        match self.terrain_program.link_program() {
            Some(e) => panic!("Failed link program: {} ", e),
            _ => {}
        }

        self.terrain_vao.bind();
        VertexArray::setup_vertex_attrib(0, 3, gl::FLOAT, gl::FALSE, (5 * mem::size_of::<f32>()) as i32, 0);
        VertexArray::setup_vertex_attrib(1, 2, gl::FLOAT, gl::FALSE, (5 * mem::size_of::<f32>()) as i32, 3 * mem::size_of::<f32>());
        self.terrain_vao.unbind();
        
        self.heightmap_texture = self.heightmap.as_texture2d();
        
        Renderer::patch_parameter(gl::PATCH_VERTICES, 4);
        self.terrain_uniform.set_float("params.height", self.terrain_height);
    }
    
    pub fn close(&self) -> bool {
        self.should_close
    }

    fn draw(&mut self) {
        self.terrain_vao.bind();
        self.heightmap_texture.bind();
        self.terrain_program.bind();
        self.terrain_uniform.set_matrix4f("coords.model", self.terrain.get_model_matrix());
        self.terrain_uniform.set_matrix4f("coords.view", self.camera.get_view_matrix());
        self.terrain_uniform.set_matrix4f("coords.projection", self.camera.get_proj_matrix());

        self.terrain.get_ind_buffer().bind();
        Renderer::draw_elements(gl::PATCHES, self.terrain.get_ind_buffer().get_size() as i32, gl::UNSIGNED_INT);
    }

    fn draw_ui(&mut self) {
        let frame = self.ui.frame().unwrap();

        frame.window("Generic").build(|| {
            if imgui::CollapsingHeader::new("Heightmap").build(&frame) {
                frame.text("Resolution:");
                let combo = frame.begin_combo("##heightmapsize", self.heightmap.size().to_string());
                let mut selected = self.heightmap.size();
                match combo {
                    Some(com) => {
                        for item in self.heightmap.get_sizes() {
                            if frame.menu_item(item.to_string()) {
                                selected = *item;
                            }
                        }
                        com.end();
                    },
                    _ => {}
                }

                if selected != self.heightmap.size() {
                    self.heightmap.set_size(selected);
                }
                
                frame.separator();
                if frame.button("Generate") {
                    self.modifier.modify(&mut self.heightmap);
                    self.heightmap_texture = self.heightmap.as_texture2d();
                }
    
                if frame.button("Save as") {
                    let path = rfd::FileDialog::default()
                    .set_file_name("heightmap")
                    .add_filter("PNG", &["png"])
                    .add_filter("JPEG", &["jpg"])
                    .save_file();
    
                    match path {
                        Some(p) => { self.heightmap.save(p.to_str().unwrap()) },
                        _=> {}
                    }
                }
            }
            if imgui::CollapsingHeader::new("Algorithm settings").build(&frame) {
                self.modifier.on_render(frame);
            }

            if imgui::CollapsingHeader::new("Terrain rendering").build(&frame) {
                frame.text("Height");
                if frame.slider("##terrainheight", 20.0, 100.0, &mut self.terrain_height) {
                    self.terrain_uniform.set_float("params.height", self.terrain_height);
                }
            }

            if imgui::CollapsingHeader::new("TEST!!!! settings").build(&frame) {
                self.combo.on_render(frame);
                frame.text(self.combo.get_selected().to_string());
            }
        });

        let style_var = frame.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
        frame.window("Viewport").build(|| {
            let size = frame.content_region_avail();
            if [self.framebuffer.get_width(), self.framebuffer.get_height()] != size {
                Renderer::viewport(0, 0, size[0] as i32, size[1] as i32);
                self.framebuffer.resize(size[0] as u32, size[1] as u32);
                self.camera.set_aspect(size[0] / size[1]);
            }
            self.viewport_selected = frame.is_window_hovered();
            imgui::Image::new(TextureId::new(self.framebuffer.get_color_texture() as usize),
                frame.content_region_avail()).build(&frame);
        });
        style_var.end();

    }

    pub fn render_iteration(&mut self) {
        self.framebuffer.bind();
        Renderer::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.draw();
        self.framebuffer.unbind();

        Renderer::set_clear_color(0.2, 0.2, 0.2, 1.0);
        Renderer::clear(gl::COLOR_BUFFER_BIT);
        self.ui.start_frame(&self.window);
        self.draw_ui();
        self.ui.end_frame();

        self.window.swap_buffers();
  
        self.window.take_event(|event| {
            self.ui.handle_event(&event);
            if self.viewport_selected {
                self.camera.event_dispatch(event.clone());
            }
            match event {
                Event::Quit { .. } => { self.should_close = true },
                _ => {}
            }
        });
    }
}