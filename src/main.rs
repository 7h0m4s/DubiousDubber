#![windows_subsystem = "windows"]

use crate::csv_lib::initialize_csv_state;
use crate::ui_lib::set_display_visuals;
use crate::{
    csv_lib::CsvRecord,
    file_lib::{get_current_attempt_files, get_current_commit_files},
};
use audio_recorder_rs::Recorder;
use eframe::{Frame, egui, wgpu::naga::FastHashSet};
use egui::Ui;
use egui_notify::Toasts;
use rodio::{OutputStream, Sink};
use std::{fmt, path::PathBuf, rc::Rc};

mod audio_lib;
mod csv_lib;
mod file_lib;
mod ui_lib;

pub static CSV_FILEPATH: &str = "AllLinesExport.csv";

#[derive(Debug, PartialEq)]
enum CurrentMode {
    Idle,
    Recording,
    Playback,
}

//#[derive(Debug)] // not needed with Debug implementation
struct DubiousDubsState {
    enable_top: bool,
    enable_bottom: bool,
    enable_left: bool,
    enable_right: bool,
    is_first_frame: bool,
    is_after_refresh: bool,
    is_light_mode: bool,
    volume: f32,
    ui_csv_table_height: f32,
    ui_csv_table_min_height: f32,
    current_mode: CurrentMode,
    csv_filepath: PathBuf,
    attempt_directory: String,
    commit_directory: String,
    recorder: audio_recorder_rs::Recorder,
    player: Option<rodio::Sink>,
    player_out_stream: Option<OutputStream>, // do not get rid of
    csv_records: Vec<Rc<CsvRecord>>,
    has_filtered_csv_changed: bool,
    filtered_csv_records: Vec<Rc<CsvRecord>>,
    selected_csv_record: Rc<CsvRecord>,
    csv_filter: String,
    is_csv_filtered_by_uncommitted: bool,
    selected_dub_id: String,
    hovered_dub_index: usize,

    current_attempt_filenames: Vec<String>,
    next_attempt_filename: String,
    next_attempt_path: PathBuf,
    hovered_attempt_filename: String,
    selected_attempt_filename: String,

    committed_dub_id_lookup: FastHashSet<String>,

    is_instant_replay_enabled: bool,
    is_filter_box_in_focus: bool,
    toasts: Toasts,
}

impl fmt::Debug for DubiousDubsState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DubiousDubsState")
            .field("player", &"Debug data not available")
            .field("player_out_stream", &"Debug data not available")
            .field("toasts", &"Debug data not available")
            .finish()
    }
}

impl Default for DubiousDubsState {
    fn default() -> Self {
        let mut toasts = Toasts::default();
        let stream_handle: Option<OutputStream> =
            match rodio::OutputStreamBuilder::open_default_stream() {
                Ok(stream_handle) => Some(stream_handle),
                Err(error) => {
                    toasts.error(error.to_string());
                    None
                }
            };

        let sink: Option<Sink> = stream_handle
            .as_ref()
            .map(|stream_handle| rodio::Sink::connect_new(&stream_handle.mixer()));

        Self {
            enable_top: true,
            enable_bottom: false,
            enable_left: false,
            enable_right: false,
            is_first_frame: true,
            is_after_refresh: true,
            is_light_mode: false,
            volume: 1.0,
            ui_csv_table_height: 300.0,
            ui_csv_table_min_height: 30.0,
            current_mode: CurrentMode::Idle,
            csv_filepath: CSV_FILEPATH.into(),
            attempt_directory: "Attempts".to_owned(),
            commit_directory: "Commit".to_owned(),
            recorder: Recorder::default(),
            player: sink,
            player_out_stream: stream_handle,
            has_filtered_csv_changed: true,
            filtered_csv_records: Vec::<Rc<CsvRecord>>::default(), // run before csv_records
            csv_records: Vec::<Rc<CsvRecord>>::default(),
            selected_csv_record: Rc::default(),
            csv_filter: "".to_owned(),
            is_csv_filtered_by_uncommitted: true,
            selected_dub_id: String::default(),
            hovered_dub_index: usize::MAX,

            current_attempt_filenames: vec![],
            next_attempt_filename: "".to_owned(),
            next_attempt_path: PathBuf::default(),
            hovered_attempt_filename: "".to_owned(),
            selected_attempt_filename: String::default(),

            committed_dub_id_lookup: FastHashSet::default(),

            is_instant_replay_enabled: false,
            is_filter_box_in_focus: false,
            toasts: toasts,
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
        let mut this = Self {
            state: <DubiousDubsState>::default(),
        };

        initialize_csv_state(&mut this.state, None);
        this
    }
}

impl eframe::App for DubiousDubsApp {
    // The update loop for the GUI
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        // Show Toasts
        self.state.toasts.show(ui);

        if self.state.is_after_refresh {
            self.state.is_after_refresh = false
        };

        if self.state.is_first_frame {
            set_display_visuals(ui, &mut self.state);
            //ui.set_zoom_factor(1.20);
        }

        if (self.state.has_filtered_csv_changed || self.state.is_first_frame) {
            self.state.is_after_refresh = true;
            //get_current_attempt_files(&mut self.state);
            get_current_commit_files(&mut self.state);

            self.state.filtered_csv_records = self
                .state
                .csv_records
                .iter()
                .filter(|r| {
                    !(self.state.is_csv_filtered_by_uncommitted
                        && self.state.committed_dub_id_lookup.contains(&r.dub_id))
                })
                .filter(|r| {
                    self.state.csv_filter.is_empty() || r.dub_id.contains(&self.state.csv_filter)
                })
                .cloned()
                .collect();
            self.state.has_filtered_csv_changed = false;
        }

        if self.state.is_first_frame && self.state.filtered_csv_records.len() > 0 {
            self.state.selected_csv_record = self.state.filtered_csv_records[0].clone();
            self.state.selected_dub_id = self.state.selected_csv_record.dub_id.to_string();
        }

        ui_lib::generated_ui_panels(ui, &mut self.state);

        if self.state.is_after_refresh {
            get_current_attempt_files(&mut self.state);
        }

        if self.state.is_first_frame {
            self.state.is_first_frame = false;
        }
        if self.state.has_filtered_csv_changed {}
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size((1974.0, 1000.0))
            .with_min_inner_size((1974.0, 1000.0)),
        centered: true,
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Dubious Dubber",
        native_options,
        Box::new(|_cc| Ok(Box::new(DubiousDubsApp::default()))),
    )
}
