use crate::audio_lib::{start_audio_playback, stop_audio_playback};
use crate::{CurrentMode, DubiousDubsState, ui_lib};
use eframe::egui::Align;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self};
use egui_extras::{Column, TableBuilder};

pub fn draw_attempt_file_table(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    let mut is_scroll_to_row: bool = state.is_after_refresh;
    let mut scroll_index: usize = 0;

    // Select last attempt filename if none selected
    if state.selected_attempt_filename.is_empty() && state.current_attempt_filenames.len() > 0 {
        state.selected_attempt_filename =
            state.current_attempt_filenames[state.current_attempt_filenames.len() - 1].to_string();
    }

    if !state
        .current_attempt_filenames
        .contains(&state.selected_attempt_filename)
    {
        if (!state.current_attempt_filenames.is_empty()) {
            state.selected_attempt_filename = state.current_attempt_filenames
                [state.current_attempt_filenames.len() - 1]
                .to_string();
            is_scroll_to_row = true
        } else {
            state.selected_attempt_filename.clear();
        }
    }

    if state.current_mode == CurrentMode::Recording && !state.current_attempt_filenames.is_empty() {
        state.selected_attempt_filename =
            state.current_attempt_filenames[state.current_attempt_filenames.len() - 1].to_string();
    }

    if (is_scroll_to_row) {
        let index_of_selected_attempt = state
            .current_attempt_filenames
            .iter()
            .position(|filename| filename.eq(&state.selected_attempt_filename));
        if let Some(index) = index_of_selected_attempt {
            scroll_index = index;
        }
    }

    // Keyboard Scroll Controls
    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
        let index_of_selected_attempt = state
            .current_attempt_filenames
            .iter()
            .position(|filename| filename.eq(&state.selected_attempt_filename));
        if let Some(index) = index_of_selected_attempt
            && (index > 0)
        {
            scroll_index = index - 1;
            is_scroll_to_row = true;
            state.selected_attempt_filename = state.current_attempt_filenames[scroll_index].clone();
        }
    }
    if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
        let index_of_selected_attempt = state
            .current_attempt_filenames
            .iter()
            .position(|filename| filename.eq(&state.selected_attempt_filename));
        if let Some(index) = index_of_selected_attempt
            && (index < state.current_attempt_filenames.len() - 1)
        {
            scroll_index = index + 1;
            is_scroll_to_row = true;
            state.selected_attempt_filename = state.current_attempt_filenames[scroll_index].clone();
        }
    }

    egui::Frame::canvas(ui.style_mut())
        .inner_margin(egui::Margin::symmetric(4, 0))
        .corner_radius(5)
        .show(ui, |ui| {
            let table_height = ui.available_height() * 0.9;

            let mut table = TableBuilder::new(ui)
                .striped(true)
                .sense(egui::Sense::click())
                .id_salt("attempt_table")
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .vscroll(true)
                .max_scroll_height(table_height);

            if is_scroll_to_row {
                table = table.scroll_to_row(scroll_index, Some(Align::Center));
            }

            table
                .column(
                    Column::initial(100.0)
                        .at_least(40.0)
                        .resizable(false)
                        .clip(true),
                )
                .column(
                    Column::initial(250.0)
                        .at_least(40.0)
                        .resizable(false)
                        .clip(true),
                )
                .header(25.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Attempt ID");
                    });
                    header.col(|ui| {
                        ui.heading("Filename");
                    });
                })
                .body(|mut body| {
                    let mut should_delay_repaint = false;

                    body.rows(30.0, state.current_attempt_filenames.len(), |mut row| {
                        let mut rec = &state.current_attempt_filenames[row.index()].to_string();

                        let is_selected = rec.eq(&state.selected_attempt_filename);
                        row.set_selected(is_selected);
                        row.set_overline(true);
                        let row_id = row.index();
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                false,
                                false,
                                false,
                                true,
                                &row_id.to_string(),
                                state.is_light_mode,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                false,
                                false,
                                false,
                                false,
                                &rec.to_string(),
                                state.is_light_mode,
                            );
                        });

                        let is_clicked: bool = row.response().clicked();
                        let is_double_clicked: bool = row.response().double_clicked();
                        let is_hovered = row.response().hovered();

                        if is_clicked {
                            state.selected_attempt_filename = rec.to_string();
                        }

                        if is_double_clicked {
                            state.selected_attempt_filename = rec.to_string();
                            if state.current_mode == CurrentMode::Playback {
                                state.current_mode = CurrentMode::Idle;
                                stop_audio_playback(state).unwrap_or_else(|err| {
                                    state.toasts.error(format!("{}", err));
                                });
                            } else {
                                state.current_mode = CurrentMode::Playback;
                                start_audio_playback(state, 2, true);
                                should_delay_repaint = true;
                            }
                        }

                        if is_hovered {
                            state.hovered_attempt_filename = rec.to_string()
                        }
                    });
                });
        });
    ui.add_space(30.0);
}
