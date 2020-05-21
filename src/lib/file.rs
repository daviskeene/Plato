use std::fs;
use fs::Metadata;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Local};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use users;
use std::ffi::OsString;
use pretty_bytes::converter::convert;
use pathdiff::diff_paths;
use std::path::PathBuf;

// Takes in the raw permission decimal value and returns the 
// the permissions string using the letter format. Grabs the 
// permissions for user, group, and other. The S_IRUSR constants
// are used for doing bitwise and operations to check if those bits
// are set in the file's permissions. 
fn parse_permissions(is_dir: bool, mode: u32) -> String {
    let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
    let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
    let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
    if is_dir {
        ["d".to_string(), user, group, other].join("")
    }
    else {
        ["-".to_string(), user, group, other].join("")
    }
}

// Uses bitwise and operator to fetch what permissions are set for this object.
fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String {
    match (mode & u32::from(read), mode & u32::from(write), mode & u32::from(execute)) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }.to_string()
}

// Determines if a file is a hidden file or not.
pub fn is_hidden_file(file: &PathBuf, current_dir: &PathBuf) -> bool{
    let relative_path = diff_paths(&file, &current_dir).unwrap();
    let file = relative_path.to_str().unwrap();
    return file.starts_with(".")
}

// Get the permissions using the associate file metadata
pub fn get_permissions(meta: &Metadata) ->  String {
    return parse_permissions(meta.is_dir(), meta.mode());
}

// Get the name of the owning user of this file
pub fn get_owning_user(meta: &Metadata) -> OsString {
    let user = users::get_user_by_uid(meta.uid()).unwrap();
    return user.name().to_os_string();
}

// Get the time this file was last modified
pub fn get_time_last_modified(meta: &Metadata) -> DateTime<Local> {
    let modified: DateTime<Local> = DateTime::from(meta.modified().unwrap());
    modified
}

// Get the file size
pub fn get_file_size(meta: &Metadata) -> u64 {
    if meta.is_dir() {
        0
    }
    else {
        meta.size()
    }
}

// Get the file size but formatted to easily be read
// Uses the rust pretty bytes crate 
// https://github.com/banyan/rust-pretty-bytes
pub fn get_human_file_size(meta: &Metadata) -> String {
    let num = get_file_size(meta);
    return convert(num as f64);
}

