use fltk::{self, button::Button, enums::{ Color, Cursor, Align, FrameType }, prelude::*, image}; // GUI Library: Fast Light ToolKit
use fltk_flex::{ Flex, FlexType };
use fltk_theme::*;
use fltk_evented::{ Listener };

pub fn create() {
    let app_ = fltk::app::App::default();
    let mut wn_ = fltk::window::Window::new(0, 0, 900, 900, "Daisy Scraper");
    // let theme = WidgetTheme::new(ThemeType::AquaClassic);
    // theme.apply();
    wn_.set_color(Color::from_rgb(2, 7, 46));
    
    top_buttons(&mut wn_); // create top buttons

    wn_.end();
    wn_.show();
    app_.run().unwrap();
}

fn top_buttons(window: &mut fltk::window::Window)
{
    // Button size tuple type
    let btn_size = (150, 55); // x, y
    let button_color = Color::rgb_color(48, 55, 110);

    // Container for buttons
    let mut f_con = Flex::default().with_size(btn_size.0 * 2 + 50, btn_size.1).row();
    let f_center_parent_x = f_con.clone().center_x(&*window).x() + (120 / 2);
    f_con.set_pos(f_center_parent_x, 50);
    f_con.set_type(FlexType::Row);

    // Add buttons to container
    let mut search_btn = Button::default().center_of(&mut f_con).with_label("C");
    let search_btn_img = image::SvgImage::load("svg/search.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "search" ("search.svg")"#);
    search_btn.set_label("Search");
    search_btn.set_label_color(Color::White);
    search_btn.set_frame(widget_themes::OS_SPACER_THIN_DOWN_BOX);
    search_btn.set_color(button_color);
    search_btn.clear_visible_focus();

    let mut menu_btn = Button::default().center_of(&mut f_con);
    let menu_btn_img = image::SvgImage::load("svg/menu.svg").expect(r#"Cound't load menu icon from folder ./svg. Add svg file which is svg file and his name is "menu" ("menu.svg")"#);
    menu_btn.set_label("Menu");
    menu_btn.set_label_color(Color::White);
    menu_btn.set_frame(widget_themes::OS_SPACER_THIN_DOWN_BOX);
    menu_btn.set_color(button_color);
    menu_btn.clear_visible_focus();

    // Set immages to buttons
    search_btn.set_image(Some(search_btn_img));
    menu_btn.set_image(Some(menu_btn_img));

    // Size of buttons
    f_con.set_size(&mut search_btn, btn_size.0); // width: of the button
    f_con.set_size(&mut menu_btn, btn_size.0); // width: of the button

    // Listen events on buttons
    let mut menu_btn_listener: Listener<_> = menu_btn.clone().into();
    let mut search_btn_listener: Listener<_> = search_btn.clone().into();

    menu_btn_listener.on_hover(|btn| {
        fltk::draw::set_cursor(Cursor::Hand);
        btn.set_color(btn.color().darker());
    });

    menu_btn_listener.on_leave({ 
        let button_color_c  = button_color.clone();
        move |btn| {
            fltk::draw::set_cursor(Cursor::Default);
            btn.set_color(button_color_c);
        }
    });

    search_btn_listener.on_hover(|btn| {
        fltk::draw::set_cursor(Cursor::Hand);
        btn.set_color(btn.color().darker());
    });

    search_btn_listener.on_leave({ 
        let button_color_c  = button_color.clone();
        move |btn| {
            fltk::draw::set_cursor(Cursor::Default);
            btn.set_color(button_color_c);
        }
    });


    // Add changes from code
    f_con.end();
}