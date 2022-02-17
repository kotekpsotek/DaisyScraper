use fltk::enums::{ Font, Color };

pub struct Setting {
    pub font: String, // Font which is using in all elements from application
    pub element_font_color: Color, // Font colot for elements
    pub app_backround_color: Color, // Application background color
    pub btn_element_background_color: Color, // Color for elements like: Buttons
    pub inp_element_background_color: Color, // Color for elements like: Inputs elements and text areas,
    pub fr_element_background_color: Color // Color for elements like: Frames which are in two menu ("Search" and "Menu")
}

impl Setting {
    pub fn app_default() -> Self {
        Self {
            font: Font::load_font("fonts/robosapien.ttf").unwrap(),
            element_font_color: Color::White,
            app_backround_color: Color::from_rgb(2, 7, 46),
            btn_element_background_color: Color::from_rgb(48, 55, 110),
            inp_element_background_color: Color::from_rgb(32, 39, 89),
            fr_element_background_color: Color::White
        }
    }
}