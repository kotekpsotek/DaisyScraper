#[allow(unused_imports)]
use fltk::{
    self,
    button::Button,
    enums::{Align, Color, Cursor, FrameType},
    image::{ SvgImage },
    prelude::*,
    window::Window,
    frame::Frame,
    input::Input
}; // GUI Library: Fast Light ToolKit
use fltk_evented::Listener;
use fltk_flex::{ Flex, FlexType };
use fltk_theme::*;

const BUTTON_COLOR: (u8, u8, u8) = (48, 55, 110);
const INPUT_COLOR: (u8, u8, u8) = (32, 39, 89);

#[allow(dead_code)]
enum ElementType {
    Frame,
    Button,
    Input
}

struct TransferredStyleData {
    icon: Option<SvgImage>,
    color: Option<(u8, u8, u8)>,
    label: Option<&'static str>
}

struct SearchAction;
impl SearchAction {
    
    fn create(window: &mut Window) {
        SearchAction::create_search_field(window);
    }

    // Create Search Bar placed on top
    fn create_search_field(window: &mut Window) {
        // Container for Bar Elements
        let mut fl_container = Flex::default()
            .with_size(650, 55)
            .with_label("flex_container2")
            .with_pos(100, 120)
            .row()
            .center_x(&*window);

        // Search Elements
            // -- Button: focus
        let focus_image = SvgImage::load("svg/pointer.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "pointer" ("pointer.svg")"#);
        let mut foucus_on_search_btn = Button::default();
        SearchAction::set_static_styles_for_buttons(&mut foucus_on_search_btn, ElementType::Button, TransferredStyleData { icon: Some(focus_image), color: Some(BUTTON_COLOR), label: Some("Start Typing") });

            // -- Input: Pass links here
        let label_txt: &'static str = "Put URL/URLs";
        let mut search_input = Input::default();
        search_input.set_value(label_txt);
        search_input.set_text_color(Color::White);
        SearchAction::set_static_styles_for_buttons(&mut search_input, ElementType::Input, TransferredStyleData { icon: None, color: Some(INPUT_COLOR), label: None });

            // -- Button: Add Link
        let add_link_image: SvgImage = SvgImage::load("svg/add-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "add-icon" ("add-icon.svg")"#);
        let mut add_link_to_link_list_btn = Button::default(); 
        SearchAction::set_static_styles_for_buttons(&mut add_link_to_link_list_btn, ElementType::Button, TransferredStyleData { icon: Some(add_link_image), color: Some(BUTTON_COLOR), label: Some("Add Link") });

            // -- Button: Scrap Words
        let start_scrap_words_image = SvgImage::load("svg/send-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "send-icon" ("send-icon.svg")"#);
        let mut start_scrap_words_btn = Button::default();
        SearchAction::set_static_styles_for_buttons(&mut start_scrap_words_btn, ElementType::Button, TransferredStyleData { icon: Some(start_scrap_words_image), color: Some(BUTTON_COLOR), label: Some("Scrap Words") });

        // Set size of all elements in flex
        fl_container.set_size(&mut foucus_on_search_btn, 100); // Button: focus
        fl_container.set_size(&mut search_input, 350); // Input: Pass links here
        fl_container.set_size(&mut add_link_to_link_list_btn, 100); // -- Button: Add Link
        fl_container.set_size(&mut start_scrap_words_btn, 100); // -- Button: Scrap Words

        // Handle Events Section
        let mut foucus_on_search_btn_listener: Listener<_> = foucus_on_search_btn.into(); // Button: focus
        let mut search_input_listener: Listener<_> = search_input.clone().into(); // Input: Pass links here
        let mut add_link_to_list_listener: Listener<_> = add_link_to_link_list_btn.into(); // -- Button: Add Link
        let mut scrap_words_btn_listener: Listener<_> = start_scrap_words_btn.into(); // -- Button: Scrap Words
        
        let search_input_interaction_action = { // action which removes placeholder from Input
            move |r#in: &mut Input| {
                if r#in.value().trim().len() == 0 || r#in.value() == label_txt.to_string() {
                    r#in.set_value("");                    
                    r#in.take_focus().unwrap();
                };
            }
        };
            // -- Input: Add Urls listener for events
        search_input_listener.on_click(search_input_interaction_action.clone()); // When user click on Input element
        search_input_listener.on_unfocus(|r#in: &mut Input| { // when user click on other window using mouse cursor 
            if r#in.value().trim().len() == 0
            {
                r#in.set_value(label_txt);
            };
        });

            // -- Button: Start Typing listeners
        foucus_on_search_btn_listener.on_click({ // When user click on "Start Typing" button
            let mut search_input = search_input.clone();
            move |btn| {
                search_input_interaction_action(&mut search_input);
                btn.clear_visible_focus();
                search_input.take_focus().unwrap();
            }
        }); // TODO: Remove additional brackets when they aren't needed

            // -- Button: Add Links To List
        add_link_to_list_listener.on_click({ // add urls to url list // TODO: urls must be reall add to url list
            let search_input = search_input.clone();
            move |_btn| {
                if search_input.value().trim().len() > 0 && search_input.value() != label_txt.to_string()
                {
                    let b_values = search_input.value();
                    let values = b_values.split(" ").collect::<Vec<&str>>();
                    for url in values
                    {
                        if url.starts_with("https://") || url.starts_with("http://")
                        {
                            // TODO: In this place program add urls to list
                            println!("{}", url);
                        }
                        else 
                        { // TODO: alert system which inform user 

                        };
                    };
                };
            }
        });

            // -- Button: Start Scrap words from url to scrap list or input when scrap words list is empty
        scrap_words_btn_listener.on_click(move |_btn| {
            let mut search_vec = Vec::<&str>::new(); // vec which is sending to search function
            
            // Add value from input to search_vec
            if search_input.value().trim().len() > 0
            {
                let b_ = search_input.value();
                let search_input_vec =  b_.trim().split(" ").collect::<Vec<&str>>();
                for url in search_input_vec
                {
                    if url.starts_with("https://") || url.starts_with("http://")
                    {
                        search_vec.push(url);
                    };
                };
            };

            // TODO: (when list will be created) Add URLs from list to search_vec
            // TODO: Search words function which is outside function
        });

        // Load changes to flex box 
        fl_container.end();
    }

    fn set_static_styles_for_buttons<Elem: WidgetExt>(elem: &mut Elem, elem_type: ElementType, additional_elements: TransferredStyleData) // this function simplifing set styles for elements which implements WifgetExt trait and this action should be call from set static types for elements 
    {
        // Default Style setting
        if let Some(v) = additional_elements.color {
            elem.set_color(Color::from_rgb(v.0, v.1, v.2));
        }
        elem.set_frame(widget_themes::OS_SPACER_THIN_DOWN_BOX);
        // Set specific style for element based on elem_type parameter which represent the real type of elem
        match elem_type
        {
            ElementType::Button => {
                elem.set_label(additional_elements.label.unwrap());
                elem.set_label_color(Color::White);
                elem.set_image(additional_elements.icon);
                elem.clear_visible_focus();
            },
            ElementType::Input => {
                elem.set_selection_color(Color::Black);
            },
            _ => ()
        };
    }
}

pub fn create() {
    let app_ = fltk::app::App::default();
    let mut wn_ = fltk::window::Window::new(0, 0, 900, 900, "Daisy Scraper");

    wn_.set_color(Color::from_rgb(2, 7, 46));
    top_buttons(&mut wn_); // create top buttons
    SearchAction::create(&mut wn_); // create search GUI

    wn_.end();
    wn_.show();
    app_.run().unwrap();
}

fn top_buttons(window: &mut Window) {
    // Button size tuple type
    let btn_size = (150, 55); // x, y
    // let button_color = Color::rgb_color(48, 55, 110).to_hex_str().as_str();

    // Container for buttons
    let mut f_con = Flex::default()
        .with_size(btn_size.0 * 2 + 50, btn_size.1)
        .row();
    let f_center_parent_x = f_con.clone().center_x(&*window).x() + (120 / 2);
    f_con.set_pos(f_center_parent_x, 50);
    f_con.set_type(FlexType::Row);

    // Add buttons to container
    let mut search_btn = Button::default().center_of(&mut f_con).with_label("C");
    let search_btn_img = SvgImage::load("svg/search.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "search" ("search.svg")"#);
    search_btn.set_label("Search");
    search_btn.set_label_color(Color::White);
    search_btn.set_frame(widget_themes::OS_SPACER_THIN_DOWN_BOX);
    search_btn.set_color(Color::from_rgb(BUTTON_COLOR.0, BUTTON_COLOR.1, BUTTON_COLOR.2));
    search_btn.clear_visible_focus();

    let mut menu_btn = Button::default().center_of(&mut f_con);
    let menu_btn_img = SvgImage::load("svg/menu.svg").expect(r#"Cound't load menu icon from folder ./svg. Add svg file which is svg file and his name is "menu" ("menu.svg")"#);
    menu_btn.set_label("Menu");
    menu_btn.set_label_color(Color::White);
    menu_btn.set_frame(widget_themes::OS_SPACER_THIN_DOWN_BOX);
    menu_btn.set_color(Color::from_rgb(BUTTON_COLOR.0, BUTTON_COLOR.1, BUTTON_COLOR.2));
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

    menu_btn_listener.on_leave(|btn| {
            fltk::draw::set_cursor(Cursor::Default);
            btn.set_color(Color::from_rgb(BUTTON_COLOR.0, BUTTON_COLOR.1, BUTTON_COLOR.2));
        
        }
    );

    search_btn_listener.on_hover(|btn| {
        fltk::draw::set_cursor(Cursor::Hand);
        btn.set_color(btn.color().darker());
    });

    search_btn_listener.on_leave(|btn| {
            fltk::draw::set_cursor(Cursor::Default);
            btn.set_color(Color::from_rgb(BUTTON_COLOR.0, BUTTON_COLOR.1, BUTTON_COLOR.2));
        }
    );

    // Add changes from code
    f_con.end();
}