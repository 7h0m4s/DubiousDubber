use std::error::Error;
use std::{fs, io};
use std::io::*;
use std::fs::{DirEntry, FileType, ReadDir};
use std::path::{Path, PathBuf};
use eframe::wgpu::naga::compact::KeepUnused::No;

use crate::DubiousDubsState;
use std::iter::{FilterMap, Map};
use std::string;

pub fn get_current_attempt_files(state: &mut DubiousDubsState) -> Result<Vec<String>> {

    let mut current_attempt_path = PathBuf::from(&state.attempt_directory);
    current_attempt_path.push(&state.selected_dub_id);

    // make sure the directory exists
    fs::create_dir_all(&current_attempt_path)?;

    // read_dir returns an iterator of Results.
    // We use expect here because if the directory doesn't exist,
    // there's no point in continuing.

    let mut attempt_files = Vec::new() as Vec<String>;

    if let Ok(entries) =  fs::read_dir(&current_attempt_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let path_buf = entry.path() as PathBuf {
                   attempt_files.push(path_buf.file_name().unwrap().to_string_lossy().into_owned());
                } else {
                    println!("Couldn't get path for {:?}", entry.path());
                }
            }
        }
    }

    Ok(attempt_files)
}

pub fn get_next_attempt_filename_from_state(state: &mut DubiousDubsState) -> Result<String> {
   let existing_attempts = get_current_attempt_files(state).unwrap();
    let mut max_file_number:usize = 0;
    for attempt in existing_attempts as Vec<String> {
        let underscore_index :usize = attempt.find('_').unwrap();
        let mut attempt_string: String = attempt;
        let file_number_string:String = attempt_string.split_off(underscore_index);
        let file_number:usize = file_number_string.parse::<usize>().unwrap();
        if file_number > max_file_number {max_file_number = file_number}
    }


    Ok(format!("{}_{}", &state.selected_dub_index, max_file_number))
}

pub fn get_next_attempt_filename_from_file_list(filenames: &Vec<String>, state: &mut DubiousDubsState) -> Result<String> {
    let mut max_file_number:usize = 0;
    for attempt in filenames{
        //let underscore_index :usize = attempt.find('_').unwrap();
        let underscore_index :usize = match attempt.find('_'){
            Some(a) => a,
            None => continue
        };
        let mut attempt_string: String = attempt.clone();
        let file_number_string:String = attempt_string.split_off(underscore_index);
        let file_number:usize = file_number_string.parse::<usize>().unwrap();
        if file_number > max_file_number {max_file_number = file_number}
    }


    Ok(format!("{}_{}", &state.selected_dub_index, max_file_number))
}