use imgui::Ui;

use crate::renderer::Texture2D;

use super::Widget;

#[derive(Clone)]
pub struct Combo<T> where T: std::fmt::Display, T: Clone
{
    name: String,
    items: Vec<T>,
    selected: T
}

impl<T> Combo<T> where T: std::fmt::Display, T: Clone {
    pub fn new(name: &str, items: Vec<T>) -> Self {
        Self { name: String::from(name), selected: items[0].clone(), items: items  }
    }
    
    pub fn get_selected(&self) -> T {
        self.selected.clone()
    }
}

impl<T> Widget for Combo<T> where T: std::fmt::Display, T: Clone {
    fn on_render(&mut self, frame: &Ui) {
        frame.text(self.name.as_str());
        let hiden_name = String::from("##") + self.name.as_str();
        let combo_token = frame.begin_combo(hiden_name.as_str(), self.selected.to_string());
        match combo_token {
            Some(token) => {
                for item in self.items.iter() {
                    if frame.menu_item(item.to_string()) {
                        self.selected = item.clone();
                    }
                }
                token.end();
            },
            _ => {}
        }
    }
}

pub struct Label {
    text: String
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self { text: String::from(text) }
    }

    pub fn change_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
}

impl Widget for Label {
    fn on_render(&mut self, frame: &Ui) {
        frame.text(self.text.as_str())
    }
}

// pub struct Viewport {
//     frame: Texture2D
// }

// impl Viewport {
//     pub fn new(frame: Texture2D) -> Self {
//         Self { frame: frame }
//     }
// }

// impl Widget for Viewport {
//     fn on_render(&mut self, frame: &Ui) {
           
//     }
// }

pub struct Button {
    label: String,
    action: Box<dyn FnMut()>
}

impl Button {
    pub fn new(label: &str, action: impl FnMut() + 'static) -> Self {
        Self { label: String::from(label), action: Box::new(action) }
    }
}

impl Widget for Button {
    fn on_render(&mut self, frame: &Ui) {
        if frame.button(self.label.as_str()) {
            (self.action)();
        }
    }
}