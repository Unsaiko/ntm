use imgui::{ Ui, FontSource};
use imgui_opengl::Renderer;
use imgui_sdl2_support::SdlPlatform;

use sdl2::event::Event;

use crate::{ application::Window };

pub mod controls;

pub trait Widget {
    fn on_render(&mut self, frame: &Ui);
}
pub struct UI {
    imgui_context: imgui::Context, 
    imgui_platrofrm: SdlPlatform,
    imgui_renderer: Renderer,
    imgui_frame: Option<*mut imgui::Ui>,
}

impl UI {
    pub fn init(window: &Window) -> UI {
        let mut context = imgui::Context::create();
        
        context.io_mut().config_flags.insert(imgui::ConfigFlags::DOCKING_ENABLE);
        context.io_mut().config_flags.insert(imgui::ConfigFlags::VIEWPORTS_ENABLE);
        context.set_log_filename(None);
        
        context.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../../fonts/OpenSans-Regular.ttf"),
            size_pixels: 18.0,
            config: None,
        }]);
        
        let style = context.style_mut();
        
        style.window_border_size = 1.0;
        
        
        UI {imgui_platrofrm: SdlPlatform::init(&mut context), 
            imgui_renderer: Renderer::new(&mut context,
                |s| window.gl_get_proc_address(s) as *const _),
                imgui_context: context, imgui_frame: None }
    }

    pub fn frame(&self) -> Option<&mut Ui> {
        let frame = unsafe { self.imgui_frame.unwrap().as_mut() };
        match frame {
            Some(ui) => {
                ui.dockspace_over_main_viewport();
                Some(ui)
            },
            _ => None
        }
    }

    pub fn start_frame(&mut self, window: &Window) {
        self.imgui_platrofrm.prepare_frame(&mut self.imgui_context, &window.get_handle(),
            &window.get_event_pump());
        self.imgui_frame = Some(self.imgui_context.new_frame());
    }

    pub fn handle_event(&mut self, event: &Event) {
        self.imgui_platrofrm.handle_event(&mut self.imgui_context, &event);
    }

    pub fn end_frame(&mut self) {
        self.imgui_renderer.render(&mut self.imgui_context);
        self.imgui_context.update_platform_windows();
    }
}