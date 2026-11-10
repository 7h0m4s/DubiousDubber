use std::error::Error;
use std::str::FromStr;
use std::{fs, io};
use std::io::*;
use std::fs::{DirEntry, FileType, ReadDir};
use std::path::{Path, PathBuf};

use cpal::FromSample;
use eframe::wgpu::naga::FastHashSet;
use eframe::wgpu::naga::compact::KeepUnused::No;

use crate::DubiousDubsState;
use std::iter::{FilterMap, Map};
use std::string;

pub fn get_current_attempt_files(state: &mut DubiousDubsState) -> Result<()> {
    let mut current_attempt_path = PathBuf::from(&state.attempt_directory);

    // make sure the directory exists
    if(state.is_first_frame){fs::create_dir_all(&current_attempt_path)?};

    let files: Vec<String> = fs::read_dir(&current_attempt_path)?
    .filter_map(Result::ok)
    .filter_map(|f| f.file_name().into_string().ok())
    .filter(|f| f.starts_with(&state.selected_dub_id) && f.ends_with(".wav"))
    .map(|f| f.to_string())
    .collect();

    state.current_attempt_filenames = files;

    // Populate next_attempt_filename
    let mut max_file_number:usize = 0;
    for attempt in &state.current_attempt_filenames{
        let underscore_index :usize = match attempt.rfind('_'){
            Some(a) => a,
            None => continue
        };
        let mut attempt_string: String = attempt.clone();
        let file_number_and_wav_string:String = attempt_string.split_off(underscore_index + 1); // +1 to skip _ char
        let file_number_only_string = &file_number_and_wav_string[..file_number_and_wav_string.len() - 4];
        let file_number:usize = match file_number_only_string.parse::<usize>(){
            Ok(a) => a,
            Err(_e) => continue
        };
        if file_number > max_file_number {max_file_number = file_number}
    }
    state.next_attempt_filename = format!("{}_{:0>3}.wav", &state.selected_dub_id, max_file_number + 1);
    let mut attempt_path = PathBuf::from_str(&state.attempt_directory).unwrap();
    attempt_path.push(&state.next_attempt_filename);
    state.next_attempt_path = attempt_path;


    Ok(())
}


pub fn get_current_commit_files(state: &mut DubiousDubsState) -> Result<()> {
    let mut current_commit_path = PathBuf::from(&state.commit_directory);

    // make sure the directory exists
    if(state.is_first_frame){fs::create_dir_all(&current_commit_path)?};

    let files: FastHashSet<String> = fs::read_dir(&current_commit_path)?
    .filter_map(Result::ok)
    .filter_map(|f| f.file_name().into_string().ok())
    .filter(|f| f.ends_with(".wav"))
    .map(|f| f.to_string().replace(".wav", ""))
    .collect();

    state.committed_dub_id_lookup = files;

    Ok(())
}

pub fn commit_selected_attempt_file(state: &mut DubiousDubsState) -> std::io::Result<()> {

    if state.selected_attempt_filename == "" { 
        return Ok(());
    }
    let attempt_path :PathBuf = [&state.attempt_directory, &state.selected_attempt_filename].iter().collect();
    let commit_path :PathBuf = [&state.commit_directory, &format!("{}.wav", &state.selected_dub_id) ].iter().collect();

    fs::create_dir_all(PathBuf::from(&state.commit_directory))?;
    fs::copy(attempt_path, commit_path)?;  
    Ok(())
}
