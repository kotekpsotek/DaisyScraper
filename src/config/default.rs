use fltk::enums::{Color, Font};
use std::{fs, path::Path};
use {
    serde::{Deserialize, Serialize},
    serde_json,
};

pub struct Setting {
    pub font: String, // Font which is using in all elements from application
    pub element_font_color: Color, // Font colot for elements
    pub app_backround_color: Color, // Application background color
    pub btn_element_background_color: Color, // Color for elements like: Buttons
    pub inp_element_background_color: Color, // Color for elements like: Inputs elements and text areas,
    pub fr_element_background_color: Color, // Color for elements like: Frames which are in two menu ("Search" and "Menu")
    pub fr_elements_top_bar_background_color: Color, // Color for elements which are in frame top bar like: Select All button
}

// Remember: This structure is only more redable representation for keys which are into gui.json file (that is created for empowerment for user better understand what that key change) // This structure shoudn't extends Settings structure for other keys before when that in Settings struct that keys aren't added first
#[derive(Serialize, Deserialize)]
struct GuiSettings {
    font_file_name: String, // name of custom font in "fonts" dir
    font_color: (u8, u8, u8), // Font colot for elements
    main_backround_color_rgb: (u8, u8, u8), // Application background color
    buttons_1st_backround_color_rgb: (u8, u8, u8), // Color for elements like: Buttons
    input_backround_color_rgb: (u8, u8, u8), // Color for elements like: Inputs elements and text areas,
    frame_background_color_rgb: (u8, u8, u8), // Color for elements like: Frames which are in two menu ("Search" and "Menu")
    frame_top_bar_elements_background_color: (u8, u8, u8) // Color for elements which are in frame top bar like: Select All button
}

impl Setting {
    pub fn app_default() -> Self {
        let settings_from_file = Self::load_settings_from_file();
        match settings_from_file {
            Ok(from_file) => {
                Self {
                    font: Font::load_font(format!("fonts/{}", from_file.font_file_name)).unwrap(),
                    element_font_color: Color::from_rgb(from_file.font_color.0,from_file.font_color.1, from_file.font_color.2),
                    app_backround_color: Color::from_rgb(from_file.main_backround_color_rgb.0,from_file.main_backround_color_rgb.1, from_file.main_backround_color_rgb.2),
                    btn_element_background_color: Color::from_rgb(from_file.buttons_1st_backround_color_rgb.0,from_file.buttons_1st_backround_color_rgb.1, from_file.buttons_1st_backround_color_rgb.2),
                    inp_element_background_color: Color::from_rgb(from_file.input_backround_color_rgb.0,from_file.input_backround_color_rgb.1, from_file.input_backround_color_rgb.2),
                    fr_element_background_color: Color::from_rgb(from_file.frame_background_color_rgb.0,from_file.frame_background_color_rgb.1, from_file.frame_background_color_rgb.2),
                    fr_elements_top_bar_background_color: Color::rgb_color(from_file.frame_top_bar_elements_background_color.0,from_file.frame_top_bar_elements_background_color.1, from_file.frame_top_bar_elements_background_color.2),
                }
            },
            Err(err) => {
                println!("Program coudn't load your GUI settings from file. Error reason: {}", err);
                Self {
                    font: Font::load_font("fonts/robosapien.ttf").unwrap(),
                    element_font_color: Color::White,
                    app_backround_color: Color::from_rgb(2, 7, 46),
                    btn_element_background_color: Color::from_rgb(48, 55, 110),
                    inp_element_background_color: Color::from_rgb(32, 39, 89),
                    fr_element_background_color: Color::from_rgb(48, 55, 110),
                    fr_elements_top_bar_background_color: Color::rgb_color(32, 39, 89),
                }
            }
        }
    }

    fn load_settings_from_file() -> Result<GuiSettings, String> {
        let path = format!("settings/gui.json");
        let path_to_gui_settings = Path::new(&path);
        if path_to_gui_settings.exists() {
            let read_file = fs::read_to_string(&path_to_gui_settings);
            let file_content = if let Err(err) = read_file {
                return Err(err.to_string());
            } else {
                read_file.unwrap()
            };
            match serde_json::from_str(file_content.as_str()) {
                Ok(data) => Ok(data),
                Err(err) => Err(err.to_string()),
            }
        } else {
            Err("path_doesn't_exists".to_string())
        }
    }
}
