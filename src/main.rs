use eframe::{egui, Frame};
use egui::Ui;
mod ui_lib;
mod csv_lib;

struct DubiousDubsState {
    enable_top: bool, enable_bottom: bool, enable_left: bool, enable_right: bool,
    sel_13: usize,
}

impl Default for DubiousDubsState {
    fn default() -> Self {
        Self {
            enable_top: true, enable_bottom: false, enable_left: false, enable_right: false,
            sel_13: 0,
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
        Self { state: Default::default() }
    }
}

impl eframe::App for DubiousDubsApp {

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        ui_lib::generated_ui(ui, &mut self.state);
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