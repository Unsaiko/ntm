use sdl2::{Sdl, video::{GLContext}, EventPump, event::Event};
use crate::logger::Logger;

pub struct Window {
    width: u32,
    height: u32,
    _sdl_context: Sdl,
    window_handle: sdl2::video::Window,
    event_pump: EventPump
}

impl Window {
    pub fn create(title: &str, width: u32, height: u32) -> Self where Self: Sized {
        let sdl = match sdl2::init() {
            Ok(context) => { context },
            Err(e) => {  
                Logger::error_box("Sdl context", &e);
            }
        };

        let video_sub = match sdl.video() {
            Ok(subsystem) => { subsystem },
            Err(e) => { 
                Logger::error_box("Video subsystem", &e);
            }
        };

        video_sub.gl_attr().set_context_profile(sdl2::video::GLProfile::Core);
        video_sub.gl_attr().set_context_version(4, 0);

        let window = match video_sub.window(title, width, height).position_centered().resizable()
            .opengl().build() {
                Ok(wind) => { wind },
                Err(e) => {
                    Logger::error_box("Window", &e.to_string());
                }
            };

        let event_pump = sdl.event_pump().unwrap();
        Window { width: width, height: height, _sdl_context: sdl,
            window_handle: window, event_pump}
    }

    pub fn get_handle(&self) -> &sdl2::video::Window {
        &self.window_handle
    }

    pub fn get_event_pump(&self) -> &EventPump {
        &self.event_pump
    }

    pub fn take_event<F>(&mut self, mut f: F) where F: FnMut(Event) {
        for event in self.event_pump.poll_iter() {
            f(event);
        }
    }

    pub fn gl_get_proc_address(&self, procname: &str) -> *const () {
        self.window_handle.subsystem().gl_get_proc_address(procname)
    }

    pub fn get_width(&self) -> u32 {
        self.width.clone()
    }

    pub fn get_height(&self) -> u32 {
        self.height.clone()
    }

    pub fn create_gl_context(&self) -> GLContext {
        match self.window_handle.gl_create_context() {
            Ok(context) => { context },
            Err(e) => {
                Logger::error_box("GLContext", &e);
            }
        }
    }

    pub fn enable_vsync(&self, value: bool) {
        self.window_handle.subsystem().gl_set_swap_interval(value as i32).unwrap();
    }

    pub fn swap_buffers(&self) {
        self.window_handle.gl_swap_window();
    }
}