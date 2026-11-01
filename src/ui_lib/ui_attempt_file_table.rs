use crate::audio_lib::{
    self, start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::CsvRecord;
use crate::file_lib::get_current_attempt_files;
use crate::{CurrentMode, DubiousDubsState, csv_lib, ui_lib};
use anyhow::Error;
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Stroke, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;

pub fn draw_attempt_file_table(
    ui: &mut egui::Ui,
    state: &mut DubiousDubsState,
) -> Result<(), Error> {
    //ui_lib::draw_gridcell_text(ui, "Next Filename:", &state.next_attempt_filename, 100.0, 100.0);
    ui.add_space(15.0);

    // Select last attempt filename if none selected
    if &state.selected_attempt_filename == "" && state.current_attempt_filenames.len() > 0 {
        state.selected_attempt_filename =
            state.current_attempt_filenames[state.current_attempt_filenames.len() - 1].to_string();
    }

    if !state.current_attempt_filenames.contains(&state.selected_attempt_filename)  {
        if(state.current_attempt_filenames.len() > 0){
        state.selected_attempt_filename =
            state.current_attempt_filenames[state.current_attempt_filenames.len() - 1].to_string();
        }
        else {
            state.selected_attempt_filename.clear();
        }
    }

    if state.current_mode == CurrentMode::Recording && state.current_attempt_filenames.len() > 0 {
        state.selected_attempt_filename = state.current_attempt_filenames[state.current_attempt_filenames.len() - 1].to_string();
    }

    TableBuilder::new(ui)
        .striped(true)
        .sense(egui::Sense::click())
        .id_salt("attempttable")
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .vscroll(true)
        .max_scroll_height(200.0)
        .min_scrolled_height(200.0)
        .column(
            Column::initial(100.0)
                .at_least(40.0)
                .resizable(true)
                .clip(true),
        )
        .column(
            Column::initial(250.0)
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
            let mut should_delay_repaint = false;

            body.rows(30.0, state.current_attempt_filenames.len(), |mut row| {
                let mut rec = &state.current_attempt_filenames[row.index()].to_string();

                let is_selected = rec.eq(&state.selected_attempt_filename);
                row.set_selected(is_selected);
                row.set_overline(true);
                let row_id = row.index();
                row.col(|ui| {
                    ui_lib::format_table_cell(ui, false, false, false, true, &row_id.to_string());
                });
                row.col(|ui| {
                    ui_lib::format_table_cell(ui, false, false, false,false, &rec.to_string());
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
                        audio_lib::stop_audio_playback(state).expect("Error when stopping playback");
                    } else {
                        state.current_mode = CurrentMode::Playback;
                        audio_lib::start_audio_playback(state, 2);
                        should_delay_repaint = true;
                        
                    }
                }

                if is_hovered {
                    state.hovered_attempt_filename = rec.to_string()
                }
            });
            
        });

    Ok(())
}
