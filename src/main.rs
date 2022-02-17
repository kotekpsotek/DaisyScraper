mod gui; // GUI module
mod scrap; // SCRAP module
mod config { // GUI config
    pub mod default;
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
        let urls_from_arg = app.values_of("url").unwrap().collect::<Vec<&str>>(); // get all added url from command line interface (CLI)
        scrap_from(urls_from_arg).await;
    } else {
        // Launch GUI
        // GUI library: FLTK, GLK
        println!("GUI application has been launched!!!");
        gui::create(); // create GUI app
    };
}
