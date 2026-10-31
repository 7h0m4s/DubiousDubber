use crate::audio_lib::{
    start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::CsvRecord;
use crate::file_lib::{get_current_attempt_files, get_next_attempt_filename_from_file_list};
use crate::{CurrentMode, DubiousDubsState, csv_lib, ui_lib};
use anyhow::Error;
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Stroke, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;


pub fn draw_attempt_file_table(ui: &mut egui::Ui, state: &mut DubiousDubsState) -> Result<(), Error> {
    ui.vertical_centered(|ui| {
        let current_files = get_current_attempt_files(state).unwrap();
        let next_filename =
            get_next_attempt_filename_from_file_list(&current_files, state).unwrap();
        ui_lib::draw_gridcell_text(ui, "Next Filename:", &next_filename);
        ui.add_space(15.0);

        TableBuilder::new(ui)
            .striped(true)
            .sense(egui::Sense::click())
            .id_salt("attempttable")
            .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
            .vscroll(true)
            .max_scroll_height(200.0)
            .min_scrolled_height(200.0)
            .column(
                Column::initial(80.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .column(
                Column::initial(80.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Attempt ID");
                });
                header.col(|ui| {
                    ui.heading("Filename");
                });
            })
            .body(|mut body| {
                //generate_csv_rows(&mut body, state).expect("TODO: panic message");

                body.rows(30.0, current_files.len(), |mut row| {
                    let rec = &current_files[row.index()];
                    let is_selected = rec.eq(&state.selected_attempt_filename);
                    row.set_selected(is_selected);
                    row.set_overline(true);
                    let row_id = row.index();
                    row.col(|ui| {
                        ui_lib::format_table_cell(ui, false, false, &row_id.to_string());
                    });
                    row.col(|ui| {
                        ui_lib::format_table_cell(ui, false, false, &rec.to_string());
                    });

                    let is_clicked: bool = row.response().clicked();
                    let is_hovered = row.response().hovered();

                    if is_clicked {
                        state.selected_attempt_filename = rec.to_string();
                    }

                    if is_hovered {
                        state.hovered_attempt_filename = rec.to_string()
                    }
                });
            });
    });
    Ok(())
}