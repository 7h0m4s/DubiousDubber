use crate::{csv_lib::{CsvRecord, parse_csv_records}, file_lib::{get_current_attempt_files}};
use audio_recorder_rs::Recorder;
use eframe::{Frame, egui};
use egui::Ui;
use rodio::{OutputStream, source::Source};
use std::path::{self, PathBuf};

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
    current_mode: CurrentMode,
    selected_filename: String,
    attempt_directory: String,
    recorder: audio_recorder_rs::Recorder,
    player: rodio::Sink,
    player_out_stream: OutputStream, // do not get rid of
    csv_records: Vec<CsvRecord>,
    selected_csv_record: CsvRecord,
    selected_dub_id: String,
    hovered_dub_index: usize,
    selected_dub_index: usize,

    current_attempt_filenames: Vec<String>,
    next_attempt_filename:String,
    next_attempt_path:PathBuf,
    hovered_attempt_filename: String,
    selected_attempt_filename: String,

    

}

impl Default for DubiousDubsState {
    fn default() -> Self {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        let csv_results = parse_csv_records().unwrap();

        let mut selected_dub_id = "".to_string();
        let mut selected_dub_index = 0;
        let mut selected_filename = "".to_string();
        let mut selected_csv_record = CsvRecord::default();
        let selected_attempt_filename = "".to_string();

        if csv_results.len() > 0 {
            let rec = &csv_results[0];
            selected_dub_id = rec.dub_id.to_string();
            selected_dub_index = 0;
            selected_filename = rec.filename.to_string();
            selected_csv_record = rec.clone();
        }

        Self {
            enable_top: true,
            enable_bottom: false,
            enable_left: false,
            enable_right: false,
            sel_13: 0,
            current_mode: CurrentMode::Idle,
            selected_filename: selected_filename,
            attempt_directory: "Attempts".to_owned(),
            recorder: Recorder::default(),
            player: sink,
            player_out_stream: stream_handle,
            csv_records: csv_results,
            selected_csv_record: selected_csv_record,
            selected_dub_id: selected_dub_id,
            hovered_dub_index: selected_dub_index,
            selected_dub_index: 0,

            current_attempt_filenames: vec![],
            next_attempt_filename: "".to_owned(),
            next_attempt_path: PathBuf::default(),
            hovered_attempt_filename: "".to_owned(),
            selected_attempt_filename: selected_attempt_filename,
            
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
        ui_lib::generated_ui_panels(ui, &mut self.state);
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
