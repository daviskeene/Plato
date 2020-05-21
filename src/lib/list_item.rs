use std::fs;
use std::ffi::OsString;
use chrono::{DateTime, Local};
use super::file;
use std::collections::HashMap;
use std::path::PathBuf;
use std::vec::Vec;
use pathdiff::diff_paths;

use crate::lib::list;

#[derive(Debug)]
pub struct ListItem {
    pub owning_user: OsString,
    pub file_size: String,
    pub time_last_modified: DateTime<Local>,
    pub name: String
}

pub fn get_file_data(path: PathBuf, directory: &PathBuf, flags: &HashMap<&str, bool>) -> ListItem {
    let meta = fs::metadata(&path).unwrap();
    let owning_user = file::get_owning_user(&meta);
    let time_last_modified = file::get_time_last_modified(&meta);
    let relative_path = diff_paths(&path, &directory);
    let name = match relative_path {
        Some(f) => f.to_str().unwrap().to_string(),
        None => path.to_str().unwrap().to_string()
    };

    let file_size = if *flags.get("human").unwrap() && *flags.get("long").unwrap() {
        file::get_human_file_size(&meta)
    }
    else if *flags.get("long").unwrap() {
        file::get_file_size(&meta).to_string()
    }
    else {
        "".to_string()
    };

    let list_item = ListItem {
        owning_user: owning_user,
        file_size: file_size,
        time_last_modified: time_last_modified,
        name: name
    };

    return list_item;
}

// Uses the current directory and the associated metadata with a file to
// create list item structs for each file. Depending on the flags, the list item 
// content might be different for file size. 
pub fn create_list_items(base: PathBuf, flags: &HashMap<&str, bool>) -> Vec<ListItem> {
   
    let mut list_items = Vec::new();
    let result = list::get_directory_contents(&base);
    match result {
        Ok(dir_items) => {
            let hidden = flags.get("hidden").unwrap();
            for item in dir_items {
                let path = item.unwrap().path();
                if !hidden {
                    if file::is_hidden_file(&path, &base) {
                        continue;
                    }
                }
                let list_item = get_file_data(path, &base, flags);
                list_items.push(list_item);
            }
            return list_items;
        },
        Err(_e) => {
            let current_directory = list::get_current_directory();
            let list_item = get_file_data(base, &current_directory, flags);
            list_items.push(list_item);
            return list_items;
        }
    }
}
