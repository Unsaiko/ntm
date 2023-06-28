use imgui::Ui;

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