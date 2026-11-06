use crate::audio_lib::{
    start_audio_playback, start_audio_recording, stop_audio_playback, stop_audio_recording,
};
use crate::csv_lib::CsvRecord;
use crate::file_lib::{self, get_current_attempt_files};
use crate::{CurrentMode, DubiousDubsState, csv_lib};
use anyhow::{Error, Ok};
use eframe::egui::Role::CheckBox;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{self, IdSalt, Layout, Stroke, vec2};
use eframe::egui::{Align, Color32, FontSelection, RichText, Style, Vec2, text};
use egui_extras::{Column, Size, StripBuilder, TableBody, TableBuilder, TableRow};
use std::fmt::Alignment;
use std::ops::Index;

pub fn draw_bottom_buttons(
    ui: &mut egui::Ui,
    state: &mut DubiousDubsState,
) -> Result<(), anyhow::Error> {
    ui.horizontal(|ui| {
        let record_button_text = if state.current_mode == CurrentMode::Recording {
            "(Z) Stop Recording"
        } else {
            "(Z) Start Recording"
        };
        let record_button_styled_text = RichText::new(record_button_text)
            .size(24.0)
            .color(if state.is_light_mode {
                Color32::BLACK
            } else {
                Color32::WHITE
            });
        let record_button = ui.add_enabled(
            state.current_mode != CurrentMode::Playback,
            egui::Button::new(record_button_styled_text)
                .fill(if state.current_mode == CurrentMode::Recording {
                    if state.is_light_mode {
                        Color32::LIGHT_RED
                    } else {
                        Color32::DARK_RED
                    }
                } else {
                    if state.is_light_mode {
                        Color32::LIGHT_GREEN
                    } else {
                        Color32::DARK_GREEN
                    }
                })
                .min_size(Vec2::new(110.0, 50.0)),
        );

        if record_button.clicked() || (ui.input(|i| i.key_pressed(egui::Key::Z) && !state.is_filter_box_in_focus)) {
            state.has_filtered_csv_changed = true;
            if state.current_mode == CurrentMode::Recording {
                stop_audio_recording(state).expect("Error when stopping recording");
                if state.is_instant_replay_enabled {
                    start_audio_playback(state, 20, true);
                    ui.ctx()
                        .request_repaint_after(std::time::Duration::from_millis(50));
                }
            } else {
                start_audio_recording(state).expect("Error when starting recording");
            }
        }

        // Keep repainting if we are waiting for playback to finish
        if (state.current_mode == CurrentMode::Playback) {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(50));
        }

        // Check playback status
        if state
            .player
            .as_ref()
            .is_some_and(|player| player.empty() || player.is_paused())
            && state.current_mode == CurrentMode::Playback
        {
            state.current_mode = CurrentMode::Idle;
        }
        let playback_button_text = if state.current_mode == CurrentMode::Playback {
            "(X) Stop playback"
        } else {
            "(X) Start playback"
        };
        let playback_button_styled_text = RichText::new(playback_button_text)
            .size(24.0)
            .color(if state.is_light_mode {
                Color32::BLACK
            } else {
                Color32::WHITE
            });
        let playback_button = ui.add_enabled(
            state.current_mode != CurrentMode::Recording,
            egui::Button::new(playback_button_styled_text)
                .fill(if state.current_mode == CurrentMode::Playback {
                    if state.is_light_mode {
                        Color32::LIGHT_RED
                    } else {
                        Color32::DARK_RED
                    }
                } else {
                    if state.is_light_mode {
                        Color32::LIGHT_GREEN
                    } else {
                        Color32::DARK_GREEN
                    }
                })
                .min_size(Vec2::new(110.0, 50.0)),
        );

        if playback_button.clicked() || (ui.input(|i| i.key_pressed(egui::Key::X) && !state.is_filter_box_in_focus)) {
            state.has_filtered_csv_changed = true;
            if state.current_mode == CurrentMode::Playback {
                stop_audio_playback(state).expect("Error when stopping Playback");
            } else {
                start_audio_playback(state, 2, true);
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_millis(50));
            }
        }

        ui.add_space(10.0);

        let commit_selected_button_styled_text = RichText::new("(C) Commit & Next")
            .size(24.0)
            .color(if state.is_light_mode {
                Color32::BLACK
            } else {
                Color32::WHITE
            });

        let commit_selected_button = egui::Button::new(commit_selected_button_styled_text)
            .min_size(Vec2::new(200.0, 50.0))
            .fill(if state.is_light_mode {
                Color32::LIGHT_GRAY
            } else {
                Color32::DARK_GRAY
            });

        if (ui
            .add_enabled(
                state.current_mode == CurrentMode::Idle,
                commit_selected_button,
            )
            .clicked()
            || (ui.input(|i| i.key_pressed(egui::Key::C)) && !state.is_filter_box_in_focus))
            && state.current_mode == CurrentMode::Idle
            && !state.selected_attempt_filename.is_empty()
        {
            state.has_filtered_csv_changed = true;
            file_lib::commit_selected_attempt_file(state).expect("failed to copy file");
            let current_csv_selection_index = state
                .filtered_csv_records
                .iter()
                .position(|a| a.dub_id.eq(&state.selected_dub_id))
                .unwrap_or(0);
            let new_selection_index;
            if state.filtered_csv_records.len() > 1 {
                if current_csv_selection_index >= state.filtered_csv_records.len() - 1 {
                    new_selection_index = current_csv_selection_index - 1;
                } else {
                    new_selection_index = current_csv_selection_index + 1;
                }

                state.selected_csv_record = state.filtered_csv_records[new_selection_index].clone();
                state.selected_dub_id = state.selected_csv_record.dub_id.to_string();
                state.selected_attempt_filename.clear();
            }
        }

        ui.add_space(10.0);
        ui.checkbox(&mut state.is_instant_replay_enabled, "Instant Replay");

        ui.add_space(50.0);
    });

    Ok(())
}
