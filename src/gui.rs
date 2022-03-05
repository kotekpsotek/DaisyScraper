use crate::{ scrap::scrap_from, config::default::Setting};

use super::config::default as config;
use fltk::text::TextBuffer;
#[allow(unused_imports)]
use fltk::{
    self,
    button::{ CheckButton, Button},
    enums::{Align, Color, Cursor, Font, FrameType, Event, Key },
    frame::Frame,
    image::SvgImage,
    input::Input,
    prelude::*,
    window::{ Window, DoubleWindow },
    draw,
    tree::{ Tree, TreeItem },
    dialog,
    misc::Progress,
    group
}; // GUI Library: Fast Light ToolKit
use fltk_evented::Listener;
use fltk_flex::Flex;
#[allow(unused_imports)]
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
    Update,
    Delete
}

struct TransferredStyleData {
    icon: Option<SvgImage>,
    color: Option<(u8, u8, u8)>,
    label: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct ContainerForLinks { // Struct for Scroll container which is add here in LoadElement::create_search_frame method
    pub src: fltk::tree::Tree,
    elements_in_count: Frame
}

impl ContainerForLinks { // Update Frame Container VIA elements maniupulation
    fn update_list(&mut self, added_element: Vec<&str>, settings: &Setting, _window: DoubleWindow, input: &mut Input) { 
        let screen_size = fltk::app::screen_size();
        for url in added_element { // Add Links to the Container
            if url.starts_with("https://") || url.starts_with("http://") {                
                // When list is closed list becomes open now
                if self.src.is_close("Links List") {
                    if self.src.open("Links List", true).is_err() { // When list coudn't be opened from some reason the program return empty tuple type to prevent before program crash 
                        ()
                    }
                };
            
                // Update the links list
                let url = url.replace("//", &"\\".repeat(4)); // change url for stop create new sub-lists
                let item = self.src.root().unwrap().tree().unwrap().add(&url);
                
                // Add Element to the container only when the element didn't already exists
                if let Some(mut item) = item {
                    item.set_label_color(settings.element_font_color); // Dont't remove set variable if you would like to application work
                    item.set_label_font(Font::Courier);
                    self.src.redraw(); // load visible changes for the user to the container with links
                    
                    // Update displayed elemtnts count
                    Self::update_elements_count(&mut self.elements_in_count, ActionType::Update, 1);

                    // Clear Input with added links after succesfull add link to the container
                    input.set_value("");
                }
                else {
                    dialog::alert((screen_size.0 as i32 - 900) / 2, 10, &format!("This URL: \"{}\" which is added to input field is already in the links list. The values which are in the list cound't be repeated multiple times. Links List allow only unique elements!!!", url));
                };
            } else { // When link starts without http:// | https:// 
                dialog::alert((screen_size.0 as i32 - 900) / 2, 10, &format!("The adding urls to the search field should starts with protocols http:// or https://. Your link \"{}\" should to begin with https:// or http:// protocol!!!", url));
            };
        };
    }

    // Update the elements in container count
    fn update_elements_count(label: &mut Frame, at: ActionType, number: i32) {
        // Get actual number elements in container
        let actual_count_v = label.label();
        let regex_number_f = Regex::new(r"[0-9]").unwrap();
        let search_results_r = regex_number_f.find(actual_count_v.as_str()).unwrap().as_str();
        let search_results_to_number = search_results_r.parse::<i32>().unwrap();

        // Set greater or less number of the elements to the frame
        let new_count = if let ActionType::Update = at {
            search_results_to_number + number
        }
        else if let ActionType::Delete = at {
            let result_of_adding = search_results_to_number - number;
            // If value starts with - then value is 0 not minus value
            if result_of_adding.to_string().starts_with("-") {
                0
            }
            else {
                result_of_adding
            }
        }
        else {
            0
        };

        let label_text = format!("Elements Count: {}", new_count);
        label.set_label(&label_text);
    }

    // return the value from elements which are in links list
    fn links_container_get_values(&self) -> Result<Vec<String>, &'static str> {
        if let Some(vals) = self.src.get_items() {
            let mut returned_vec = Vec::<String>::new();
            for item in vals {
                if let Some(val) = item.label() {
                    if val != self.src.root().unwrap().label().unwrap() { // when item value isn't the same value as root label value
                        let val = val.replace("\\", "/");
                        returned_vec.push(val);
                    }
                };
                // in onther hand when value coudn't be getted nothing has been doed and loop go to next iteration
            };

            if returned_vec.len() > 0 {
                Ok(returned_vec)
            }
            else {
                Err("the vector which should be returned is empty (doesn't have any values because program coudn't get values)")
            }
        }
        else {
            Err("links container hasn't got any links inside")
        }
    }
} 

pub enum CreateElementCategoryType { // The type of window which should be created should because if you use "Deafult" element then Search window is creating by default
    Default,
    Search,
    Menu
}

struct ElementsCategories;
impl ElementsCategories {
    fn create_search_words_from_links_elements(window: &mut Window, set: &config::Setting) { // create elements for searching links from webpages
        // Create Top Bar
        LoadElement::create_top_bar(&mut *window, &*set); // this must be the last because this bar have the functions for remove elements
        // Create Frame
        let container_for_links: ContainerForLinks = LoadElement::create_search_frame(&mut *window, &*set);
        // Create Search Bar
        LoadElement::create_search_bar(&mut *window, &*set, container_for_links.clone());
    }

    fn create_menu_elements(window: &mut Window, set: &config::Setting) { // TODO: Create Menu Elements for read saved data
        // Create Top Bar
        LoadElement::create_top_bar(&mut *window, &*set); // this must be the last because this bar have the functions for remove elements
        // Create Search Bar
        LoadElement::create_search_frame_menu(&mut *window, &*set);
        // Create Title
        LoadElement::create_title_menu(&mut *window, &*set);
        // Create Frame and add elements to the frame
        LoadElement::create_scrollframe_with_lists_menu(&mut *window, &*set);
    }
}

struct LoadElement;
impl LoadElement {
    pub fn create(window: &mut Window, set: &config::Setting, r#type: CreateElementCategoryType) {
        // from outside you should invoke only this function no any other function (this is simplier way to invoke the function)
        /* Order of creating elements and adding it to the guiL:
            1. Top Bar has been created,
            2. Frame has been created (this is for because element from frame should be returned to section where links are added to allow add links to the frame)
        */
        match r#type { // Open correct window type
            CreateElementCategoryType::Default => { // open last opened window via user. It is using when program is starting displaing GUI
                let last_opened_window = crate::states::which_widnow_was_opened_recently();
                match last_opened_window {
                    Ok(last_opened_window) => {
                        match last_opened_window {
                            crate::states::OpenedWindow::Menu => {
                                crate::states::save_window_state(crate::states::OpenedWindow::Menu).unwrap(); // save/update APP STATE
                                ElementsCategories::create_menu_elements(window, set);
                            },
                            crate::states::OpenedWindow::Search => {
                                crate::states::save_window_state(crate::states::OpenedWindow::Search).unwrap(); // save/update APP STATE
                                ElementsCategories::create_search_words_from_links_elements(window, set);
                            }
                        }
                    },
                    Err(_) => { // When is error program creates search words from links window
                        crate::states::save_window_state(crate::states::OpenedWindow::Search).unwrap(); // save/update APP STATE
                        ElementsCategories::create_search_words_from_links_elements(window, set);
                    }
                }
            },
            CreateElementCategoryType::Search => { // create Search Window
                crate::states::save_window_state(crate::states::OpenedWindow::Search).unwrap(); // save/update APP STATE
                ElementsCategories::create_search_words_from_links_elements(window, set);
            },
            CreateElementCategoryType::Menu => { // create menu GUI elements
                crate::states::save_window_state(crate::states::OpenedWindow::Menu).unwrap(); // save/update APP STATE
                ElementsCategories::create_menu_elements(window, set);
            }
            
        }
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

        // Event Handling section
        let mut search_btn_listen: Listener<_> = search_btn.into();
        let mut menu_btn_listen: Listener<_> = menu_btn.into();
        let both_hover = |btn: &mut Button| {
            btn.set_color(btn.color().lighter());
            draw::set_cursor(Cursor::Hand);
        };
        let both_leave = {
            let def_color_button = set.fr_element_background_color;
            move |btn: &mut Button| {
                btn.set_color(def_color_button);
                draw::set_cursor(Cursor::Default);
            }
        };

        // -- Button: Search
        search_btn_listen.on_hover(both_hover);
        search_btn_listen.on_leave(both_leave);

        // -- Button: Menu
        menu_btn_listen.on_hover(both_hover);
        menu_btn_listen.on_leave(both_leave);

        // When user Click on "Search" button
        search_btn_listen.on_click({
            let mut window = window.clone();
            move |_| {
                window.hide(); // Remove now opened window
                crate::gui::create(CreateElementCategoryType::Search); // Create New window
            }
        });

        // When user Click on "Menu" button
        menu_btn_listen.on_click({
            let mut window = window.clone();
            move |_| {
                window.hide(); // Remove now opened window
                crate::gui::create(CreateElementCategoryType::Menu); // Create New window
            }
        });

        // Add changes from code
        f_con.end();
    }

    // Search: Create Search Bar placed on top
    fn create_search_bar(window: &mut Window, set: &config::Setting, links_list: ContainerForLinks) -> Vec<Flex> {
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
            let window = window.clone();
            let mut links_list = links_list.clone();

            move |_btn| {
                if search_input.value().trim().len() > 0
                    && search_input.value() != label_txt.to_string()
                {
                    let b_values = search_input.value().clone();
                    let values = b_values.trim().split(" ").collect::<Vec<&str>>();
                    links_list.update_list(values, &set, window.clone(), &mut search_input); // links list: Add new elements to the links 
                };
            }
        });
        
        // !!! Function which initialize download words from GUI
        fn show_window_and_scrap_words(links_list: &ContainerForLinks, search_input: &Input) {
            let mut links_list = links_list.clone();
            let (screen_width, screen_height) = fltk::app::screen_size();
            let search_input = search_input.clone();

            // Links from Input Container
            let mut base_he = if search_input.value().trim().len() > 0 {
                // Process the value
                let res = Vec::from_iter(search_input.value().trim().split(" ").collect::<Vec<&str>>().iter().map(|val| {
                    let val = val.trim();
                    if val.starts_with("https://") || val.starts_with("http://") {
                        val.to_string()
                    }
                    else {
                        String::new()
                    }
                }));
                
                // Return value
                if res.join(" ").trim().len() > 0 {
                    res
                }
                else {
                    Vec::<String>::new()
                }
            }
            else {
                Vec::<String>::new()
            };
            // Links from Container for other links
            let mut urls_count = if let Ok(vec) = links_list.links_container_get_values() {
                vec
            }
            else {
                Vec::<String>::new()
            };
            urls_count.append(&mut base_he);
            
            // When URLs have been added
            if urls_count.len() > 0 {
                let mut wn = DoubleWindow::new(0, 0, 700, 250, "Scrap words progress");
                wn.set_pos((screen_width as i32 - 700) / 2, (screen_height as i32 - 250) / 2);

                tokio::spawn({
                    let data = LoadElement::create_progress_frame(wn.clone()); // create scrap words progress window 
                    async move { // start scrap words (this must be in tokio block because scrap words is async function)
                        scrap_words(&mut links_list, &search_input, data).await;
                    }
                });
                wn.end();
                wn.show();
            };
        }

        scrap_words_btn_listener.on_click({ // When user click on button "Scrap Words"
            let links_list = links_list.clone();
            let search_input = search_input.clone();
            move |_| {
                show_window_and_scrap_words(&links_list, &search_input); // initialize download words from webpages
            }
        });

        // -- Button: Start Scrap words from url to scrap list or input when scrap words list is empty
        async fn scrap_words(link_list: &mut ContainerForLinks, search_input: &Input, gui_params: (Progress, Frame, Frame, DoubleWindow)) { // starts scrap words from pages based on added links
            let mut search_vec = Vec::<String>::new(); // vec which is sending to search function

            // Add value from input to search_vec
            if search_input.value().trim().len() > 0 {
                let b_ = search_input.value();
                let search_input_vec = b_.trim().split(" ").collect::<Vec<&str>>();
                for url in search_input_vec {
                    if url.starts_with("https://") || url.starts_with("http://") {
                        search_vec.push(url.to_string());
                    };
                }
            };

            // Add values from links list to vec with links list
            if let Ok(links_values) = link_list.links_container_get_values() {
                for url in links_values {
                    search_vec.push(url)
                }
            };

            // Scrap words and show scrap progress bar
            scrap_from(search_vec, Some(gui_params.clone()), Some((link_list.clone(), search_input.clone()))).await;
        }

        // -- Keyboard events
        window.handle({
            let mut links_list = links_list.clone();
            let mut search_input = search_input.clone();
            let mut last_crl_pressed: bool = false;
            let window = window.clone();            

            move |_wn, ev| {
                let key = fltk::app::event_key();
                let text = fltk::app::event_text();

                if let Event::KeyUp = ev { // when user release the button 
                    if let Key::Enter = key { // When user click enter key the words will be download from web-pages
                        show_window_and_scrap_words(&links_list, &search_input); // initialize download words from webpages
                    };
                    true
                }
                else if let Event::KeyDown = ev { // when user click button only
                    if let Key::ControlL = key { // when user click LControl he have got autorizxation for using ctrl + other_key actions
                        last_crl_pressed = true
                    } 
                    else if last_crl_pressed {
                        match text.as_str() { // Do specific actions for specific clicked keys 
                            "a" => links_list.update_list(search_input.value().split(" ").collect::<Vec<&str>>(), &Setting::app_default(), window.clone(), &mut search_input), // links list: Add new elements to the links  // add putted in input element links to the links container
                            _ => ()
                        };
                        last_crl_pressed = false
                    };
                    true
                }
                else {
                    false
                }
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

        // Style Events
        let both_hover = |btn: &mut Button| {
            btn.set_color(btn.color().lighter());
            draw::set_cursor(Cursor::Hand);
        };
        let both_leave = {
            let def_color_button = set.fr_element_background_color;
            move |btn: &mut Button| {
                btn.set_color(def_color_button);
                draw::set_cursor(Cursor::Default);
            }
        };
        scrap_words_btn_listener.on_hover(both_hover);
        scrap_words_btn_listener.on_leave(both_leave);
        foucus_on_search_btn_listener.on_hover(both_hover);
        foucus_on_search_btn_listener.on_leave(both_leave);
        add_link_to_list_listener.on_hover(both_hover);
        add_link_to_list_listener.on_leave(both_leave);

        // Set size of all elements in flex
        fl_container.set_size(&mut foucus_on_search_btn, 100); // Button: focus
        fl_container.set_size(&mut search_input, 350); // Input: Pass links here
        fl_container.set_size(&mut add_link_to_link_list_btn, 100); // -- Button: Add Link
        fl_container.set_size(&mut start_scrap_words_btn, 100); // -- Button: Scrap Words

        // Load changes to flex box
        fl_container.end();

        // Return Values From Function
        return vec![fl_container];
    }

    // Create/Read/Update Search Frame -> this is a frame with searching url adresses
    fn create_search_frame(window: &mut Window, set: &config::Setting) -> ContainerForLinks
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

        // -- Delete Elements Button
        let delete_button_img = SvgImage::load("svg/trash-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "trash-icon" ("trash-icon.svg")"#);
        let mut delete_buttton =  Button::default()
            .with_size(50, 50)
            .with_pos(window_w / 2 + 75, 190); // position button on window start
        delete_buttton.clear_visible_focus();
        delete_buttton.set_color(set.fr_elements_top_bar_background_color);
        delete_buttton.set_frame(FrameType::BorderBox);
        delete_buttton.set_image(Some(delete_button_img));
        
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
        let mut delete_buttton_list: Listener<_> = delete_buttton.into();
            
        select_all_button_list.on_click({ // when user click on select on button
            let mut tree = tree.clone();
            move |_b| {
                let items_list = tree.clone().get_items().unwrap();
                // When list is closed list becomes open now
                if tree.is_close("Links List") {
                    tree.open("Links List", true).unwrap();
                };

                for item in items_list {
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
        
        delete_buttton_list.on_click({
            let mut tree = tree.clone();
            let _window = window.clone();
            let mut count_info = count_info.clone();
            move |_| {
                // -- Deselect root element for prevent in delete it accidentally
                let mut root_element = tree.root().unwrap();
                if root_element.is_selected() {
                    root_element.deselect();
                };

                // -- Remove Selected Items
                let selected_items = tree.get_selected_items();
                let screen_size = fltk::app::screen_size();
                match selected_items {
                    Some(items) => {
                        let selected_items_count = items.len();
                        let choice: i32 = dialog::choice((screen_size.0 as i32 - 900) / 2, 10, &format!("Are sure to delete selected elements from links list ({} elements)?", selected_items_count), "Yes", "No", ""); // value 1 = No, value 2 = Yes, value 3 = isn't presented
                        println!("{}", choice);
                        if choice == 0 { // only when user click on "Yes" button elements will be remove
                            let mut removed: bool = false; // Elements are removed? -> Status Yes or No
                            for item in &items {
                                let remove = tree.remove(&item);
                                match remove {
                                    Ok(_) => {
                                        removed = true;
                                        tree.redraw(); // load changes to the list
                                        ContainerForLinks::update_elements_count(&mut count_info, ActionType::Delete, items.len().to_string().parse::<i32>().unwrap()); // remove x count of elements from the count info
                                    },
                                    Err(err) => {
                                        let _alert = dialog::alert((screen_size.0 as i32 - 900) / 2, 10, &format!("Program coudn't delete selected elements from this reason: {}", err.to_string()));
                                    }
                                };
                            };
                            
                            // When elements has been succesfull removed from the container is displayed alert inform user about succesfull action
                            if removed {
                                dialog::message((screen_size.0 as i32 - 900) / 2, 10,&format!("Deleted {} links from the list!!!", selected_items_count)); // infor about deleted elements count
                            }
                        }
                    },
                    None => {
                        let _alert = dialog::alert((screen_size.0 as i32 - 900) / 2, 10, r#"No one element is selected. Click on element which you want delete or use "Select All" button to select all added elements!!!"#);
                    }
                }
            }
        });
        // Below: Listen events for the both buttons
        let both_hover = |btn: &mut Button| {
            btn.set_color(btn.color().lighter());
            draw::set_cursor(Cursor::Hand);
        };
        let both_leave = {
            let def_color_button = set.fr_elements_top_bar_background_color;
            move |btn: &mut Button| {
                btn.set_color(def_color_button);
                draw::set_cursor(Cursor::Default);
            }
        };
        select_all_button_list.on_hover(both_hover);
        select_all_button_list.on_leave(both_leave);

        delete_buttton_list.on_hover(both_hover);
        delete_buttton_list.on_leave(both_leave);

        container_for_links = ContainerForLinks { src: tree, elements_in_count: count_info };
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
        elem.set_frame(FrameType::BorderBox);
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

    // Menu: Create Search Bar
    fn create_search_frame_menu(window: &mut Window, set: &config::Setting) {
        let window_w = window.width();
        let x_localization = (window_w / 2) - (610 / 2) + 10;
        let mut search_frame = Flex::new(x_localization , 120, 600, 55, "")
            .row();

        // Button: Start Typing
        let mut foucus_on_search_btn = Button::default();
        LoadElement::set_static_styles_for_buttons(foucus_on_search_btn.clone(), ElementType::Button, TransferredStyleData { icon: Some(SvgImage::load("svg/pointer.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "pointer" ("pointer.svg")"#)), color: Some(set.btn_element_background_color.to_rgb()), label: Some("Start Typing") });

        // Input: Search Input
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
        );

        // Button: Search
        let mut start_search_words_btn = Button::default();
        LoadElement::set_static_styles_for_buttons(
            start_search_words_btn.clone(),
            ElementType::Button,
            TransferredStyleData {
                icon: Some(
                    SvgImage::load("svg/send-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "send-icon" ("send-icon.svg")"#)
                ),
                color: Some(set.btn_element_background_color.to_rgb()),
                label: Some("Search"),
            },
        );

        // Listen Events
        let mut start_typing_button: Listener<_> = foucus_on_search_btn.clone().into();
        let mut input: Listener<_> = search_input.clone().into();
        let mut search_button: Listener<_> = start_search_words_btn.clone().into();

        let search_input_interaction_action = move |r#in: &mut Input| {
            // action which removes placeholder from Input
            if r#in.value().trim().len() == 0 || r#in.value() == label_txt.to_string() {
                r#in.set_value("");
                r#in.take_focus().unwrap();
            };
        };

        // -- Input: Add Urls listener for events
        input.on_click(search_input_interaction_action.clone()); // When user click on Input element
        input.on_unfocus(|r#in: &mut Input| {
            // when user click on other window using mouse cursor
            if r#in.value().trim().len() == 0 {
                r#in.set_value(label_txt);
            };
        });

        // -- Button: Start Typing listeners
        start_typing_button.on_click({
            // When user click on "Start Typing" button
            let mut search_input = search_input.clone();
            move |btn| {
                search_input_interaction_action(&mut search_input); //
                btn.clear_visible_focus();
                search_input.take_focus().unwrap();
            }
        });

        // -- Button: Search
        search_button.on_click(|_| {}); // TODO: When use click lists must be search throught or serach list by his name

        // Style Events
        let both_hover = |btn: &mut Button| {
            btn.set_color(btn.color().lighter());
            draw::set_cursor(Cursor::Hand);
        };
        let both_leave = {
            let def_color_button = set.fr_element_background_color;
            move |btn: &mut Button| {
                btn.set_color(def_color_button);
                draw::set_cursor(Cursor::Default);
            }
        };
        start_typing_button.on_hover(both_hover);
        start_typing_button.on_leave(both_leave);
        search_button.on_hover(both_hover);
        search_button.on_leave(both_leave);
        

        search_frame.set_size(&mut foucus_on_search_btn, 100); // Button: Foucs on search input
        search_frame.set_size(&mut search_input, 400); // Input: Put words to scrap
        search_frame.set_size(&mut start_search_words_btn, 100); // Button: Search
        search_frame.end();
    }

    // Menu: Create Title
    fn create_title_menu(window: &mut Window, set: &config::Setting) {
        let w_w = window.width();
        // Title: Words List Container
        let mut container_title = Flex::new((w_w - ((300*2) - 10))/2, 190, 250, 55, "")
            .row();
        container_title.set_align(Align::Left | Align::Inside);
        container_title.set_frame(FrameType::BorderBox);
        container_title.set_color(set.btn_element_background_color);

        // Create Icon
        let icon_words = SvgImage::load("svg/list-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "list-icon" ("list-icon.svg")"#);
        let mut frame_icon_words = Frame::new(0, 0, 0, 55, "")
            .with_align(Align::Bottom | Align::Inside);
        frame_icon_words.set_image(Some(icon_words));

        // Create Text
        let mut text_description = Frame::new(0, 0, 0, 55, "")
            .with_label("Words Lists")
            .with_align(Align::Left | Align::Inside);
        text_description.set_label_font(Font::Courier);
        text_description.set_label_color(set.element_font_color);
        text_description.set_label_size(20);

        container_title.set_size(&mut frame_icon_words, 50);
        container_title.set_size(&mut text_description, 200);
        container_title.end();
    }

    // Menu: Create Frame container with others words
    fn create_scrollframe_with_lists_menu(window: &mut Window, set: &config::Setting) {
        let mut container_with_words_list = group::Scroll::new((window.width() - 610) / 2 + 10, 260, 610, 610, "");
        container_with_words_list.set_frame(FrameType::BorderBox);
        container_with_words_list.set_color(set.fr_element_background_color);
        container_with_words_list.set_type(group::ScrollType::VerticalAlways);
        container_with_words_list.set_scrollbar_size(10); // scroll bar width
        let mut pack = group::Pack::new(0, 0,610,610, "");
        pack.clone().center_of_parent(); // the all elements is now in the center of the parent
        pack.set_spacing(5); // the space between elements in single column

        // Add All words lists to the list container with styles and with all services related with specified list
        Self::add_lists_to_the_scrollframe_with_lists_menu(window, set, &mut pack);
        
        pack.end();
        pack.show();
        container_with_words_list.end();
        container_with_words_list.show();
    }

    // Menu: Add All Words list to the Lists Container
    fn add_lists_to_the_scrollframe_with_lists_menu(_window: &mut Window, set: &config::Setting, lists_container: &mut group::Pack) {
        let flags_names = crate::config::additional::Features::get_flags_data_from_words_files();
        let files_names = crate::config::additional::Features::get_files_names();
        match flags_names {
            Ok(mut flags) => {
                let flag_size = flags.get_elements_count();
                for flag_field in 0..flag_size { 
                    let flag = flags.get_element_from_index(flag_field);
                    let file_name = &files_names[flag_field];
                    if flag.name == "from" {
                        let protocol_from_flag = flag.value.0;
                        let domainname_from_flag = flag.value.1;
                        let url_path_from_flag = flag.value.2;
                        let port_from_flag = if let Some(wal) = flag.value.3 {
                            String::from(wal)
                        }
                        else {
                            String::new()
                        };
                        
                        // Get File Download form flag
                        let download_from_url_name = if port_from_flag.len() > 0 {
                            format!("{protocol}://{domain_name}:{port}/{url_path}", protocol = protocol_from_flag, domain_name = domainname_from_flag, port = port_from_flag, url_path = url_path_from_flag)
                        }
                        else {
                            format!("{protocol}://{domain_name}/{url_path}", protocol = protocol_from_flag, domain_name = domainname_from_flag, url_path = url_path_from_flag)
                        };

                        // Section: Create GUI List and add it to the parent
                        create_gui_list(file_name, download_from_url_name, set, lists_container);
                    }
                }
            },
            Err(_) => 
                ()
        }
        
        // Creating Window which show us all lists with words downloaded from pages before
        fn create_gui_list(file_name: &String, flag_from: String, set: &config::Setting, lists_container: &mut group::Pack) {
            let words_from_file = crate::config::additional::Features::get_words_in_file(file_name.clone()); // all words from file

            let mut single_list_master_con = Flex::new(0, 0, lists_container.width(), 50, "")
                .row();

            // Container with localization from where words have been downloaded
            let frame_with_list_name_width = (single_list_master_con.width() as f64 / 1.3) as i32;

            let mut frame_with_list_name = fltk::text::TextDisplay::new(0, 0, frame_with_list_name_width, single_list_master_con.height(), "");
            let mut text_buf = fltk::text::TextBuffer::default();
            text_buf.set_text(&flag_from);
            frame_with_list_name.set_frame(FrameType::BorderBox);
            frame_with_list_name.set_buffer(text_buf);
            frame_with_list_name.set_text_font(Font::Courier);
            frame_with_list_name.set_text_color(set.element_font_color);
            frame_with_list_name.set_text_size(16);
            frame_with_list_name.set_color(set.fr_elements_top_bar_background_color);
            frame_with_list_name.wrap_mode(fltk::text::WrapMode::AtBounds, 10);

            // Container: Buttons and Infos
            let buttons_infos_container_width = lists_container.width() - frame_with_list_name_width;
            let mut buttons_infos_container = Flex::new(0, 0, buttons_infos_container_width, lists_container.height(), "")
                .row();
            let child_elements_width = (buttons_infos_container.width() / 2) - 5;

            // -- Info: Words which are in list count
            let mut words_count_in_list = Frame::new(0, 0, child_elements_width, buttons_infos_container.height(), "")
                .with_align(Align::Center | Align::Inside);
            words_count_in_list.set_frame(FrameType::BorderBox);
            words_count_in_list.set_color(set.fr_elements_top_bar_background_color);
            words_count_in_list.set_label_color(set.element_font_color);
            words_count_in_list.set_label_font(Font::Courier);
            // -- > -- Add words count form the file to the GUI Element
            match &words_from_file {
                Some(val) => {
                    words_count_in_list.set_label(&val.len().to_string())
                },
                None => words_count_in_list.set_label("0")
            }

            // -- Button: Open List
            let button_open_list_image = SvgImage::load("svg/up-arrow-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "up-arrow-icon" ("up-arrow-icon.svg")"#);
            let mut button_open_list = Button::new(0, 0, child_elements_width, buttons_infos_container.height(), "")
                .with_align(Align::Bottom | Align::Inside);
            button_open_list.set_frame(FrameType::BorderBox);
            button_open_list.set_color(set.fr_elements_top_bar_background_color);
            button_open_list.set_label_color(set.element_font_color);
            button_open_list.set_image(Some(button_open_list_image));
            button_open_list.clear_visible_focus();

            // Hidden Container with file name
            let mut file_name_frame_container = Frame::new(0, 0, frame_with_list_name_width, single_list_master_con.height(), "")
                .with_label(file_name);
            file_name_frame_container.hide();
            
            buttons_infos_container.set_size(&mut words_count_in_list, child_elements_width); // Set Width for the element with the info about words count in the list
            buttons_infos_container.set_size(&mut button_open_list, child_elements_width); // Set Width for the button for the open lists with words
            buttons_infos_container.end();

            single_list_master_con.set_size(&mut frame_with_list_name, frame_with_list_name_width); // Set width for container with List Name
            single_list_master_con.set_size(&mut buttons_infos_container, buttons_infos_container_width); // Set width for container with Additional infos and buttons
            single_list_master_con.end();


            // Listen events Section
            let mut open_list_button: Listener<_> = button_open_list.clone().into();
            let mut frame_with_list_name_listener: Listener<_> = frame_with_list_name.clone().into();

            frame_with_list_name_listener.on_click({
                let words_from_file_cc = words_from_file.clone().unwrap();
                move |txt_elem| { // when user click on frame with list name
                    create_words_from_list_window(txt_elem.buffer().unwrap().text(), &words_from_file_cc, config::Setting::app_default());
                }
            });

            open_list_button.on_click({ // when user click on frame with button for open the list
                let words_from_file_cc = words_from_file.clone().unwrap();
                let list_name = frame_with_list_name.clone().buffer().unwrap().text();
                move |_button| { // when user click on frame with list name
                    create_words_from_list_window(list_name.clone(), &words_from_file_cc, config::Setting::app_default());
                }
            });

            // Style Events
            let both_hover = |btn: &mut Button| {
                btn.set_color(btn.color().lighter());
                draw::set_cursor(Cursor::Hand);
            };
            let both_leave = {
                let def_color_button = set.fr_elements_top_bar_background_color;
                move |btn: &mut Button| {
                    btn.set_color(def_color_button);
                    draw::set_cursor(Cursor::Default);
                }
            };
            open_list_button.on_hover(both_hover);
            open_list_button.on_leave(both_leave);
            frame_with_list_name_listener.on_hover({
                let mut list_button = button_open_list.clone();
                let mut words_count_info = words_count_in_list.clone();
                move |frame| {
                    // Set Style for frame
                    frame.set_color(frame.color().lighter());
                    draw::set_cursor(Cursor::Hand);

                    // Set Style for Words Count info
                    words_count_info.set_color(words_count_info.color().lighter());
                    words_count_info.redraw(); // load changed color

                    // Set style for Open List Button
                    list_button.set_color(list_button.color().lighter());
                    list_button.redraw(); // load changed color
                }
            });
            frame_with_list_name_listener.on_leave( {
                let def_color_button = set.fr_elements_top_bar_background_color;
                let mut list_button = button_open_list.clone();
                let mut words_count_info = words_count_in_list.clone();
                move |frame| {
                    // Set Style for frame
                    frame.set_color(def_color_button);
                    draw::set_cursor(Cursor::Default);

                    // Set Style for Words Count info
                    words_count_info.set_color(def_color_button);
                    words_count_info.redraw(); // load changed color

                    // Set style for Open List Button
                    list_button.set_color(def_color_button);
                    list_button.redraw(); // Load changed color
                }
            });

            // Add elements to the group container
            lists_container.add(&single_list_master_con);
        }
    
        // Creating Window with words occurs for the specified lists (this window will be created when user click on any list with words created by create_gui_list() function which is at the top :])
        fn create_words_from_list_window(name_words_list: String, list_words: &Vec<String>, set: Setting) {
            let frame_size = 750; // Size for frame with words list in x and y axis            
            
            // Create Window
            let screen_size = fltk::app::screen_size();
            let window_name = format!("Daisy Scraper -> Words Lists -> {}", name_words_list);
            let mut list_window = fltk::window::Window::new((screen_size.0 as i32 - 900) / 2, (screen_size.1 as i32 - 900) / 2, 900, 900, "");
            list_window.set_label(&window_name);
            list_window.set_color(set.app_backround_color);

            // Create List Title
            let mut list_title_container = Flex::new((list_window.width() - frame_size) / 2 + 10, 50, 300, 55, "")
                .row();
            list_title_container.set_frame(FrameType::BorderBox);
            list_title_container.set_color(set.fr_elements_top_bar_background_color);

            // -- Words List Icon Container
            let specified_words_list_icon = SvgImage::load("svg/word-icon.svg").expect(r#"Cound't load search icon from folder ./svg. Add svg file which is svg file and his name is "word-icon" ("word-icon.svg")"#);
            let mut words_icon = Frame::new(0, 0, 0, list_title_container.height(), "")
                .with_align(Align::Bottom | Align::Inside);
            words_icon.set_image(Some(specified_words_list_icon));

            // -- Words List Title
            let mut words_title = Frame::new((list_window.width() - frame_size) / 2 + 10, 50, 300, 55, "")
                .with_label("Selected List")
                .with_align(Align::Left | Align::Inside);
            words_title.set_label_color(set.element_font_color);
            words_title.set_label_font(Font::Courier);
            words_title.set_label_size(20);

            list_title_container.set_size(&mut words_icon, 50); // set width for the element with words list icon
            list_title_container.set_size(&mut words_title, 250); // set width for the element with words list title
            list_title_container.end();
            
            
            // Create Frame With All Words
            let mut container_with_words_list = group::Scroll::new((list_window.width() - frame_size) / 2 + 10, 120, frame_size, frame_size, "");
            container_with_words_list.set_frame(FrameType::BorderBox);
            container_with_words_list.set_color(set.fr_element_background_color);
            container_with_words_list.set_type(group::ScrollType::VerticalAlways);
            container_with_words_list.set_scrollbar_size(10); // scroll bar width
            
            let mut pack = group::Pack::new(0, 0, container_with_words_list.width(), container_with_words_list.height(), "");
            pack.clone().center_of_parent(); // the all elements is now in the center of the parent
            pack.set_spacing(5); // the space between elements in single column

            // Add Words to the words list
            let mut count = 0;
            for word in list_words {
                count += 1;
                let mut word_element = Flex::new(0, 0, container_with_words_list.width(), 50, "")
                    .row();

                // Create Button with element number in list
                let mut element_number_container = Frame::new(0, 0, 50, word_element.height(), "")
                    .with_align(Align::Center | Align::Inside)
                    .with_label(count.to_string().as_str());
                element_number_container.set_frame(FrameType::BorderBox);
                element_number_container.set_color(set.fr_elements_top_bar_background_color);
                element_number_container.set_label_color(set.element_font_color);
                element_number_container.clear_visible_focus();

                // Create Check Button
                let mut check_button = CheckButton::new(0, 0, 100, word_element.height(), "")
                    .with_label("learned")
                    .with_align(Align::Center | Align::Inside);
                check_button.set_frame(FrameType::BorderBox);
                check_button.set_color(set.fr_elements_top_bar_background_color);
                check_button.set_label_font(Font::Courier);
                check_button.set_label_size(16);
                check_button.set_label_color(set.element_font_color);
                check_button.clear_visible_focus();
                
                // Create Container with word text
                let mut word_text_buf = TextBuffer::default();
                word_text_buf.set_text(word);
                let mut word_text = fltk::text::TextDisplay::new(0, 0, word_element.width() - 100, word_element.height(), "");
                word_text.set_buffer(word_text_buf);
                word_text.set_frame(FrameType::BorderBox);
                word_text.set_color(set.fr_elements_top_bar_background_color);
                word_text.set_text_color(set.element_font_color);
                word_text.set_text_font(Font::Courier);


                word_element.set_size(&mut element_number_container, 50); // set size for the number info element
                word_element.set_size(&mut check_button, 100); // set size for the check button
                word_element.set_size(&mut word_text, word_element.width() - (element_number_container.width() + check_button.width()));
                word_element.end();

                // Listen events section
                let mut check_button_listener: Listener<_> = check_button.into();
                
                // When user click on "learned" check button
                check_button_listener.on_click({
                    let default_color = set.fr_elements_top_bar_background_color;
                    let mut number_element = element_number_container.clone();
                    let mut word_text = word_text.clone();
                    move |btn| {
                        if btn.color().to_rgb() == default_color.to_rgb() {
                            number_element.set_color(number_element.color().lighter()); // set number element color
                            btn.set_color(btn.color().lighter()); // set button color
                            word_text.set_color(word_text.color().lighter()); // set color word text element
                            number_element.redraw(); // load changes
                            word_text.redraw(); // load changes
                        }
                        else {
                            number_element.set_color(default_color); // reset number element color
                            btn.set_color(default_color);
                            word_text.set_color(default_color); // reset color word text element
                            number_element.redraw(); // reload changes
                            word_text.redraw(); // reload changes
                        }
                    }
                });

                // Add Word Element to the container with elements
                pack.add(&word_element);
            }

            // Load Changes to window and display window
            list_window.end();
            list_window.show();
        }
    }

    fn create_progress_frame(wn: DoubleWindow) -> (Progress, Frame, Frame, DoubleWindow) {
        let window_size = (wn.width(), wn.height());
        // Style of the progress bar window
        let mut info_title = Frame::new((window_size.0 - 500) / 2, (window_size.1 - (35 + 45)) / 2, 500, 20, "");
        info_title.set_label("Scrap words progress...");
        info_title.set_label_font(Font::Courier);
        info_title.set_align(Align::Left | Align::Inside);
        info_title.set_label_size(18);
        let mut progress_bar = Progress::new((window_size.0 - 500) / 2, (window_size.1 - 35) / 2, 500, 35, "");
        progress_bar.set_selection_color(Color::DarkGreen);
        let mut info_downloaded_success = Frame::new((window_size.0 - 500) / 2, (window_size.1 + 35) / 2, 500, 20, "");
        info_downloaded_success.set_label("0/0");
        info_downloaded_success.set_label_font(Font::Courier);
        info_downloaded_success.set_align(Align::Right | Align::Inside);
        let mut info_cant_download_from = Frame::new((window_size.0 - 500) / 2, (window_size.1 + 65) / 2, 500, 20, "");
        info_cant_download_from.set_label("Can't download words from: 0 pages");
        info_cant_download_from.set_label_color(Color::Red);
        info_cant_download_from.set_label_font(Font::Courier);
        info_cant_download_from.set_align(Align::Right | Align::Inside);
        info_cant_download_from.hide();

        (progress_bar, info_downloaded_success, info_cant_download_from, wn)
    }
}

pub fn create(r#type: CreateElementCategoryType) {
    let app_ = fltk::app::App::default();
    let screen_size = fltk::app::screen_size();
    let mut wn_ = fltk::window::Window::new((screen_size.0 as i32 - 900) / 2, (screen_size.1 as i32 - 900) / 2, 900, 900, "Daisy Scraper");
    let settings = config::Setting::app_default();

    // Set fonts for elements
    Font::set_font(Font::Courier, &settings.font); // replace Font::Courier by custon font

    // Set bacground Color of this application window
    wn_.set_color(settings.app_backround_color.clone());

    // Create Elements
    LoadElement::create(&mut wn_, &settings, r#type); // create search GUI (when program is started then is created "Search" Window by default)

    wn_.end();
    wn_.show();
    app_.run().unwrap();
}
