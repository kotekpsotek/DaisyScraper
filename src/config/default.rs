use fltk::{ enums::Font };

pub struct Setting {
    pub font: String
}

impl Setting {
    pub fn bars_default() -> Self {
        Self {
            font: Font::load_font("fonts/robosapien.ttf").unwrap()
        }
    }
}