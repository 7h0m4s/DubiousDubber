use audio_recorder_rs::Recorder;
use eframe::{egui, Frame};
use egui::Ui;

mod ui_lib;
mod csv_lib;
mod audio_recorder_worker;

#[derive(Debug,PartialEq)]
enum CurrentMode {
    Idle,
    Recording,
    Playback,
}

#[derive(Debug)]
struct DubiousDubsState {
    enable_top: bool, enable_bottom: bool, enable_left: bool, enable_right: bool,
    sel_13: usize,
    current_mode: CurrentMode,
    selected_filename: String,
    recorder: audio_recorder_rs::Recorder,
}

impl Default for DubiousDubsState {
    fn default() -> Self {
        Self {
            enable_top: true, enable_bottom: false, enable_left: false, enable_right: false,
            sel_13: 0,
            current_mode: CurrentMode::Idle,
            selected_filename: "123.wav".to_owned(),
            recorder: Recorder::default(),
        }
    }
}

// =============================================================================
// Application entry point
// =============================================================================

#[derive(Debug)]
pub struct DubiousDubsApp {
    state: DubiousDubsState,
}

impl Default for DubiousDubsApp {
    fn default() -> Self {
        Self { state: <DubiousDubsState>::default() }



    }
}

impl eframe::App for DubiousDubsApp {

    // The update loop for the GUI
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        ui_lib::generated_ui_panels(ui, &mut self.state);
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 800.0)),
        centered: true,
        ..eframe::NativeOptions::default() };

    eframe::run_native(
        "Dubious Dub",
        native_options,
        Box::new(|_cc| Ok(Box::new(DubiousDubsApp::default()))),
    )
}