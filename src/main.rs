#![windows_subsystem = "windows"]
use application::{Application};

mod renderer;
mod generator;
mod utils;
mod ui;
mod application;
mod logger;
mod modifiers;

fn main() {
    let mut app = Application::create("NTM", 800, 600);
    app.init();
    while !app.close() {
        app.render_iteration();
    }
}