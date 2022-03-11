mod gui; // GUI module
mod scrap; // SCRAP module
mod states; // APP STATES module
mod config { // GUI config and Other configs
    pub mod default;
    pub mod additional; // additional functions for application
}
use clap::{self, App, Arg};
use scrap::scrap_from;

#[tokio::main]
async fn main() {
    let app = App::new("DaisyScraper")
        .author("https://github.com/kotekpsotek")
        .version("0.1")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .takes_value(true)
                .multiple_values(true)
                .value_name("FIRST_PAGE_FROM_WHICH_YOU_WOULD_LIKE_GET_WORDS")
                .value_name("FROM_WHERE_YOU_WOULD_LIKE_GET_WORDS")
                .help("Add url from where you would like to scarp words"),
        )
        .get_matches();

    if app.is_present("url") {
        let urls_from_arg = app.values_of("url").unwrap().collect::<Vec<&str>>().iter().map(|val| val.to_string()).collect(); // get all added url from command line interface (CLI)
        scrap_from(urls_from_arg, None, None).await;
    } else {
        // Launch GUI
        // GUI library: FLTK, GLK
        gui::create(gui::CreateElementCategoryType::Default); // create GUI app
    };
}

#[cfg(test)]
mod Test {
    use crate::config::additional;
    #[test]
    fn load_flags_from_file_test() {
        let mut _flags = additional::Features::get_flags_data_from_words_files().unwrap();
        /* let specified_element = flags.get_element_from_index(0);
        let port = specified_element.value.2.unwrap();
        println!("{}", port); */
    }

    #[test]
    fn file_from_dir() {
        let mut action = additional::Features::get_flag_from_specifici_filename(format!("2022 Mar 11 01-58-25.648 +0100"), format!("from")).unwrap();
        let action_value_converted = action.convert_value();
        println!("Flag Value is: {:?}", action_value_converted)
    }
}