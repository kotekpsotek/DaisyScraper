use core::panic;

use crate::config::default::Setting;

use super::config::default as config;
#[allow(unused_imports)]
use fltk::{
    self,
    button::Button,
    enums::{Align, Color, Cursor, Font, FrameType},
    frame::Frame,
    image::SvgImage,
    input::Input,
    prelude::*,
    window::Window,
    draw,
    tree::{ Tree, TreeItem }
}; // GUI Library: Fast Light ToolKit
use fltk_evented::Listener;
use fltk_flex::Flex;
use fltk_theme::*;
use regex::Regex;

#[allow(dead_code)]
enum ElementType {
    Frame,
    Button,
    Input,
}

#[allow(dead_code)]
enum ActionType {
    Create,
    Read,
    Update(&'static str),
    Delete
}

struct TransferredStyleData {
    icon: Option<SvgImage>,
    color: Option<(u8, u8, u8)>,
    label: Option<&'static str>,
}

#[derive(Debug)]
struct ContainerForLinks { // Struct for Scroll container which is add here in LoadElement::create_search_frame method
    src: fltk::tree::Tree
}

struct LoadElement;
impl LoadElement {
    pub fn create(window: &mut Window, set: &config::Setting) {
        // from outside you should invoke only this function no any other function (this is simplier way to invoke the function)
        /* Order of creating elements and adding it to the guiL:
            1. Top Bar has been created,
            2. Frame has been created (this is for because element from frame should be returned to section where links are added to allow add links to the frame)
        */
        
        // Create Top Bar
        Self::create_top_bar(&mut *window, &*set);
        // Create Frame
        let container_for_links: ContainerForLinks = Self::crud_search_frame(&mut *window, ActionType::Create, &*set);
        let links_container: Tree = container_for_links.src;
        // Create Search Bar
        Self::create_search_bar(&mut *window, &*set, links_container);
    }

    // Create Top Bar
    fn create_top_bar(window: &mut Window, set: &config::Setting) {
        // Button size tuple type
        let btn_size = 150; // x, y

        // Container for buttons
        let mut f_con = Flex::default().with_size(btn_size * 2 + 50, 55).row();
        f_con.set_pos(f_con.clone().center_x(&*window).x() + (120 / 2), 50);

        // Add buttons to container
        let btn_color = set.btn_element_background_color.clone().to_rgb();
        // -- Button: Search
        let mut search_btn = Button::default().center_of(&mut f_con).with_label("C");
        Self::set_static_styles_for_buttons(search_btn.clone(), ElementType::Button, TransferredStyleData { icon:  Some(SvgImage::load("svg/search.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "search" ("search.svg")"#)), color: Some(btn_color), label: Some("Search") });

        // -- Button: Menu
        let mut menu_btn = Button::default().center_of(&mut f_con);
        Self::set_static_styles_for_buttons(menu_btn.clone(), ElementType::Button, TransferredStyleData { icon: Some(SvgImage::load("svg/menu.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "menu" ("menu.svg")"#)), color: Some(btn_color), label: Some("Menu") });

        // Size of buttons
        f_con.set_size(&mut search_btn, btn_size); // width: of the button
        f_con.set_size(&mut menu_btn, btn_size); // width: of the button

        // Add changes from code
        f_con.end();
    }

    // Create Search Bar placed on top
    fn create_search_bar(window: &mut Window, set: &config::Setting, mut links_list: Tree) {
        // Container for Bar Elements
        let mut fl_container = Flex::default()
            .with_size(650, 55)
            .with_pos(100, 120) // 15 point space between other containers in y axis
            .row()
            .center_x(&*window);

        // Search Elements
        // -- Button: focus
        let mut foucus_on_search_btn = Button::default();

        // -- Input: Pass links here
        let label_txt: &'static str = "Put URL/URLs";
        let mut search_input = Input::default();
        search_input.set_value(label_txt);
        search_input.set_text_color(set.element_font_color);
        search_input.set_text_font(Font::Courier);
        search_input.set_text_size(15);
        LoadElement::set_static_styles_for_buttons(
            search_input.clone(),
            ElementType::Input,
            TransferredStyleData {
                icon: None,
                color: Some(set.inp_element_background_color.to_rgb()),
                label: None,
            },
        ); // This function must be here if you would like to input element events works correctly

        // -- Button: Add Link
        let add_link_image: SvgImage = SvgImage::load("svg/add-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "add-icon" ("add-icon.svg")"#);
        let mut add_link_to_link_list_btn = Button::default();

        // -- Button: Scrap Words
        let start_scrap_words_image = SvgImage::load("svg/send-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "send-icon" ("send-icon.svg")"#);
        let mut start_scrap_words_btn = Button::default();

        // Handle Events Section
        let mut foucus_on_search_btn_listener: Listener<_> = foucus_on_search_btn.clone().into(); // Button: focus
        let mut search_input_listener: Listener<_> = search_input.clone().into(); // Input: Pass links here
        let mut add_link_to_list_listener: Listener<_> = add_link_to_link_list_btn.clone().into(); // -- Button: Add Link
        let mut scrap_words_btn_listener: Listener<_> = start_scrap_words_btn.clone().into(); // -- Button: Scrap Words

        let search_input_interaction_action = move |r#in: &mut Input| {
            // action which removes placeholder from Input
            if r#in.value().trim().len() == 0 || r#in.value() == label_txt.to_string() {
                r#in.set_value("");
                r#in.take_focus().unwrap();
            };
        };
        // -- Input: Add Urls listener for events
        search_input_listener.on_click(search_input_interaction_action.clone()); // When user click on Input element
        search_input_listener.on_unfocus(|r#in: &mut Input| {
            // when user click on other window using mouse cursor
            if r#in.value().trim().len() == 0 {
                r#in.set_value(label_txt);
            };
        });

        // -- Button: Start Typing listeners
        foucus_on_search_btn_listener.on_click({
            // When user click on "Start Typing" button
            let mut search_input = search_input.clone();
            move |btn| {
                println!("Test 123");
                search_input_interaction_action(&mut search_input); //
                btn.clear_visible_focus();
                search_input.take_focus().unwrap();
            }
        });

        // -- Button: Add Links To List
        add_link_to_list_listener.on_click({
            // add urls to url list // TODO: urls must be reall add to url list
            let mut search_input = search_input.clone();
            let set = crate::config::default::Setting::app_default(); // TODO: this is only temporary solution so i must replace that or all sharing settin between functions patern

            move |_btn| {
                if search_input.value().trim().len() > 0
                    && search_input.value() != label_txt.to_string()
                {
                    let b_values = search_input.value().clone();
                    let values = b_values.split(" ").collect::<Vec<&str>>();
                    for url in values { // Add Links to the Container
                        if url.starts_with("https://") || url.starts_with("http://") {
                            // When list is closed list becomes open now
                            if links_list.is_close("Links List") {
                                if links_list.open("Links List", true).is_err() {
                                    ()
                                }
                            };
                            
                            let url = url.replace("//", &"\\".repeat(4)); // change url for stop create new sub-lists
                            let mut item = links_list.root().unwrap().tree().unwrap().add(&url).unwrap();
                            item.set_label_color(set.element_font_color); // Dont't remove set variable if you would like to application work
                            item.set_label_font(Font::Courier);
                            links_list.redraw();
                        } else { // TODO: alert system which inform user
                        };
                    };
                    // This must work better search_input.set_value(""); // after we add to the links container all links input element should be clean
                };
            }
        });

        // -- Button: Start Scrap words from url to scrap list or input when scrap words list is empty
        scrap_words_btn_listener.on_click({
            let search_input = search_input.clone();
            move |_btn| {
                let mut search_vec = Vec::<&str>::new(); // vec which is sending to search function

                // Add value from input to search_vec
                if search_input.value().trim().len() > 0 {
                    let b_ = search_input.value();
                    let search_input_vec = b_.trim().split(" ").collect::<Vec<&str>>();
                    for url in search_input_vec {
                        if url.starts_with("https://") || url.starts_with("http://") {
                            search_vec.push(url);
                        };
                    }
                };

                // TODO: (when list will be created) Add URLs from list to search_vec
                // TODO: Search words function which is outside function
            }
        });

        // Add styles and defaults behaviours for buttons
        // -- Button: Focus on Search
        LoadElement::set_static_styles_for_buttons(foucus_on_search_btn.clone(), ElementType::Button, TransferredStyleData { icon: Some(SvgImage::load("svg/pointer.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "pointer" ("pointer.svg")"#)), color: Some(set.btn_element_background_color.to_rgb()), label: Some("Start Typing") });
        // -- Button: Add Link
        LoadElement::set_static_styles_for_buttons(
            add_link_to_link_list_btn.clone(),
            ElementType::Button,
            TransferredStyleData {
                icon: Some(add_link_image),
                color: Some(set.btn_element_background_color.to_rgb()),
                label: Some("Add Link"),
            },
        );
        // Button: Start Scrap Words
        LoadElement::set_static_styles_for_buttons(
            start_scrap_words_btn.clone(),
            ElementType::Button,
            TransferredStyleData {
                icon: Some(start_scrap_words_image),
                color: Some(set.btn_element_background_color.to_rgb()),
                label: Some("Scrap Words"),
            },
        );

        // Set size of all elements in flex
        fl_container.set_size(&mut foucus_on_search_btn, 100); // Button: focus
        fl_container.set_size(&mut search_input, 350); // Input: Pass links here
        fl_container.set_size(&mut add_link_to_link_list_btn, 100); // -- Button: Add Link
        fl_container.set_size(&mut start_scrap_words_btn, 100); // -- Button: Scrap Words

        // Load changes to flex box
        fl_container.end();
    }

    // Create/Read/Update Search Frame -> this is a frame with searching url adresses
    fn crud_search_frame(window: &mut Window, ac: ActionType, set: &config::Setting) -> ContainerForLinks
    {
        let container_for_links: ContainerForLinks;
        // Create 2 containers: 1 - top bar for buttons, 2 - scroll element for other elements which can be added to him
        let flex_container_width_height = 650;
        let window_w = window.width() - flex_container_width_height;
        let mut flex_column = Flex::default()
            .with_size(flex_container_width_height + 15, flex_container_width_height)
            .with_pos(window_w/2, 190)
            .column();
        flex_column.set_pad(0);
        
        // -- TopBar container
        let mut buttons_bar_main = Flex::default()
            .row();
        buttons_bar_main.set_frame(FrameType::BorderBox);
        buttons_bar_main.set_color(set.btn_element_background_color);
        buttons_bar_main.end();
        
        // -- ScrollElements container // Scroll Element only
        let mut tree = fltk::tree::Tree::new(0, 0, flex_container_width_height, flex_container_width_height - 50, "");
        tree.set_root_label("Links List");
        tree.set_frame(FrameType::BorderBox);
        tree.set_color(set.fr_element_background_color);
        let mut t_root = tree.root().unwrap();
        t_root.set_label_font(Font::Courier);
        t_root.set_label_color(set.element_font_color);
        t_root.set_label_size(18);
        
        // -- Set Size for this two containers
        flex_column.set_size(&mut buttons_bar_main, 50); // height of buttons bar
        // flex_column.set_size(&mut elements_scroll, flex_container_width_height - 50); // height of scroll elemenets container
        
        // Stop Load changes to elements
        flex_column.end();
        
        // Create Elements for TopBar
        // -- Select All button
        let mut select_all_button = Button::default()
        .with_label("Select All")
        .with_size(75, 50)
        .with_pos(window_w / 2, 190);
        select_all_button.clear_visible_focus();
        select_all_button.set_color(set.fr_elements_top_bar_background_color);
        select_all_button.set_label_color(set.element_font_color);
        select_all_button.set_label_font(Font::Courier);
        select_all_button.set_frame(FrameType::BorderBox);
        
        // -- Count Info Element
        let mut count_info = Frame::default()
            .with_label("Elements Count: 0")
            .with_size(150, 50)
            .with_pos(650 - 10, 190);
        count_info.set_color(set.fr_elements_top_bar_background_color);
        count_info.set_label_color(set.element_font_color);
        count_info.set_label_font(Font::Courier);
        count_info.set_frame(FrameType::BorderBox);
        
        // -- Events Handle Section for TopBar button
        let mut select_all_button_list: Listener<_> = select_all_button.into();        
            
        select_all_button_list.on_click({ // when user click on select on button
            let mut tree = tree.clone();
            move |_b| {
                let items_list = tree.clone().get_items().unwrap();
                for item in items_list {
                    // When list is closed list becomes open now
                    if tree.is_close("Links List") {
                        tree.open("Links List", true).unwrap();
                    };

                    // Select all Elements and handle the selecting Result
                    let select_all_action = tree.select_all(&item, true);
                    match select_all_action { // TODO: Add error and success alert handling // in some kind of reason program return in this place a error
                        Err(_) => (),
                        Ok(_) => ()
                    };

                    // Unselect root wlement when it is selected
                    let mut tree_root_element = tree.root().unwrap();
                    if tree_root_element.is_selected() {
                        tree_root_element.deselect();
                    };
                }
            }
        });
        
        select_all_button_list.on_hover(|btn| {
            btn.set_color(btn.color().lighter());
            draw::set_cursor(Cursor::Hand);
        });
        
        select_all_button_list.on_leave({
            let def_color_button = set.fr_elements_top_bar_background_color;
            move |btn| {
                btn.set_color(def_color_button);
                draw::set_cursor(Cursor::Default);
            }
        });

        // TODO: CRUD on created elements 

        container_for_links = ContainerForLinks { src: tree };
        container_for_links
    }

    fn set_static_styles_for_buttons<Elem: WidgetExt + WidgetBase + std::default::Default + Clone + 'static,
    >(
        mut elem: Elem,
        elem_type: ElementType,
        additional_elements: TransferredStyleData,
    ) // this function simplifing set styles for elements which implements WifgetExt trait and this action should be call from set static types for elements
    {
        // Default Style setting
        if let Some(v) = additional_elements.color {
            elem.set_color(Color::from_rgb(v.0, v.1, v.2));
        }
        elem.set_frame(widget_themes::OS_SPACER_THIN_DOWN_BOX);
        // Set specific style for element based on elem_type parameter which represent the real type of elem
        match elem_type {
            ElementType::Button => {
                elem.set_label(additional_elements.label.unwrap());
                elem.set_label_color(Color::White);
                elem.set_image(additional_elements.icon);
                elem.clear_visible_focus();
                elem.set_label_font(Font::Courier);
            }
            ElementType::Input => {
                elem.set_selection_color(Color::Black);
            }
            _ => (),
        };
    }
}

pub fn create() {
    let app_ = fltk::app::App::default();
    let mut wn_ = fltk::window::Window::new(0, 0, 900, 900, "Daisy Scraper");
    let settings = config::Setting::app_default();

    // Set fonts for elements
    Font::set_font(Font::Courier, &settings.font); // replace Font::Courier by custon font

    // Set bacground Color of this application window
    wn_.set_color(settings.app_backround_color.clone());

    // Create Elements
    LoadElement::create(&mut wn_, &settings); // create search GUI

    wn_.end();
    wn_.show();
    app_.run().unwrap();
}
