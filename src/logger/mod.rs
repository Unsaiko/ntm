use std::process::exit;

pub struct Logger {
    
}

impl Logger {
    pub fn info_box(source: &str, message: &str) {
        sdl2::messagebox::show_simple_message_box(sdl2::messagebox::MessageBoxFlag::INFORMATION,
            source, message, None).unwrap();
    }

    pub fn error_box(source: &str, message: &str) -> ! {
        sdl2::messagebox::show_simple_message_box(sdl2::messagebox::MessageBoxFlag::ERROR,
            source, message, None).unwrap();
            exit(-1);
    }
}