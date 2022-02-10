use clap::{ self, App, Arg };
use reqwest;
use tokio::{ self, spawn };
use scraper::{ Html, Selector };
use regex::{ Regex };
use serde::{ Serialize, Deserialize };
use serde_json;
use chrono::{ offset::* };
use std::{ fs, path::{ Path, PathBuf } };


#[derive(Serialize, Deserialize)]
struct JsonDocument
{
    url: String,
    words: Vec<String>
}

const FOLDER_FILES_WITH_WORDS: &str = "files";

fn save_words(d: Vec<String>, u: String) -> Result<String, String>
{
    // create file title
    let time_now = Local::now().format("%Y %b %d %H-%M-%S%.3f %z").to_string();

    // Serialize Data To JSON
    let struct_in = JsonDocument { url: u, words: d };
    let content_json_check = serde_json::to_string(&struct_in); // TODO: Better error handling without .expect
    
    match content_json_check
    {
        Ok(converted_val) => {
            // Save File
            let path_to: PathBuf = Path::new(".").join(FOLDER_FILES_WITH_WORDS).join(format!("{}.json", time_now));
            match fs::write(path_to, converted_val)
            {
                Ok(_) => {
                    Ok("saved".to_string())
                },
                Err(err) => {
                    Err(err.to_string())
                }
            }
        },
        Err(e) => Err(e.to_string())
    }
}

#[tokio::main]
async fn main() {
    let app = App::new("DaisyScraper")
        .author("https://github.com/kotekpsotek")
        .version("0.1")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .required(true)
                .takes_value(true)
                .multiple_values(true)
                .value_name("FIRST_PAGE_FROM_WHICH_YOU_WOULD_LIKE_GET_WORDS")
                // .value_name("FROM_WHERE_YOU_WOULD_LIKE_GET_WORDS")
                .help("Add url from where you would like to scarp words")
        )
    .get_matches();

    if app.is_present("url")
    {
        let urls_from_arg = app.values_of("url").unwrap().collect::<Vec<&str>>(); // get all added url from command line interface (CLI)
        
        // Iterate over added urls and go to them
        for url_from_arg in urls_from_arg
        {
            // Replace all found "\\" characters in url to "\"
            let regex_replace_needed = Regex::new(r"\\").unwrap().replace_all(url_from_arg, "/").to_string();
            let url_from_arg = regex_replace_needed.as_str();

            // when added url includes "//"" after added protocol
            match url_from_arg.find("//")
            {
                Some(byte_id) => {
                    let protocol_ = &url_from_arg[..byte_id + 2]; // select protocol which is used to connect with added url
                    let url_without_protocol = url_from_arg.replace(protocol_, ""); // url without protocol section for better error showing in error arms

                    if protocol_ == "https://" || protocol_ == "http://"
                    {
                        let request = reqwest::get(url_from_arg).await;
                        match request
                        {
                            Ok(res) => 
                            {
                                let status_base = res.status();
                                let status_code = status_base.as_u16();
                                let status_code_txt = status_base.canonical_reason();
                                if status_code >= 200 &&  status_code < 400
                                { // when result of response is good
                                    // Response HTTP data
                                    let _res_headers = res.headers();
                                    let response_url = res.url().to_string();
                                    let resonse_text = res.text().await.unwrap();
            
                                    // GET Text from body tag
                                    let parse_document = Html::parse_document(resonse_text.as_str());
                                    let selector = Selector::parse("body").expect("Program coudn't parse document <body></body> tag");
                                    let body_vec = parse_document.select(&selector).next().unwrap().text().collect::<Vec<&str>>();
            
                                    // Format text to more redable form using regexp
                                    let body_text = body_vec.join(" ");
                                    let regex_replace_st1 = Regex::new(r"\W|\d").unwrap().replace_all(body_text.as_str(), " ").to_string(); // replace non word characters and digest
                                    let regex_replace_st2 = Regex::new(r"\n(?s)|\s{2,}").unwrap().replace_all(regex_replace_st1.as_str(), "").to_string(); // replace \n words and \s which are 2 or more after itself
            
                                    // Format result text to vector
                                    let words_vec = regex_replace_st2.split(" ").collect::<Vec<&str>>();
            
                                    // Convert data from vec to string and better parse these words
                                    let mut string_vec: Vec<String> = Vec::new();
                                    for s_w in words_vec
                                    {
                                        let val = s_w.to_string();
                                        let regex_check_capital_let = Regex::new("[A-Z]").unwrap();
                                        let regex_check_space = Regex::new(r"\s").unwrap();
                                        if regex_check_capital_let.is_match(&val) && !regex_check_space.is_match(&val.trim()) 
                                        // when word has got capital letters in his body
                                        {
                                            // Add uppercase characters to vector
                                            let mut losed_uppercase_characters = Vec::<&str>::new();
                                            for mat in regex_check_capital_let.find_iter(&val)
                                            {
                                                let c_nv = mat.as_str();
                                                losed_uppercase_characters.push(c_nv);
                                            };
            
                                            if losed_uppercase_characters.join("").len() != val.len() // when word is build only with capital letters // only slice word on capital letters creating more then one word from single word when all word isn't build with capital letters
                                            {
                                                // Replace Capital letters spaces for create Vector with many words from one word
                                                let replace_capital_letter_to_spaces = regex_check_capital_let.replace_all(&val, " ").to_string();
                                                let vec_without_capital_letters = replace_capital_letter_to_spaces.split(" ").collect::<Vec<&str>>();
            
                                                // Add losed uppercase character to word again
                                                let mut result = Vec::<String>::new(); // vector with added uppercase character to other word fragment
                                                for number in 1..vec_without_capital_letters.len()
                                                {
                                                    let word = vec_without_capital_letters[number].trim();
                                                    let uppercase = losed_uppercase_characters[number - 1];
            
                                                    let mut word_col = word.split("").collect::<Vec<&str>>();
                                                    word_col.insert(0, uppercase); // add uppercase letter to word
                                                    let word_str = word_col.join("");
            
                                                    result.push(word_str.trim().to_string());
                                                };
            
                                                // Cancel action
                                                std::mem::drop(vec_without_capital_letters);
                                                string_vec.append(&mut result); // add to other words vector our result vector with converted word
                                            }
                                            else
                                            {
                                                // when word has got capital letter or letters but it is not build only with capital letters
                                                string_vec.push(val);
                                            };
                                        }
                                        else
                                        {
                                            // when word hasn't got any capital letters in his body
                                            string_vec.push(val);
                                        };
                                    };
            
                                    // println!("{}", string_vec.join(" ")); // Printing scrapped words in CLI
                                    let save_result = save_words(string_vec, response_url);
                                    match save_result
                                    {
                                        Ok(_) => println!("Words has been saved!!!"),
                                        Err(e) => panic!("{}", e)
                                    };
                                }
                                else
                                { // result of request isn't good
                                    let status_text_res: String = if let Some(code) = status_code_txt
                                                                  {
                                                                    code.to_string()
                                                                  }
                                                                  else
                                                                  {
                                                                    String::from("Code IS Unavaileble").to_uppercase()
                                                                  };
                                    println!("Program coudn't connect with given url from you. Reason:\n\n\tResponse HTTP status code: {code}\n\tResponse HTTP status text: {status_text}", code = status_code, status_text = status_text_res);
                                }
                            },
                            Err(err) => println!("Program coudn't sent request to added addres by you ({url_name}). Error description:\n{err_desc}", err_desc = err.to_string(), url_name = url_without_protocol)
                        };
                    }
                    else
                    {
                        println!("You add bad url construction for adress: {url_name}!!!\nUrl must starts with protocols http or https in this from \"https://{url_name}\" (\"https://target_domain.tld\") or \"http://{url_name}\" (\"http://target_domain.tld\")", url_name = url_without_protocol)
                    };    
                },
                None => println!("You add bad adress url format for {}", url_from_arg)
            };
        };
    }
    else
    {
        println!("You must give url or urls to where you would like to scrap words!!!")
    };
}