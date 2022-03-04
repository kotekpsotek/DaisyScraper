use core::panic;
use std::{
    fs,
    path::Path
};
use fs_extra::dir;

#[allow(dead_code)]
pub struct Features;
impl Features {
    pub fn get_flags_data_from_words_files() -> Result<Flags, String> { // this function must be safety for other components
        let path_string = format!("./files");
        let files_path = Path::new(&path_string);
        if files_path.exists() {
            let dir_checker = dir::get_dir_content(&files_path);
            if let Ok(data) = dir_checker {
                let files_count = data.files.len();
                if files_count >= 1 {
                    let dir_files = fs::read_dir(&files_path);
                    if let Ok(dir) = dir_files {
                        let  mut all_flags = Flags { data: Vec::<SingleFlag>::new() }; // this structure includes all flags in his data field and after when all flags has been added to his body this struct is returned from function in Result Ok()
                        // loop via all files
                        for elem in dir {
                            if elem.is_ok() {
                                let elem = elem.unwrap();
                                let elem_type = elem.file_type();
                                let file_name = elem.file_name()
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                if elem_type.is_ok() {
                                    let elem_type = elem_type.unwrap();

                                    if elem_type.is_file() { // function can only read data from file not from other fs elements
                                        let mut base_file_extension = file_name.split(".").collect::<Vec<&str>>();
                                        let file_extension = base_file_extension[base_file_extension.len() - 1].clone();
                                        base_file_extension.pop(); // delete the file extension
                                        let file_name_without_extension = base_file_extension.join(".");
                                        
                                        if file_extension == "json" { // Program can read only JSON file
                                            let file_flag_fragments = file_name_without_extension.split("flags_section").collect::<Vec<&str>>()[1].trim(); // in file name can be only one element which name is flags_section and this element must be in filename as same as flag from (so we must assume in file always be minimum 1 flag)
                                            let get_all_flags = file_flag_fragments.split("&").collect::<Vec<&str>>();
                                            
                                            for flag in get_all_flags {
                                                let flag_step1 = flag.replace("Xkd-=234s", "/");
                                                let flag_separate_name_with_value = flag_step1.split("=").collect::<Vec<&str>>();
                                                let name = flag_separate_name_with_value[0].to_string(); // flag name
                                                let value = flag_separate_name_with_value[1].to_string(); // flag value
                                                
                                                // Create struct for this specific file and add this flag to struct with all flags which is next returned from function
                                                let struct_flag = SingleFlag {
                                                    name,
                                                    value
                                                };
                                                all_flags.data.push(struct_flag);
                                            }
                                        }
                                        else { // when file is other loop go to next iteration
                                            continue;
                                        }

                                    }
                                    else { // when type of element isn't file loop go to next iteration
                                        continue;
                                    }

                                }
                                else { // when is error loop go to next iteration
                                    continue;
                                }
                            }
                            else { // when is error loop go to next iteration
                                continue;
                            }
                        };
                        
                        Ok(all_flags) // return flags 
                    }
                    else {
                        Err(dir_files.unwrap_err().to_string())
                    }
                }
                else {
                    Err(r#""files" folder is empty or contains only folders when program can read only data form files!!!"#.to_string())
                }
            }
            else {
                Err(format!("Program coudn't check {:?} content!!!", files_path))   
            }
        }
        else {
            Err("Files directory doesn't exists!!!".to_string())
        }
    }

    // Get All Files Names from files in ./files folder
    pub fn get_files_names() -> Vec<String> {
        let read_dir = fs::read_dir(Path::new("./files")).unwrap();
        let mut vec = Vec::<String>::new();

        for file in read_dir {
            let file = file.unwrap();
            let file_name = file
                .file_name()
                .to_str()
                .unwrap()
                .to_string();
            let file_name_without_flags = file_name.split("flags_section")
                .collect::<Vec<&str>>()[0]
                .trim()
                .to_string();
            vec.push(file_name_without_flags);
        } 

        return vec;
    }

    pub fn get_words_in_file(file_name: String) -> Option<Vec<String>> {
        let read_dir = fs::read_dir(Path::new("./files")).unwrap();
        let mut vec = Vec::<String>::new();
        
        // Get words from file and put it into Vector
        for file in read_dir {
            let file = file.unwrap();
            let iter_file_name = file.file_name()
                .to_str()
                .unwrap()
                .to_string();
            let iter_file_name_without_flags = iter_file_name.split("flags_section")
                .collect::<Vec<&str>>()[0]
                .trim()
                .to_string();
            
            if iter_file_name_without_flags == file_name {
                let this_file_content = fs::read_to_string(Path::new(&format!("./files/{}", iter_file_name))).unwrap();
                let mut deserialized_content: crate::scrap::JsonDocument = serde_json::from_str(&this_file_content).unwrap();

                vec.append(&mut deserialized_content.words);
                break;
            }
        }

        // Return Some when in vector are words from file or None when in vector aren't words from file
        if vec.len() > 0 {
            Some(vec)
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Flags { // this structure includes into his body all flags and add methods to read this flags in simple way
    pub data: Vec<SingleFlag>
}

#[allow(dead_code)]
impl Flags {
    pub fn get_elements_count(&self) -> usize {
        self.data.len()
    }

    pub fn get_element_from_index(&mut self, index: usize) -> ReturnOutsideFrameData {
        if index <= self.data.len() {
            let element = &mut self.data[index];
            let return_instance = ReturnOutsideFrameData {
                name: element.name.clone(),
                value: element.convert_value()
            };
            return_instance
        } else {
            panic!("You add too height index")
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SingleFlag {
    pub name: String,
    pub value: String
}

#[allow(dead_code)]
impl SingleFlag {
    fn convert_value(&mut self) -> (String, String, String, Option<String>) { // for moment when this method was created the values can be only in tuple types
        // Remove Brackets
        let converting_base = &mut self.value;
        converting_base.remove(0); // remove the first bracket "(" from value String
        converting_base.pop(); // remove the last bracket "(" from value String

        // Split element base on comma ","
        let without_comma = converting_base.split(",").collect::<Vec<&str>>();
        
        // Assign elements to the return value
        let protocol = without_comma[0].trim().to_string();
        let url_domain_name = without_comma[1].trim().to_string();
        let url_path_section = without_comma[2].trim().to_string();
        let port = if without_comma.len() == 4 && without_comma[3].trim() != "null" {
            Some(without_comma[3].trim().to_string())
        }
        else {
            None
        };

        let return_value: (String, String, String, Option<String>) = (protocol, url_domain_name, url_path_section, port);
        return_value
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ReturnOutsideFrameData { // this is a struct with converted values which is returned from Flags get_element_from_index() method
    pub name: String,
    pub value: (String, String, String, Option<String>)
}