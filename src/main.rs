use crate::{csv_lib::{CsvRecord, parse_csv_records}, file_lib::{get_current_attempt_files, get_current_commit_files}};
use audio_recorder_rs::Recorder;
use eframe::{Frame, egui, wgpu::naga::FastHashSet};
use egui::Ui;
use rodio::{OutputStream, source::Source};
use std::{collections::{HashSet, hash_set}, path::{self, PathBuf}, rc::Rc};

mod audio_lib;
mod csv_lib;
mod file_lib;
mod ui_lib;

#[derive(Debug, PartialEq)]
enum CurrentMode {
    Idle,
    Recording,
    Playback,
}

// #[derive(Debug)]
struct DubiousDubsState {
    enable_top: bool,
    enable_bottom: bool,
    enable_left: bool,
    enable_right: bool,
    sel_13: usize,
    is_first_frame:bool,
    current_mode: CurrentMode,
    attempt_directory: String,
    commit_directory: String,
    recorder: audio_recorder_rs::Recorder,
    player: rodio::Sink,
    player_out_stream: OutputStream, // do not get rid of
    csv_records: Vec<Rc<CsvRecord>>,
    filtered_csv_records: Vec<Rc<CsvRecord>>,
    selected_csv_record: Rc<CsvRecord>,
    csv_filter: String,
    is_csv_filtered_by_uncommitted: bool,
    selected_dub_id: String,
    hovered_dub_index: usize,

    current_attempt_filenames: Vec<String>,
    next_attempt_filename:String,
    next_attempt_path:PathBuf,
    hovered_attempt_filename: String,
    selected_attempt_filename: String,

    committed_dub_id_lookup: FastHashSet<String>,

    is_instant_replay_enabled: bool    

}

impl Default for DubiousDubsState {
    fn default() -> Self {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        let csv_results = parse_csv_records().unwrap();

        let mut selected_dub_id = "".to_string();
        let mut selected_csv_record = Rc::new(CsvRecord::default());
        let selected_attempt_filename = "".to_string();

        if csv_results.len() > 0 {
            let rec = &csv_results[0];
            selected_dub_id = rec.dub_id.to_string();
            selected_csv_record = rec.clone();
        }

        Self {
            enable_top: true,
            enable_bottom: false,
            enable_left: false,
            enable_right: false,
            sel_13: 0,
            is_first_frame: true,
            current_mode: CurrentMode::Idle,
            attempt_directory: "Attempts".to_owned(),
            commit_directory: "Commit".to_owned(),
            recorder: Recorder::default(),
            player: sink,
            player_out_stream: stream_handle,
            filtered_csv_records: csv_results.iter().cloned().collect(), // run before csv_records
            csv_records: csv_results,
            selected_csv_record: selected_csv_record,
            csv_filter: "".to_owned(),
            is_csv_filtered_by_uncommitted: true,
            selected_dub_id: selected_dub_id,
            hovered_dub_index: usize::MAX,

            current_attempt_filenames: vec![],
            next_attempt_filename: "".to_owned(),
            next_attempt_path: PathBuf::default(),
            hovered_attempt_filename: "".to_owned(),
            selected_attempt_filename: selected_attempt_filename,
            
            committed_dub_id_lookup: FastHashSet::default(),

            is_instant_replay_enabled: false
        }
    }
}

// =============================================================================
// Application entry point
// =============================================================================

pub struct DubiousDubsApp {
    state: DubiousDubsState,
}

impl Default for DubiousDubsApp {
    fn default() -> Self {
        Self {
            state: <DubiousDubsState>::default(),
        }
    }
}

impl eframe::App for DubiousDubsApp {
    // The update loop for the GUI
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {

        get_current_attempt_files(&mut self.state);
        get_current_commit_files(&mut self.state);
        ui_lib::generated_ui_panels(ui, &mut self.state);

        if self.state.is_first_frame{
            self.state.is_first_frame = false;
        }
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size((1800.0, 800.0))
            .with_min_inner_size((1800.0, 800.0)),
        centered: true,
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Dubious Dub",
        native_options,
        Box::new(|_cc| Ok(Box::new(DubiousDubsApp::default()))),
    )
}
