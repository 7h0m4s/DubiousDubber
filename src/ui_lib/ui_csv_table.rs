use crate::audio_lib::{
    start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::CsvRecord;
use crate::file_lib::get_current_attempt_files;
use crate::{CurrentMode, DubiousDubsState, csv_lib, ui_lib};
use anyhow::Error;
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Sense, Stroke, text_edit, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;
use std::ops::Div;
use std::rc::Rc;

pub fn draw_csv_table(ui: &mut egui::Ui, state: &mut DubiousDubsState){

    let mut is_scroll_to_row:bool = false;
    let mut scroll_index:usize = 0;
    // Keyboard Scroll Controls
    if ui.input(|i| i.key_pressed(egui::Key::PageUp) || i.key_pressed(egui::Key::ArrowLeft)) {
        let index_of_selected_record = state.filtered_csv_records.iter().position(|a| a.dub_id == state.selected_dub_id && a.choice_id == state.selected_csv_record.choice_id);
        if let Some(index) = index_of_selected_record && (index > 0) {
            scroll_index = index - 1;
            is_scroll_to_row = true;
            state.selected_csv_record = state.filtered_csv_records[scroll_index].clone();
            state.selected_dub_id = state.selected_csv_record.dub_id.to_string();
            state.selected_attempt_filename.clear();
            state.has_filtered_csv_changed = true;

        }
    }
    if ui.input(|i| i.key_pressed(egui::Key::PageDown) || i.key_pressed(egui::Key::ArrowRight)) {
        let index_of_selected_record = state.filtered_csv_records.iter().position(|a| a.dub_id == state.selected_dub_id && a.choice_id == state.selected_csv_record.choice_id);
        if let Some(index) = index_of_selected_record && (index < state.filtered_csv_records.len() - 1) {
            scroll_index = index + 1;
            is_scroll_to_row = true;
            state.selected_csv_record = state.filtered_csv_records[scroll_index].clone();
            state.selected_dub_id = state.selected_csv_record.dub_id.to_string();
            state.selected_attempt_filename.clear();
            state.has_filtered_csv_changed = true;
        }
    }

    egui::Frame::canvas(ui.style_mut())
        .inner_margin(egui::Margin::symmetric(4,0))
        .corner_radius(3)
        .show(ui, |ui| {
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .sense(egui::Sense::click())
                .auto_shrink([false, false])
                .id_salt("csvtable")
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .vscroll(true)
                .min_scrolled_height(state.ui_csv_table_min_height)
                // .max_scroll_height(300.0)
                .max_scroll_height(f32::INFINITY);

                if is_scroll_to_row{
                    table = table.scroll_to_row(scroll_index, Some(Align::Center));
                }

                table.column(
                    Column::initial(200.0)
                        .at_least(40.0)
                        .resizable(true)
                        .auto_size_this_frame(state.is_first_frame)
                        .clip(true),
                )
                .column(
                    Column::initial(200.0)
                        .at_least(40.0)
                        .resizable(true)
                        .auto_size_this_frame(state.is_first_frame)
                        .clip(true),
                )
                .column(
                    Column::initial(80.0)
                        .at_least(40.0)
                        .resizable(true)
                        .auto_size_this_frame(state.is_first_frame)
                        .clip(true),
                )
                .column(
                    Column::initial(80.0)
                        .at_least(40.0)
                        .resizable(true)
                        .auto_size_this_frame(state.is_first_frame)
                        .clip(true),
                )
                .column(
                    Column::initial(80.0)
                        .at_least(40.0)
                        .resizable(true)
                        .auto_size_this_frame(state.is_first_frame)
                        .clip(true),
                )
                .column(
                    Column::initial(80.0)
                        .at_least(40.0)
                        .resizable(true)
                        .auto_size_this_frame(state.is_first_frame)
                        .clip(true),
                )
                .column(
                    Column::remainder()
                        .at_least(40.0)
                        //.resizable(true)
                        //.auto_size_this_frame(state.is_first_frame)
                        .clip(true),
                )
                .header(25.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Dub ID");
                    });
                    header.col(|ui| {
                        ui.heading("Choice ID");
                    });
                    header.col(|ui| {
                        ui.heading("Character");
                    });
                    header.col(|ui| {
                        ui.heading("Characters");
                    });
                    header.col(|ui| {
                        ui.heading("Button Text");
                    });
                    header.col(|ui| {
                        ui.heading("Sets");
                    });
                    header.col(|ui| {
                        ui.heading("Line To Dub");
                    });
                })
                .body(|mut body| {
                    //generate_csv_rows(&mut body, state).expect("TODO: panic message");

                    body.rows(30.0, state.filtered_csv_records.len(), |mut row| {
                        let rec = &state.filtered_csv_records[row.index()];
                        let is_selected = rec.dub_id.eq(&state.selected_dub_id) && rec.choice_id.eq(&state.selected_csv_record.choice_id);
                        let mut is_hovered = row.index().eq(&state.hovered_dub_index);
                        let is_committed = state.committed_dub_id_lookup.contains(&rec.dub_id);

                        row.set_selected(is_selected);
                        row.set_overline(true);

                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                false,
                                &rec.dub_id,
                                state.is_light_mode,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                false,
                                &rec.choice_id,
                                state.is_light_mode,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                false,
                                &rec.character,
                                state.is_light_mode,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.characters,
                                state.is_light_mode,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.button_text,
                                state.is_light_mode,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.sets,
                                state.is_light_mode,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                false,
                                &rec.line_to_dub.replace("\n", ""),
                                state.is_light_mode,
                            );
                        });

                        let is_clicked: bool = row.response().clicked();
                        let is_double_clicked: bool = row.response().double_clicked();
                        is_hovered = row.response().hovered();

                        if is_clicked {
                            state.selected_dub_id = rec.dub_id.to_string();
                            state.selected_csv_record = rec.clone();
                            state.selected_attempt_filename.clear();
                            get_current_attempt_files(state);
                        }

                        if is_double_clicked{
                            start_audio_playback(state, 1, false);
                        }

                        if is_hovered {
                            state.hovered_dub_index = row.index();
                        }
                    });
                });
        });

    let (drag_handle_rect, drag_handle) = ui.allocate_exact_size(vec2(ui.available_width(), 5.0), Sense::drag());

    if drag_handle.hovered() || drag_handle.dragged(){
        ui.set_cursor_icon(egui::CursorIcon::ResizeVertical);
    }
    if drag_handle.dragged(){
        state.ui_csv_table_height += drag_handle.drag_delta().y;
    }
    let available_height = ui.clip_rect().height() * 0.5;
    //let available_height:f32 = 0.8;
    state.ui_csv_table_height = state.ui_csv_table_height.clamp(state.ui_csv_table_min_height, (available_height).max(0.1));

    let stroke = if drag_handle.hovered() || drag_handle.dragged() {
        ui.visuals().widgets.active.fg_stroke
    } else {
        ui.visuals().widgets.noninteractive.bg_stroke
    };
    ui.painter().hline(drag_handle_rect.x_range(), drag_handle_rect.center().y, stroke);


}
