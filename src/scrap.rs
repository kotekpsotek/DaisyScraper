use chrono::offset::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tokio::{self, task::JoinHandle};
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};

#[derive(Serialize, Deserialize)]
struct JsonDocument {
    url: String,
    words: Vec<String>,
}

const FOLDER_FILES_WITH_WORDS: &str = "files";

fn add_flags_to_file_name(base_name: String, flags: Vec<(String, String, Option<String>)>) -> String {
    /* Parameters Description: 
        base_name - it's a base name of file without any flags only with time downloaded using chrono crate,
        flags - this is a list of data which must be writed to the name // Flags are always added to the end of the name // Values in tuple: 0 - protocol off url, 1 - name of url
    */
    let flg_name_per_flg_vec_index = ["from"]; // 0 - from:page_url (From what url words has been downloaded)
    let mut name_conversion = base_name; // this is a value where new values will be add
    name_conversion.push_str(" flags_section "); // add "monkey" symbol to the end of name 
    
    let flag_count = flg_name_per_flg_vec_index.len();
    let mut it_count = 0;
    while flag_count > it_count { // loop bases on 
        // Flag fragments
        let name_for_flag = flg_name_per_flg_vec_index[it_count];
        let protocol_for_flag = flags[it_count].0
            .replace("://", "");
        let domain_for_flag = flags[it_count].1
            .clone();
        let port = if let Some(val) = flags[it_count].2.clone() {
            val
        }
        else
        {
            String::from("null")
        };
        let value_for_flag = (protocol_for_flag, domain_for_flag, port);
        // Ready Flag
        let mut ready_flag = format!("{name}={value:?}", name = name_for_flag, value = value_for_flag);
        ready_flag.push('&');

        // Add Flag to Name which is return
        name_conversion.push_str(&ready_flag);

        it_count += 1;
    };
    let mut ready_name = name_conversion
        .trim()
        .replace("\"", "");
    let ready_name = if ready_name.ends_with("&") {
        ready_name.pop().unwrap();
        ready_name
    }
    else {
        ready_name
    };
    ready_name
}

fn save_words(d: Vec<String>, u: String, from: (String, String, Option<String>)) -> Result<String, String> {
    /* Parameters Description:
        d - this is a list with scraped words from indicated url,
        u - it is a url from where response has been send (response to your reequest because if you would like scrap words you must send request to some kind url),
        from - it is a collection of url from where words has been downloaded: 0 [key] - this is not converted to be correct for FileSystem protocol of page, 1 [key] - this is a url from where words has been downloaded, 2 - port from the Url or None value,
    */
    
    // create time for file title
    let time_now = Local::now().format("%Y %b %d %H-%M-%S%.3f %z").to_string();
    // Create vector with flags which are next add to the file name
    let flags_vec = vec![from];
    

    // Serialize Data To JSON
    let struct_in = JsonDocument { url: u, words: d };
    let content_json_check = serde_json::to_string(&struct_in);

    match content_json_check {
        Ok(converted_val) => {
            // Save File
            let file_name: String = format!("{}.json", add_flags_to_file_name(format!("{}", time_now), flags_vec.clone()));
            let path_to: PathBuf = Path::new(".")
                .join(FOLDER_FILES_WITH_WORDS)
                .join(file_name);
            match fs::write(path_to, converted_val) {
                Ok(_) => Ok("saved".to_string()),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn scrap_from(urls_from_arg: Vec<&str>)
{
    let mut joinhandle_process_vec = Vec::<JoinHandle<()>>::new();

    // Iterate over added urls and go to them
    for url_from_arg in &urls_from_arg {
        // Replace all found "\\" characters in url to "\"
        let url_from_arg = Regex::new(r"\\")
            .unwrap()
            .replace_all(url_from_arg, "/")
            .to_string();
        let process_for_url = tokio::spawn(async move {
            match url_from_arg.find("//") {
                Some(byte_id) => {
                    let protocol_ = &url_from_arg[..byte_id + 2]; // select protocol which is used to connect with added url
                    let mut url_without_protocol = url_from_arg.replace(protocol_, ""); // url without protocol section for better error showing in error arms
                    if protocol_ == "https://" || protocol_ == "http://" {
                        let request = reqwest::get(&url_from_arg).await;
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
            
                                    // TODO: Check if this action shoudn't be in better place
                                    // Prepare url to be saved correctly (remove dangerous characters like "/" on the end of url)
                                    let url_without_protocol = if url_without_protocol.ends_with("/") {
                                        url_without_protocol.pop();
                                        url_without_protocol
                                    }
                                    else {
                                        url_without_protocol
                                    };

                                    // Port of the URL
                                    let regex_port = Regex::new(r":\d{1,}").unwrap();
                                    let port = if let Some(port) = regex_port.find(&url_without_protocol) {
                                        Some(port.as_str().replace(":", "").to_string())
                                    }   
                                    else
                                    {
                                        None
                                    };

                                    // Remove Port from the default URL
                                    let url_without_protocol = regex_port.replace(&url_without_protocol, "").to_string();
                                    

                                    let save_result = save_words(string_vec, response_url, (protocol_.to_string(), url_without_protocol, port));
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
                    } else {
                        println!("You add bad url construction for adress: {url_name}!!!\nUrl must starts with protocols http or https in this from \"https://{url_name}\" (\"https://target_domain.tld\") or \"http://{url_name}\" (\"http://target_domain.tld\")", url_name = url_without_protocol)
                    };
                }
                None => println!("You add bad adress url format for {}", url_from_arg),
            };
        });
        joinhandle_process_vec.push(process_for_url);
    }

    let mut url_for_task_num = 0;
    for process in joinhandle_process_vec
    // enable result from sub task handled by tokio runtime
    {
        let result = process.await;
        url_for_task_num += 1;
        match result {
            Err(_) => {
                println!(
                    "Program coudn't fire task for url: {}",
                    urls_from_arg[url_for_task_num]
                );
            }
            _ => continue,
        };
    }
}