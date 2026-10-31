use crate::audio_lib::{
    start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::CsvRecord;
use crate::file_lib::{get_current_attempt_files};
use crate::{CurrentMode, DubiousDubsState, csv_lib, ui_lib};
use anyhow::Error;
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Stroke, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;



pub fn draw_current_details(ui: &mut egui::Ui, state: &mut DubiousDubsState) {
    ui.vertical(|ui| {
        ui.horizontal_centered(|ui| {
            ui.vertical(|ui| {
                ui_lib::draw_gridcell_text(ui, "Dub ID: ", &state.selected_csv_record.dub_id);
                ui_lib::draw_gridcell_text(ui, "Story ID: ", &state.selected_csv_record.story_id);
                ui_lib::draw_gridcell_text(ui, "Choice ID: ", &state.selected_csv_record.choice_id);
                ui_lib::draw_gridcell_text(ui, "Hashcode: ", &state.selected_csv_record.hashcode);
                ui_lib::draw_gridcell_text(ui, "Character: ", &state.selected_csv_record.character);
                ui_lib::draw_gridcell_text(ui, "Characters: ", &state.selected_csv_record.characters);
                ui_lib::draw_gridcell_text(ui, "Button Text: ", &state.selected_csv_record.button_text);
            });

            ui.vertical(|ui| {
                ui_lib::draw_gridcell_text(ui, "Level: ", &state.selected_csv_record.level);
                ui_lib::draw_gridcell_checkbox(
                    ui,
                    "Is Dream",
                    state.selected_csv_record.is_dream.eq("True"),
                );
                ui_lib::draw_gridcell_checkbox(
                    ui,
                    "Is Flirt",
                    state.selected_csv_record.is_flirt.eq("True"),
                );
                ui_lib::draw_gridcell_checkbox(
                    ui,
                    "Is Groundhog",
                    state.selected_csv_record.is_groundhog.eq("True"),
                );
                ui_lib::draw_gridcell_checkbox(
                    ui,
                    "Has Battle",
                    state.selected_csv_record.has_battle.eq("True"),
                );
            });
            ui.vertical(|ui| {
                ui_lib::draw_gridcell_requirements_text(
                    ui,
                    "Requirements: ",
                    &state.selected_csv_record.requirements,
                );
                ui_lib::draw_gridcell_path_text(ui, "Path: ", &state.selected_csv_record.path);
                ui_lib::draw_gridcell_sets_text(ui, "Sets: ", &state.selected_csv_record.sets);
            });
        });
        ui_lib::draw_gridcell_multiline_text(ui, "Line To Dub: ", &state.selected_csv_record.line_to_dub);
    });
}