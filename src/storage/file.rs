use crate::config;

use super::model::{self, set::Set};
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;


pub fn load_file(path: &Path) -> Option<File> {
    let file = OpenOptions::new()
        .read(true)
        .open(path);

    match file {
        Ok(file_instance) => Some(file_instance), 
        Err(_) => None
    }
}

pub fn serialize_set_to_file(set: Set) -> bool {
    todo!()
}

pub fn write_file() {
}
