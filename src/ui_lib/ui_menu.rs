use crate::DubiousDubsState;
use crate::csv_lib::initialize_csv_state;
use eframe::egui;
use rfd::FileDialog;

enum MenuButtons {
    LoadCsv,
    OpenAttemptDir,
    OpenCommitDir,
    Exit,
}

impl MenuButtons {
    fn as_str(&self) -> &'static str {
        match self {

            MenuButtons::LoadCsv => "Load CSV",
            MenuButtons::OpenAttemptDir => "Open Attempt Folder",
            MenuButtons::OpenCommitDir => "Open Commit Folder",
            MenuButtons::Exit => "Exit",
        }
    }
}
pub fn draw_menu(ui: &mut egui::Ui, state: &mut DubiousDubsState) {

    let file_menu_items = [MenuButtons::LoadCsv.as_str().to_owned(),
        MenuButtons::OpenAttemptDir.as_str().to_owned(),
        MenuButtons::OpenCommitDir.as_str().to_owned(),
        MenuButtons::Exit.as_str().to_owned()];
    ui.push_id(("menu_buttons", 13), |ui| {
        ui.menu_button("File", |ui| {
            for (i, it) in file_menu_items.iter().enumerate() {
                if ui.button(it).clicked() {
                    ui.close_kind(egui::UiKind::Menu);

                    if it.eq(MenuButtons::LoadCsv.as_str()) {
                        if let Some(path) = FileDialog::new()
                            .add_filter("csv", &["csv"])
                            .set_directory("/")
                            .pick_file(){
                            initialize_csv_state(state,Some(path));
                        } else {
                            state.toasts.error("No such file or directory");
                        }
                    }
                    else if it.eq(MenuButtons::OpenAttemptDir.as_str()) {
                        opener::open(&state.attempt_directory)
                            .unwrap_or_else(|err| {state.toasts.error(format!("{}", err));});
                    }
                    else if it.eq(MenuButtons::OpenCommitDir.as_str()) {
                        opener::open(&state.commit_directory)
                            .unwrap_or_else(|err| {state.toasts.error(format!("{}", err));});
                    }
                    else if it.eq(MenuButtons::Exit.as_str()) {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close)
                    }
                }
            }
        });
    });

}