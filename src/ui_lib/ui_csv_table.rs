use crate::audio_lib::{
    start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::CsvRecord;
use crate::file_lib::get_current_attempt_files;
use crate::{CurrentMode, DubiousDubsState, csv_lib, ui_lib};
use anyhow::Error;
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Stroke, text_edit, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;
use std::rc::Rc;

pub fn draw_csv_table(ui: &mut egui::Ui, state: &mut DubiousDubsState) -> Result<(), Error> {
    state.filtered_csv_records = state
        .csv_records
        .iter()
        .filter(|r| {
            !(state.is_csv_filtered_by_uncommitted
                && state.committed_dub_id_lookup.contains(&r.dub_id))
        })
        .filter(|r| state.csv_filter.is_empty() || r.dub_id.contains(&state.csv_filter))
        .cloned()
        .collect();

    if state.is_first_frame && state.filtered_csv_records.len() > 0{
        state.selected_csv_record = state.filtered_csv_records[0].clone();
        state.selected_dub_id = state.selected_csv_record.dub_id.to_string();
    }

    ui.horizontal(|ui| {
        ui.label("Filter: ");
        let csv_filter_box = ui.text_edit_singleline(&mut state.csv_filter);

        ui.add_space(20.0);
        ui.checkbox(
            &mut state.is_csv_filtered_by_uncommitted,
            "Filter By Uncommitted",
        );
        ui.add_space(20.0);
        ui.label(format!(
            "Displaying {} of {}",
            state.filtered_csv_records.len(),
            state.csv_records.len()
        ))
    });

    egui::Frame::canvas(ui.style_mut())
        .inner_margin(4)
        .corner_radius(3)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .sense(egui::Sense::click())
                .id_salt("csvtable")
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .vscroll(true)
                .max_scroll_height(200.0)
                .column(
                    Column::initial(200.0)
                        .at_least(40.0)
                        .resizable(true)
                        .clip(true),
                )
                .column(
                    Column::initial(200.0)
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
                .column(
                    Column::initial(80.0)
                        .at_least(40.0)
                        .resizable(true)
                        .clip(true),
                )
                .header(20.0, |mut header| {
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
                    header.col(|ui| {
                        ui.heading("Requirements");
                    });
                    header.col(|ui| {
                        ui.heading("Path");
                    });
                    header.col(|ui| {
                        ui.heading("Level");
                    });
                    header.col(|ui| {
                        ui.heading("Is Dream");
                    });
                    header.col(|ui| {
                        ui.heading("Is Flirt");
                    });
                    header.col(|ui| {
                        ui.heading("Is Groundhog");
                    });
                    header.col(|ui| {
                        ui.heading("Has Battle");
                    });
                })
                .body(|mut body| {
                    //generate_csv_rows(&mut body, state).expect("TODO: panic message");

                    body.rows(30.0, state.filtered_csv_records.len(), |mut row| {
                        let rec = &state.filtered_csv_records[row.index()];
                        let mut is_selected = rec.dub_id.eq(&state.selected_dub_id);
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
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                false,
                                &rec.story_id,
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
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.line_to_dub,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.requirements,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.path,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.level,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.is_dream,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.is_flirt,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.is_groundhog,
                            );
                        });
                        row.col(|ui| {
                            ui_lib::format_table_cell(
                                ui,
                                is_selected,
                                is_hovered,
                                is_committed,
                                true,
                                &rec.has_battle,
                            );
                        });

                        let is_clicked: bool = row.response().clicked();
                        is_hovered = row.response().hovered();

                        if is_clicked {
                            state.selected_dub_id = rec.dub_id.to_string();
                            state.selected_csv_record = rec.clone();
                            state.selected_attempt_filename.clear();
                        }

                        if is_hovered {
                            state.hovered_dub_index = row.index();
                        }
                    });
                });
        });
    Ok(())
}
