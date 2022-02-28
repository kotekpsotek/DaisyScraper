use std::{ fs, path::Path };
use serde::{ Serialize, Deserialize };
use serde_json;

#[derive(Serialize, Deserialize)]
pub enum OpenedWindow {
    Menu,
    Search
}

#[derive(Serialize, Deserialize)]
pub struct State {
    last_opened_window_is: OpenedWindow
}

// Return the last opened window type
pub fn which_widnow_was_opened_recently() -> Result<OpenedWindow, String> {
    let data_from_state_file = fs::read_to_string(Path::new("state").join("state.json"));
    match data_from_state_file {
        Ok(string_data) => {
            let deserialized_data = serde_json::from_str(&string_data) as Result<State, _>;
            match deserialized_data {
                Ok(data) => 
                    Ok(data.last_opened_window_is),
                Err(_) => // When read data coudn't be deserialized then type of the window is set to "Search"
                    Ok(OpenedWindow::Search)
            }
        },
        Err(_) => { // When program cound't read state
            self::save_window_state(OpenedWindow::Search).unwrap();
            Err(String::from("the_data_cound't_be_read"))
        }
    }
}

// Save/Update the last opened window type
pub fn save_window_state(r#type: OpenedWindow) -> Result<(), String> {
    let other_datas_from_state_file = fs::read_to_string(Path::new("state").join("state.json"));
    let path_to_loc = Path::new("state").join("state.json");
    match other_datas_from_state_file {
        Ok(string_data) => {
            // Deserialize data
            let mut deserialized_data: State = serde_json::from_str(&string_data).unwrap();
            deserialized_data.last_opened_window_is = r#type; // update the opened window state
            // Serialize data and save
            let serialized_data_again = serde_json::to_string(&deserialized_data).unwrap();
            let write_res = fs::write(path_to_loc, serialized_data_again); // try save update result
            match write_res { // handle update try result
                Ok(_) => 
                    Ok(()),
                Err(_) => 
                    Err(String::from("The data coudn't be saved"))
            }
        },
        Err(_) => {
            // When file coudn't be read the file is resaved with correct data again
            let state = State { // state which is saved in file
                last_opened_window_is: OpenedWindow::Search
            };
            let serialized_data = serde_json::to_string(&state);
            match serialized_data {
                Ok(serialized_data) => {
                    let write_res = fs::write(path_to_loc, serialized_data);
                    match write_res {
                        Ok(_) => 
                            Ok(()),
                        Err(_) => 
                            Err(String::from("The data coudn't be saved"))
                    }
                },
                Err(_) => 
                    Err(String::from("the_data_cound't_be_read"))
            }
        }
    }
}