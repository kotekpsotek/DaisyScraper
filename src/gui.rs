use fltk::{ self, prelude::* }; // GUI Library: Fast Light ToolKit

pub fn create()
{
    let app_ = fltk::app::App::default();
    let mut window_ = fltk::window::Window::new(150, 300, 1000, 1000, "Daisy Scraper");
    window_.end();
    window_.show();
    app_.run().unwrap();
}